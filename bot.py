from os import pipe
from sys import stderr
from typing import Optional

from discord.ext.commands.errors import CommandInvokeError

from settings import ROLES_DB, TOKEN, PREFIX
import api
from functools import reduce

import discord
from discord.ext import commands

# TODO reformat into class :/

client = commands.Bot(command_prefix=PREFIX)
client.add_check(commands.guild_only())
con, cur = api.makeApi(ROLES_DB)

def prefixed(message):
    return message.content.startswith(PREFIX)

def isBot(message):
    return message.author.bot

def messageifyUser(guild: discord.Guild):
    """
    Creates a mention via text using an id
    Curried.
    """
    async def f(id):
        try:
            return (member := await guild.fetch_member(id)) and member.mention or 'ERROR USER'
        except:
            return f"<@{id}>"
    return f  #f"<@{id}>"

def messageifyRole(guild: discord.Guild):
    """
    Creates a mention via text using an id.
    Curried.
    """
    return lambda id: (role := guild.get_role(id)) and role.mention or 'ERROR ROLE' # f"<@{id}>"

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
    if prefixed(message) or isBot(message): return # ignore bot commands for this

    allRoles = api.listRoles(cur, message.guild.id)
    gameRoles = [ role.id for role in message.role_mentions if role.id in allRoles ]
    async def mentionAllUsers(roleId):
        userRoles = api.listUsers(cur, message.guild.id, roleId)
        string = ''
        for u in userRoles:
            string += await messageifyUser(message.guild)(u) + ' '
        return string.strip()

    if gameRoles:
        string = ''
        for r in gameRoles:
            string += await mentionAllUsers(r) + ' '
        await message.channel.send(string.strip())

@client.command()
async def roles(ctx: commands.Context,
                user: Optional[discord.User]):
    """Displays a list of all roles (either of a user, or of a whole server)"""
    allRoles = api.listRoles(cur, ctx.guild.id, user.id if user else None)
    rolesString = '\n'.join(map(messageifyRole(ctx.guild), allRoles))
    await ctx.reply(rolesString if allRoles else "There are no registered roles on this server.")

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
                print(f"Deleting role {role.name!r} ({role.id}) in guild {ctx.guild.id}.")
                deletedRoles.append(role.name)
                await role.delete(reason="No more notification subscriptions.")
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
            errorRoles.append(role.mention)
        else:
            noErrorRoles.append(role.mention)

    # handle non-existing role
    # we are using a dictionary for easy lookup and detection
    roleDict = {r.name: r.id for r in ctx.guild.roles}
    if gameName:
        gameName = gameName.lower()
        if gameName not in roleDict: # must create role
            newRole = await ctx.guild.create_role(name=gameName, mentionable=True)
            roleDict[newRole.name] = newRole.id
            await ctx.send(f"New role {newRole.mention} created!") # TODO i tried to make this ping the new role, but it was not working
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

@client.command()
async def merge(ctx: commands.Context, roles: commands.Greedy[discord.Role], *, newRoleName: Optional[str]):
    """Merges two roles to the name of the last role given, or the string role name given at the end."""
    if not roles or len(roles) < 2:
        await ctx.send_help(merge)
        return
    lastRole = roles[-1].name
    newUsers = reduce(lambda userList, role: userList + api.listUsers(cur, ctx.guild.id, role.id), roles, list())
    deletedRoles = 0
    for role in roles:
        deletedRoles += 1
        api.removeRole(cur, ctx.guild.id, role.id)
        try:
            await role.delete(reason="Merging roles")
        except CommandInvokeError:
            pass
    distinctUsers = list(set(newUsers))
    newRole = await ctx.guild.create_role(name=newRoleName if newRoleName else lastRole, mentionable=True)
    for user in distinctUsers:
        api.addRole(cur, ctx.guild.id, newRole.id, user)
    await ctx.reply(f"Merged {deletedRoles} roles into 1 role {newRole.mention}!")
    con.commit()

NEED_MIGRATION=True
if NEED_MIGRATION:
    import os

    @client.command(hidden=True)
    async def migrate(ctx):
        roles = { x.name.lower(): x.id for x in ctx.guild.roles }
        rolesAdded = 0
        fileNames = []
        try:
            fileNames = [ f for f in os.listdir('./Roles') if f.lower()[:-4] in roles.keys() ]
        except FileNotFoundError:
            print("Roles folder not found.")
        print(roles.keys())
        print(fileNames)
        for filename in fileNames:
            rolesAdded += 1
            usersAdded = 0
            with open('./Roles/' + filename, 'r') as infile:
                userids = infile.readlines()
                for userid in userids:
                    try:
                        error = api.addRole(cur, ctx.guild.id, roles[filename.lower()[:-4]], int(userid))
                        if error:
                            print(f"Error adding user {userid} to role {roles[filename.lower()[:-4]]}.", file=stderr)
                        else:
                            usersAdded += 1
                    except ValueError:
                        print(f"Error, bad userid {userid}")
            print(f"Added {usersAdded} users to role {roles[filename.lower()[:-4]]}")
        print(f"Added {rolesAdded} roles.")
        con.commit()
        await ctx.reply(f"Migrated {rolesAdded} roles. Please test it now!")

if __name__ == '__main__':
    client.run(TOKEN)
