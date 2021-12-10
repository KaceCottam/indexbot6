import os
import sys
from dotenv import load_dotenv

load_dotenv()

BOT_APPLICATION_ID = os.environ.get("BOT_APPLICATION_ID")
BOT_TOKEN = os.environ.get("BOT_TOKEN")
BOT_ROLES_DB = os.environ.get("BOT_ROLES_DB") or "./roles.db"

if BOT_APPLICATION_ID is None:
    print("APPLICATION_ID not loaded!", file=sys.stderr)

if BOT_TOKEN is None:
    print("TOKEN not loaded!", file=sys.stderr)

if any(x is None for x in (BOT_APPLICATION_ID, BOT_TOKEN)):
    print("Invalid configuration. Aborting...")
    sys.exit(1)
