# Index Bot (version 6) [![codecov](https://codecov.io/gh/KaceCottam/indexbot6/branch/master/graph/badge.svg?token=APPT7FJZK2)](https://codecov.io/gh/KaceCottam/indexbot6) [![Code style: black](https://img.shields.io/badge/code%20style-black-000000.svg)](https://github.com/psf/black) ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/KaceCottam/indexbot6/Workflows)

Remake of Jake Arent's Indexbot in Python 3.10 with my home-made [Tiny Query Database](http://github.com/KaceCottam/tqdb).

A discord bot for creating hidden roles that still ping users.

## Essential Information

Requirements are in the [`requirements.txt`](requirements.txt)

Requires a bot to be made with the following permissions: `bot`, `applications.commands`. 

# TODO figure out permissions needed.

## How to use

Configuration is done via creation of a file titled `.env` in the same folder as the python script. This file is required to configure the bot.

You can register bot commands in your server after adding the bot by typing `$register` in the chat.

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

# database file location (optional)
# can be set to ':memory:' to make a non-persistent bot
BOT_ROLES_DB=roles.db
```
