import os
import sys
from dotenv import load_dotenv
from pathlib import Path

load_dotenv()

BOT_APPLICATION_ID = os.environ.get("BOT_APPLICATION_ID")
BOT_TOKEN = os.environ.get("BOT_TOKEN")
BOT_ROLES_DB = Path(os.environ.get("BOT_ROLES_DB") or "./roles.db")
BOT_GUILD_IDS = [ int(i) for i in ids.split() ] if (ids := os.environ.get("BOT_GUILD_IDS")) else None

if BOT_APPLICATION_ID is None:
    print("APPLICATION_ID not loaded! Aborting...", file=sys.stderr)
    sys.exit(1)
if BOT_TOKEN is None:
    print("TOKEN not loaded! Aborting...", file=sys.stderr)
    sys.exit(1)
if BOT_GUILD_IDS is None:
    print("BOT_GUILD_IDS not loaded! Aborting...", file=sys.stderr)
    sys.exit(1)
