import os
import sys
from dotenv import load_dotenv
from pathlib import Path

load_dotenv()

TOKEN = os.environ.get("BOT_TOKEN")
ROLES_DB = Path(os.environ.get("BOT_ROLES_DB") or "./roles.db")
PREFIX = os.environ.get("BOT_PREFIX") or r"!"

if not TOKEN:
    print("Token not loaded! Aborting...", file=sys.stderr)
    sys.exit(1)