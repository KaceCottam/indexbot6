from disnake.ext import commands
import disnake
import settings

bot = commands.Bot(command_prefix="$")


@bot.slash_command()
async def ping(inter: disnake.ApplicationCommandInteraction) -> None:
    await inter.response.send_message("Pong!")


if __name__ == "__main__":
    """
    permissions integer: 34628176896
    """
    bot.run(token=settings.BOT_TOKEN)
