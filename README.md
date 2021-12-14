# Index Bot (version 6) [![codecov](https://codecov.io/gh/KaceCottam/indexbot6/branch/master/graph/badge.svg?token=APPT7FJZK2)](https://codecov.io/gh/KaceCottam/indexbot6) [![Code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/psf/black) 

Remake of Jake Arent's Indexbot in Python 3.10 with TinyDB.

A discord bot for creating hidden roles that still ping users.

## Essential Information

Requirements are in the [`requirements.txt`](requirements.txt)

Requires a bot to be made with the following permissions: `bot`, `applications.commands` and `manage_roles`, `send_messages`.

Make sure to enable "Server Members Intent" under the Privileged Gateway Intents tab for you bot in the discord developer website.

## How to use

Configuration is done via creation of a file titled `.env` in the same folder as the python script. This file is required to configure the bot.

## Testing

Tests can be run using `behave` which is a behavioral driven testing framework. See examples in the `features` folder.

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
