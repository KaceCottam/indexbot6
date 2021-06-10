import sqlite3
from typing import List, Optional, Union

# given that this only works with message ids, it would be difficult to use this database unsafely.

def makeApi(path, timeout: int = 1000):
    con = sqlite3.connect(path, timeout=timeout)
    cur = con.cursor()
    return (con, cur)

def ensureTableExists(cursor: sqlite3.Cursor, guildid: int) -> None:
    cursor.execute(fr"CREATE TABLE IF NOT EXISTS guild_{guildid} (roleid INT, userid INT, UNIQUE(roleid, userid))")

def addRole(cursor: sqlite3.Cursor, guildid: int, roleid: int, userid: int) -> Optional[str]:
    try:
        cursor.execute(fr"INSERT INTO guild_{guildid} VALUES({roleid}, {userid})")
    except sqlite3.IntegrityError:
        return "That user is already there!"

def listUsers(cursor: sqlite3.Cursor, guildid: int, roleid: int) -> List[int]:
    cursor.execute(fr"SELECT userid FROM guild_{guildid} WHERE roleid={roleid}")
    return [ i[0] for i in cursor.fetchall() ]

def removeUserFromRole(cursor: sqlite3.Cursor, guildid: int, roleid: int, userid: int) -> Union[str, int]:
    try:
        cursor.execute(fr"SELECT COUNT(*) FROM GUILD_{guildid} WHERE roleid={roleid}")
        if (int(cursor.fetchone()[0]) == 0):
            return None, "That role doesn't exist!"
        cursor.execute(fr"DELETE FROM guild_{guildid} WHERE roleid={roleid} AND userid={userid}")
        cursor.execute(fr"SELECT COUNT(*) FROM guild_{guildid} WHERE roleid={roleid}")
        if (int(cursor.fetchone()[0]) == 0):
            return roleid, None
    except sqlite3.ProgrammingError:
        return None, "That query doesn't exist!"

def listRoles(cursor: sqlite3.Cursor, guildid: int, userid: int) -> List[int]:
    cursor.execute(fr"SELECT roleid FROM guild_{guildid} WHERE userid={userid}")
    return [ i[0] for i in cursor.fetchall() ]

def listAllRoles(cursor: sqlite3.Cursor, guildid: int) -> List[int]:
    cursor.execute(fr"SELECT roleid FROM guild_{guildid}")
    return [ i[0] for i in cursor.fetchall() ]

def removeRole(cursor: sqlite3.Cursor, guildid: int, roleid: int):
    cursor.execute(fr"DELETE FROM guild_{guildid} WHERE roleid={roleid}")
