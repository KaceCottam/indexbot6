import peewee
from peewee import SQL, Model, SqliteDatabase, TextField

database = SqliteDatabase(None)

def InitDB(path: str) -> None:
    """Initialize the database

    Args:
        path (str): path to the database file
    """
    database.init(path)
    
class Roles(Model):
    guildid = TextField(primary_key=True)
    roleid  = TextField()
    userid  = TextField()
    class Meta:
        database = database
        constraints = [SQL("UNIQUE (roleid, userid)")]

def AddUser(guildid: str, roleid: str, userid: str) -> Roles:
    """Add a user to a guild

    Requires database to be initialized.
    

    Args:
        guildid (str): guild id
        roleid (str):  role id
        userid (str):  user id
    """
    user = Roles.create(guildid=guildid, roleid=roleid, userid=userid)
    user.save()
    return user

def ListUsers(guildid: str, roleid: str) -> list[Roles] | None:
    pass