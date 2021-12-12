from discord.ext import commands

import settings

bot = commands.Bot(command_prefix="$")


@bot.command()
async def ping(context: commands.Context) -> None:
    await context.send("pong!")


if __name__ == "__main__":
    """
    permissions integer: 34628176896
    """
    bot.login(token=settings.BOT_TOKEN)
