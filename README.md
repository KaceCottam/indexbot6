# Index Bot (version 3)

Remake of Jake Arent's Indexbot in Python with Sqlite 3.

Made by Kace Cottam.

A discord bot for notifying users based on pings.

## Essential Information

Requires `python-dotenv` and `discord.py` to run. Uses a `sqlite3` database back-end.

## How to use

Configuration is done via creation of a `.env` file in the same folder as the python script. This file is required to describe the `BOT_TOKEN`.

> Example:
>
> ```bash
> # <bot_directory>/.env
> # defaults are indicated by filled values
> # syntax: `<identifier>=<value>`. Comments can be made with '#'
> # these values are sensitive to trailing spaces!
>
> # bot token to authenticate (required)
> BOT_TOKEN=?
> 
> # database file location (optional)
> # can be set to ':memory:' to make a non-persistent bot
> BOT_ROLES_DB=roles.db
>
> # command prefix (optional)
> BOT_PREFIX=!
> ```
>