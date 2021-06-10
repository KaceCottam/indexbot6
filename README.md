# Index Bot (version 4)

Remake of Jake Arent's Indexbot in Python with Sqlite 3.

Made by Kace Cottam.

A discord bot for notifying users based on pings.

## Essential Information

Requires `python-dotenv`, `discord_slash` and `discord.py` to run. Uses a `sqlite3` database back-end.

## How to use

Configuration is done via creation of a file titled `.env` in the same folder as the python script. This file is required to configure the bot.

### Example `.env` file

```bash
# <bot_directory>/.env
# defaults are indicated by filled values, required values are indicated by `?`
# syntax: `<identifier>=<value>`. Comments can be made with '#'
# these values are sensitive to trailing spaces!

# bot token to authenticate (required)
BOT_TOKEN=?

# bot application id (required)
BOT_APPLICATION_ID=?

# guild ids that slash commands will apply to, separated by spaces (required)
BOT_GUILD_IDS=?

# database file location (optional)
# can be set to ':memory:' to make a non-persistent bot
BOT_ROLES_DB=roles.db
```
