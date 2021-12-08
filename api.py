from tinydb import TinyDB, Query
from tinydb.storages import MemoryStorage

User = Query()


def guild(id: str) -> object:
    """
    A matcher to check if a query matches a guild
    :param id: the guild id to match
    :return: matches
    """
    return User.guildid == id


def role(id: str) -> object:
    """
    A matcher to check if a query matches a role
    :param id: the role id to match
    :return: matches
    """
    return User.roleid == id


def user(id: str) -> object:
    """
    A matcher to check if a query matches a user
    :param id: the user id to match
    :return: matches
    """
    return User.userid == id


def users(ds: list[dict]) -> list[str]:
    """
    A convenience function for grabbing all the users from a query result
    :param ds: the query result
    :return: the list of users
    """
    return [d['userid'] for d in ds]


def roles(ds: list[dict]) -> list[str]:
    """
    A convenience function for grabbing all the roles from a query result
    :param ds: the query result
    :return: the list of roles
    """
    return [d['roleid'] for d in ds]


def initDB(name: str) -> TinyDB:
    """
    An api function for initializing the database
    :param name: Name of file to initialize from. Use memory storage if this is equal to ':memory:'
    :return: database
    """
    return TinyDB(storage=MemoryStorage) if name == ':memory:' else TinyDB(name)


def addUserToRole(db: TinyDB, guildid: str, roleid: str, userid: str) -> None:
    """
    Adds a user in a guild to the notification list for a role
    :param db: database
    :param guildid: guild id
    :param roleid: role id
    :param userid: user id
    """
    db.insert(dict(guildid=guildid, roleid=roleid, userid=userid))


def showUsers(db: TinyDB, guildid: str, roleid: str) -> list[str]:
    """
    Displays the list of users in a role's notification list
    :param db: database
    :param guildid: guild id
    :param roleid: role id
    :return: list of users who have that role in guild
    """
    return users(db.search(guild(guildid) & role(roleid)))


def showRolesOfUser(db: TinyDB, guildid: str, userid: str) -> list[str]:
    """
    Displays the list of roles that a user is being notified for
    :param db:
    :param guildid:
    :param userid:
    :return: list of roles that user has in guild
    """
    return roles(db.search(guild(guildid) & user(userid)))


def showRolesOfGuild(db: TinyDB, guildid: str) -> list[str]:
    """
    Displays the list of roles that a guild has
    :param db:
    :param guildid:
    :return: list of roles in a guild
    """
    return roles(db.search(guild(guildid)))


def removeUserFromRole(db: TinyDB, guildid: str, roleid: str, userid: str) -> None | dict:
    """
    Removes a user from a role in a guild
    :param db:
    :param guildid:
    :param roleid:
    :param userid:
    :return: the user who was removed if they exist, else None
    """
    query = guild(guildid) & role(roleid) & user(userid)
    if u := db.get(query):
        db.remove(query)
        return u
    return None


def removeRole(db: TinyDB, guildid: str, roleid: str) -> None | list[str]:
    """
    Remove a role from the database
    :param db: database
    :param guildid: guild id
    :param roleid: role id
    :return: a list of the users that were removed if the role exists, else None
    """
    query = guild(guildid) & role(roleid)
    if u := db.search(query):
        db.remove(query)
        return users(u)
    return None


def removeUser(db: TinyDB, guildid: str, userid: str) -> None | list[str]:
    """
    Remove a user from the database
    :param db: database
    :param guildid: guild id
    :param userid: user id
    :return: a list of roles that were removed if the user exists, else None
    """
    query = guild(guildid) & user(userid)
    if u := db.search(query):
        db.remove(query)
        return roles(u)
    return None


def removeGuild(db: TinyDB, guildid: str) -> None | list[dict]:
    """
    Removes a guild from the database
    :param db: database
    :param guildid: guild id
    :return: a list of roles and users that was removed if the guild exists, else None
    """
    query = guild(guildid)
    if u := db.search(query):
        db.remove(query)
        return u
    return None
