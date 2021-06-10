from settings import BOT_TOKEN, BOT_APPLICATION_ID, BOT_ROLES_DB, BOT_GUILD_IDS
import discord
from discord.ext import commands
from discord_slash import SlashCommand, SlashContext
from discord_slash.utils.manage_commands import create_option, SlashCommandOptionType
import api

intent = discord.Intents.default()
intent.members = True

bot = commands.Bot(command_prefix='prefix', intents=intent)
bot.add_check(commands.guild_only())
slash = SlashCommand(bot, sync_commands=True, application_id=BOT_APPLICATION_ID)

con, cur = api.makeApi(BOT_ROLES_DB)

@slash.slash(
    name="game",
    description="Adds you to the notification list for a game",
    options=[
        create_option(
            name="input",
            description="Which game do you want to be notified for?",
            option_type=3,
            required=True)
    ],
    guild_ids=BOT_GUILD_IDS)
async def _game(ctx: SlashContext, input: str):
    input = input.lower()
    roleDict = { r.name: r for r in ctx.guild.roles }
    embed = discord.Embed(title=f'Adding to game "{input}"', color=discord.Color.dark_blue())
    if input not in roleDict:
        newRole = await ctx.guild.create_role(name=input, mentionable=True)
        roleDict[newRole.name] = newRole
        embed.add_field(name=f":white_check_mark: New role created!",value=f"New role {newRole.mention} created!", inline=False)
        embed.color = discord.Color.green()
        print(f"New role \"{newRole.name}\" ({newRole.id}) created in guild {ctx.guild.id}!")
    error = api.addRole(cur, ctx.guild.id, roleDict[input].id, ctx.author.id)
    if error:
        embed.color = discord.Color.red()
        embed.add_field(name=":x: Error!", value=f"Already in {roleDict[input].mention}!", inline=False)
    else:
        embed.add_field(name=":video_game: Successfully added user to the game!", value=f"Added {ctx.author.name} to {roleDict[input].mention}!", inline=False)
        print(f"Added user {ctx.author.id} to role {roleDict[input].id} in guild {ctx.guild.id}!")
    await ctx.send(embed=embed)
    con.commit()

@slash.slash(
    name="join",
    description="Adds you to the notification list for a game",
    options=[
        create_option(
            name="role",
            description="Which game do you want to be notified for?",
            option_type=8,
            required=True)
    ],
    guild_ids=BOT_GUILD_IDS)
async def _join(ctx: SlashContext, role: discord.Role):
    embed = discord.Embed(title=f'Adding to game "{role.name}"', color=discord.Color.dark_blue())
    error = api.addRole(cur, ctx.guild.id, role.id, ctx.author.id)
    if error:
        embed.color = discord.Color.red()
        embed.add_field(name=":x: Error!", value=f"Already in {role.mention}!", inline=False)
    else:
        embed.add_field(name=":video_game: Successfully added user to the game!", value=f"Added {ctx.author.name} to {role.mention}!", inline=False)
        print(f"Added user {ctx.author.id} to role {role.id} in guild {ctx.guild.id}!")
    await ctx.send(embed=embed)
    con.commit()

@slash.slash(
    name="remove",
    description="Removes you from the notification list for a game",
    options=[
        create_option(
            name="role",
            description="Which game do you want to not be notified for?",
            option_type=8,
            required=True)
    ],
    guild_ids=BOT_GUILD_IDS)
async def _remove(ctx: SlashContext, role: discord.Role):
    rid, error = api.removeUserFromRole(cur, ctx.guild.id, role.id, ctx.author.id)
    embed = discord.Embed(title=f"Removing from game {role.name}", color=discord.Color.dark_blue())
    if error is not None:
        embed.add_field(name=":x: Error!", value=f"Not recieving notificatiosn for {role.mention}!", inline=False)
        embed.color = discord.Color.red()
        await ctx.send(embed=embed)
        return
    if len(role.members) == 0:
        print(f"Removing role {role.id} from guild {ctx.guild.id}")
        await role.delete(reason="No more notification subscriptions.")
        embed.add_field(name=':broken_heart: Deleting role', value=f'Deleting role "{role.name}"', inline=False)
        embed.color=discord.Color.orange()
    print(f"Removed user {ctx.author.id} from role {role} in guild {ctx.guild.id}!")
    value = f"Unsubscribed from notifications for {role.mention if len(role.members) != 0 else role.name}."
    embed.add_field(name=":no_bell: Successfully unsubscribed from game!", value=value, inline=False)
    await ctx.send(embed=embed)
    con.commit()

@slash.slash(
    name="mygames",
    description="Displays all the games you are being notified for",
    guild_ids=BOT_GUILD_IDS)
async def _mygames(ctx: SlashContext):
    roleDict = { r.id: r for r in ctx.guild.roles }
    roles = api.listRoles(cur, ctx.guild.id, ctx.author.id)
    embed = discord.Embed(title=f"Your roles", color=discord.Color.dark_blue())
    if len(roles) == 0:
        embed.add_field(name=":x: Error!", value="You have no roles!")
        embed.color = discord.Color.red()
        await ctx.send(embed=embed)
        return
    embed.add_field(
        name=":video_game: Here are your roles",
        value='\n'.join( roleDict[rid].mention for rid in roles ),
        inline=False
    )
    await ctx.send(embed=embed)

@slash.slash(
    name="roles",
    description="Displays all the games in the server",
    guild_ids=BOT_GUILD_IDS)
async def _roles(ctx: SlashContext):
    roleDict = { r.id: r for r in ctx.guild.roles }
    roles = api.listAllRoles(cur, ctx.guild.id)
    embed = discord.Embed(title=f"{ctx.guild.name}'s roles", color=discord.Color.dark_blue())
    if len(roles) == 0:
        embed.add_field(name=":x: Error!", value="This server has no roles!", inline=False)
        embed.color=discord.Color.red()
        await ctx.send(embed=embed)
        return
    try:
        embed.add_field(
            name=":video_game: Roles",
            value='\n'.join( roleDict[rid].mention for rid in roles ),
            inline=False
        )
    except KeyError as c:
        embed.add_field(name=":x: Error!", value="Had trouble getting a role :(", inline=False)
        embed.color=discord.Color.red()
        print(f"Key Error when getting roles: {c!r}")
    await ctx.send(embed=embed)

@slash.slash(
    name="help",
    description="Displays help information",
    guild_ids=BOT_GUILD_IDS)
async def _help(ctx: SlashContext):
    embed = discord.Embed(title="IndexBot v4 Help", description="I will ping everyone subscribed to a game if someone mentions that game!", color=discord.Color.dark_blue())
    embed.add_field(name="/help", value="Displays help information", inline=False)
    embed.add_field(name="/game <input> or /join <role>", value="Adds you to the notification list for a game", inline=False)
    embed.add_field(name="/remove <role>", value="Removes you from the notification list for a game", inline=False)
    embed.add_field(name="/mygames", value="Displays all the games you are being notified for", inline=False)
    embed.add_field(name="/roles", value="Displays all the games in the server", inline=False)
    await ctx.send(embed=embed)

bot.load_extension('maintain_tables')
bot.load_extension('ping_roles')
bot.run(BOT_TOKEN)

#TODO merge
#TODO migrate for admins
