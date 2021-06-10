import discord
from discord.ext.commands import context
from settings import BOT_ROLES_DB
import api

con, cur = api.makeApi(BOT_ROLES_DB)

def setup(bot):
    @bot.listen("on_message")
    async def onMessage(message: discord.Message):
        """pings every user subscribed to a notification list if a mention is present in a message"""
        if message.author.bot is True: return # ignore bot commands for this

        allRoles = api.listAllRoles(cur, message.guild.id)

        gameRoles = [ role for role in message.role_mentions if role.id in allRoles ]
        if len(gameRoles) == 0: return # ignore messages without role mentions

        userRoles = { user.id: user for user in await message.guild.fetch_members().flatten() }

        embed = discord.Embed(description="> " + message.content, color=discord.Color.random())
        embed.set_author(name=message.author.display_name, url=message.jump_url, icon_url=message.author.avatar_url)
        mentions = ''
        for role in gameRoles:
            users = api.listUsers(cur, message.guild.id, role.id)
            content = ' '.join( userRoles[user].mention for user in users )
            mentions += content + ' '
            embed.add_field(name=role.name, value=content)

        await message.channel.send(content=mentions, embed=embed)
