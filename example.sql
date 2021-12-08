-- initialize table
-- id's are text because it is stored more accurately
CREATE TABLE roles (guildid TEXT, roleid TEXT, userid TEXT, UNIQUE(roleid, userid));

INSERT INTO roles VALUES(?, ?, ?);                                 -- add user to role in a server
SELECT userid FROM roles WHERE guildid = ? AND roleid = ?;         -- showing all users from role
SELECT roleid FROM roles WHERE guildid = ? AND userid = ?;         -- showing all roles from user
SELECT roleid FROM roles WHERE guildid = ?;                        -- showing all roles from server
SELECT COUNT(*) FROM roles WHERE guildid = ? AND roleid = ?;       -- count number of users from role
SELECT COUNT(*) FROM roles WHERE guildid = ? AND userid = ?;       -- count number of roles from user
SELECT COUNT(*) FROM roles WHERE guildid = ?;                      -- count number of roles from server
DELETE FROM roles WHERE guildid = ? AND roleid = ? AND userid = ?; -- remove role from user
DELETE FROM roles WHERE guildid = ? AND roleid = ?;                -- remove role from guild
DELETE FROM roles WHERE guildid = ? AND userid = ?;                -- user exit from guild
DELETE FROM roles WHERE guildid = ?;                               -- bot exit from guild