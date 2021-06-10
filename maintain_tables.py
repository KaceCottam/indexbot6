import discord
from settings import BOT_ROLES_DB
import api

con, cur = api.makeApi(BOT_ROLES_DB)

def setup(bot):
    @bot.listen("on_connect")
    async def onConnect():
        for guild in bot.guilds:
            api.ensureTableExists(cur, guild.id)
        con.commit() # save db

        print(f"Connected as {bot.user.name}#{bot.user.discriminator} ({bot.user.id})!")
        await bot.change_presence(activity = discord.Game(f"/help"))

    @bot.listen("on_guild_join")
    async def onGuildJoin(guild: discord.Guild):
        print(f"Connected to guild {guild.id}")
        api.ensureTableExists(cur, guild.id)
        con.commit()
