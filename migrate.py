from settings import ROLES_DB
import api
import sys
import os

def fileFilter(x: str):
    return x.endswith('.txt')

if __name__ == '__main__':
    if len(sys.argv) <= 1:
        print("Error: pass in guildid as input")
        sys.exit(1)

    guildid = int(sys.argv[1] or 0)

    (con, cur) = api.makeApi(ROLES_DB)

    api.ensureTableExists(cur, guildid)
    rolesAdded = 0
    for file in filter(fileFilter, os.listdir("./")):
        rolesAdded += 1
        roleid = int(file[:-4]) # chop out '.txt' and parse to int
        with open(file, 'r') as infile:
            numAdded = 0
            for userid in infile.readlines():
                try:
                    uid = int(userid)
                    error = api.addRole(cur, guildid, roleid, int(userid))
                    if error:
                        print(f"Error adding user {userid!r} to role {roleid!r}!", file=sys.stderr)
                    else:
                        numAdded += 1
                except ValueError as e:
                    print(f"bad userid: {e!s}", file=sys.stderr)
            print(f"Added {numAdded} users to role {roleid!r}.")
    print(f"Added {rolesAdded} roles to guild {guildid!r}.")
    con.commit()


