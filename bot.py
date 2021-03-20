from typing import List, Optional

from discord.ext.commands.converter import GameConverter
from settings import ROLES_DB, TOKEN, PREFIX
import api

import discord
from discord.ext import commands

# TODO reformat into class :/

client = commands.Bot(command_prefix=PREFIX)
con, cur = api.makeApi(ROLES_DB)

def prefixed(message):
    return message.content.startswith(PREFIX)

def messageify(id):
    """Creates a mention via text using an id"""
    return f"<@{id}>"

@client.listen("on_connect")
async def onConnect():
    for guild in client.guilds:
        api.ensureTableExists(cur, guild.id)
    con.commit() # save db

    print(f"Connected as {client.user.name}#{client.user.discriminator} ({client.user.id})!")
    await client.change_presence(activity = discord.Game(f"{PREFIX}help"))

@client.listen("on_message")
async def onMessage(message: discord.Message):
    """pings every user subscribed to a notification list if a mention is present in a message"""
    if prefixed(message): return # ignore bot commands for this

    allRoles = api.listRoles(cur, message.guild.id)
    gameRoles = [ role.id for role in message.role_mentions if role.id in allRoles ]

    def mentionAllUsers(roleId):
        userRoles = api.listUsers(cur, message.guild.id, roleId)
        return ' '.join(map(messageify, userRoles))

    if gameRoles:
        await message.channel.send(' '.join(map(mentionAllUsers, gameRoles)))

@client.check_once
async def notBot(ctx):
    """disable replying to bots"""
    return ctx.author.bot == False

client.add_check(commands.guild_only, call_once=True)

@client.command()
async def roles(ctx: commands.Context,
                user: Optional[discord.User]):
    """Displays a list of all roles (either of a user, or of a whole server)"""
    allRoles = api.listRoles(cur, ctx.guild.id, user.id if user else None)
    rolesString = '\n'.join(map(messageify, allRoles))
    await ctx.reply(rolesString if len(allRoles) > 0 else "There are no registered roles on this server.")

@client.command()
async def unnotify(ctx: commands.Context,
                   roles: commands.Greedy[discord.Role]):
    """Removes the user from a notification list for a game"""
    if not roles:
        await ctx.send_help(unnotify)
        return

    errorRoles   = list()
    noErrorRoles = list()
    deletedRoles = list()

    for role in roles:
        error = api.removeUserFromRole(cur, ctx.guild.id, role.id, ctx.author.id)
        if type(error) == str:
            errorRoles.append(role.name)
        elif type(error) == int:
            if not role.members:
                """check to see if this role has active members before deleting it"""
                await role.delete(reason="No more notification subscriptions.")
                print(f"Deleting role {role.name!r} ({role.id}) in guild {ctx.guild.id}.")
                deletedRoles.append(role.name)
            noErrorRoles.append(role.name)
        else:
            noErrorRoles.append(role.name)

    # i do stuff like this in order to group all of my output together.
    if errorRoles:
        errorString = ', '.join(map(repr, errorRoles))
        await ctx.reply(f"Not recieving notifications for: {errorString}!")
    if noErrorRoles:
        noErrorString = ', '.join(map(repr, noErrorRoles))
        await ctx.reply(f"Unsubscribed from notifications for: {noErrorString}!")
    if deletedRoles:
        deletedString = ', '.join(map(repr, deletedRoles))
        await ctx.send(f"Deleted roles: {deletedString}")

    con.commit() # save changes

@client.command()
async def notify(ctx: commands.Context,
                 roles: commands.Greedy[discord.Role],
                 *, gameName: Optional[str]):
    """Adds the user to a notification list for a game"""
    if not roles and not gameName:
        await ctx.send_help(notify)
        return

    errorRoles   = list()
    noErrorRoles = list()

    # handle existing roles
    for role in roles:
        error = api.addRole(cur, ctx.guild.id, role.id, ctx.author.id)
        if error:
            errorRoles.append(role.name)
        else:
            noErrorRoles.append(role.name)
    
    # handle non-existing role
    # we are using a dictionary for easy lookup and detection
    roleDict = {r.name: r.id for r in ctx.guild.roles}
    if gameName: 
        if gameName not in roleDict: # must create role
            newRole = await ctx.guild.create_role(name=gameName, mentionable=True)
            roleDict[newRole.name] = newRole.id
            await ctx.send(f"New role \"{newRole.name}\" created!") # TODO i tried to make this ping the new role, but it was not working
            print(f"New role \"{newRole.name}\" ({newRole.id}) created in guild {ctx.guild.id}!")

        error = api.addRole(cur, ctx.guild.id, roleDict[gameName], ctx.author.id)
        if error:
            errorRoles.append(gameName)
        else:
            noErrorRoles.append(gameName)
    
    if errorRoles: # only reply if there were errors
        errorString = ', '.join(map(repr, errorRoles)) # concatenate to single comma separated string
        await ctx.reply(f"Already in {errorString}!")
    if noErrorRoles: # only reply if there were successes
        noErrorString = ', '.join(map(repr, noErrorRoles)) # concatenate to single comma separated string
        await ctx.reply(f"Added {ctx.author.name} to: {noErrorString}!")

    con.commit() # save changes

if __name__ == '__main__':
    client.run(TOKEN)