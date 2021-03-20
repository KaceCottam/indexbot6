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
    return f"<@{id}>"

@client.listen("on_connect")
async def onConnect():
    for guild in client.guilds:
        api.ensureTableExists(cur, guild.id)
    con.commit()
    print(f"Connected as {client.user.name}#{client.user.discriminator} ({client.user.id})!")

@client.listen("on_message")
async def onMessage(message: discord.Message):
    if prefixed(message):
        return
    allRoles = api.listRoles(cur, message.guild.id)
    gameRoles = [ role.id for role in message.role_mentions if role.id in allRoles ]
    def mentionAllUsers(roleId):
        userRoles = api.listUsers(cur, message.guild.id, roleId)
        return ' '.join(map(messageify, userRoles))
    if len(gameRoles) > 0:
        await message.channel.send(' '.join(map(mentionAllUsers, gameRoles)))

@client.check
async def notBot(ctx):
    return ctx.author.bot == False

@client.check
async def inGuild(ctx):
    return ctx.guild != None

@client.command()
async def games(ctx: commands.Context,
                user: Optional[discord.User]):
    """Displays a list of all games (either of a user, or of a whole server)"""
    allGames = api.listRoles(cur, ctx.guild.id, user.id if user else None)
    gamesString = '\n'.join(map(messageify, allGames))
    await ctx.reply(gamesString if len(allGames) > 0 else "There are no registered games on this server.")

@client.command()
async def unnotify(ctx: commands.Context,
                   roles: commands.Greedy[discord.Role]):
    """Removes the user from a notification list for a game"""
    if len(roles) == 0:
        await ctx.send_help(unnotify)
        return
    errorRoles = []
    noErrorRoles = []
    deletedRoles = []
    for role in roles:
        error = api.removeUserFromRole(cur, ctx.guild.id, role.id, ctx.author.id)
        if type(error) == str:
            errorRoles.append(role.name)
        elif type(error) == int:
            await role.delete(reason="No more notification subscriptions.")
            noErrorRoles.append(role.name)
            deletedRoles.append(role.name)
        else:
            noErrorRoles.append(role.name)
    if errorRoles:
        errorString = ', '.join(map(repr, errorRoles))
        await ctx.reply(f"Not being notified in: {errorString}!")
    if noErrorRoles:
        noErrorString = ', '.join(map(repr, noErrorRoles))
        await ctx.reply(f"Unsubscribed from notifications for: {noErrorString}!")
    if deletedRoles:
        deletedString = ', '.join(map(repr, deletedRoles))
        await ctx.send(f"Deleted roles: {deletedString}")
    con.commit()

@client.command()
async def notify(ctx: commands.Context,
                 roles: commands.Greedy[discord.Role],
                 *, gameName: Optional[str]):
    """Adds the user to a notification list for a game"""
    if not roles and not gameName:
        await ctx.send_help(notify)
        return

    errorRoles = []
    noErrorRoles = []
    for role in roles:
        error = api.addRole(cur, ctx.guild.id, role.id, ctx.author.id)
        if error:
            errorRoles.append(role.name)
        else:
            noErrorRoles.append(role.name)
    roleDict = {r.name: r.id for r in ctx.guild.roles}
    if gameName:
        if gameName not in roleDict:
            newRole = await ctx.guild.create_role(name=gameName, mentionable=True)
            roleDict[newRole.name] = newRole.id
            await ctx.send(f"New role \"{newRole.name}\" created!")
            print(f"New role \"{newRole.name}\" ({newRole.id}) created in guild {ctx.guild.id}!")

        error = api.addRole(cur, ctx.guild.id, roleDict[gameName], ctx.author.id)
        if error:
            errorRoles.append(gameName)
        else:
            noErrorRoles.append(gameName)
    if errorRoles:
        errorString = ', '.join(map(repr, errorRoles))
        await ctx.reply(f"Already in {errorString}!")
    if noErrorRoles:
        noErrorString = ', '.join(map(repr, noErrorRoles))
        await ctx.reply(f"Added {ctx.author.name} to: {noErrorString}!")
    con.commit()

if __name__ == '__main__':
    client.run(TOKEN)