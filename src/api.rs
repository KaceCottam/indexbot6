#![macro_use]

use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use serde_derive::{Serialize, Deserialize};
use tqdb::{Database, search, remove, Query};
use thiserror::Error;

pub type Snowflake = String;
pub type GuildId = Snowflake;
pub type RoleId = Snowflake;
pub type UserId = Snowflake;

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
pub struct Roles {
    guild_id: GuildId,
    role_id: RoleId,
    user_id: UserId,
}

pub struct RolesDatabase(Database<Roles>);

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Duplicate item inserted")]
    Insertion,
    #[error("Couldn't read file")]
    BadRead,
    #[error("Item doesn't exist")]
    Removal,
}

type Result<T> = core::result::Result<T, ApiError>;

impl RolesDatabase {
    pub fn from(filename: &Path) -> Result<Self> {
        let file = File::open(filename).map_err(|_| ApiError::BadRead)?;
        let br = BufReader::new(file);
        let db = Database::try_from(br).map_err(|_| ApiError::BadRead)?;
        Ok(Self { 0: db })
    }
    pub fn add_user_to_role<G: Into<GuildId>, R: Into<RoleId>, U: Into<UserId>>(&mut self, guild_id: G, role_id: R, user_id: U) -> Result<()> {
        let guild_id = guild_id.into();
        let role_id = role_id.into();
        let user_id = user_id.into();
        self.0.insert_unique(Roles { guild_id, role_id, user_id }).map_err(|_| ApiError::Insertion)
    }
    pub fn show_roles_of_user<G: Into<GuildId>, U: Into<UserId>>(&self, guild_id: G, user_id: U) -> Vec<&RoleId> {
        let guild_id = guild_id.into();
        let user_id = user_id.into();
        search!(&self.0 => move |it: &Roles| it.guild_id == guild_id && it.user_id == user_id)
            .map(|it| &it.role_id)
            .collect()
    }
    pub fn show_users_of_role<G: Into<GuildId>, R: Into<RoleId>>(&self, guild_id: G, role_id: R) -> Vec<&UserId> {
        let guild_id = guild_id.into();
        let role_id = role_id.into();
        let mut res =
            search!(&self.0 => move |it: &Roles| it.guild_id == guild_id && it.role_id == role_id)
            .map(|it| &it.user_id)
            .collect::<Vec<&UserId>>();
        res.dedup();
        res
    }
    pub fn show_roles_of_guild<G: Into<GuildId>>(&self, guild_id: G) -> Vec<&RoleId> {
        let guild_id = guild_id.into();
        let mut res =
            search!(&self.0 => move |it: &Roles| it.guild_id == guild_id)
            .map(|it| &it.role_id)
            .collect::<Vec<&UserId>>();
        res.dedup();
        res
    }
    pub fn remove_user_from_role<G: Into<GuildId>, R: Into<RoleId>, U: Into<UserId>>(&mut self, guild_id: G, role_id: R, user_id: U) -> Result<UserId> {
        let guild_id = guild_id.into();
        let role_id = role_id.into();
        let user_id = user_id.into();
        remove!(&mut self.0 => move |it: &Roles| it.guild_id == guild_id && it.role_id == role_id && it.user_id == user_id )
            .next()
            .map(|it| it.user_id)
            .ok_or_else(|| ApiError::Removal)
    }
    pub fn remove_role<G: Into<GuildId>, R: Into<RoleId>>(&mut self, guild_id: G, role_id: R) -> Result<Vec<UserId>> {
        let guild_id = guild_id.into();
        let role_id = role_id.into();
        let users =
            remove!(&mut self.0 => move |it: &Roles| it.guild_id == guild_id && it.role_id == role_id )
                .map(|it| it.user_id)
                .collect::<Vec<UserId>>();
        if users.is_empty() {
            Err(ApiError::Removal)
        } else {
            Ok(users)
        }
    }
    pub fn remove_user<G: Into<GuildId>, U: Into<UserId>>(&mut self, guild_id: G, user_id: U) -> Result<Vec<RoleId>> {
        let guild_id = guild_id.into();
        let user_id = user_id.into();
        let roles =
            remove!(&mut self.0 => move |it: &Roles| it.guild_id == guild_id && it.user_id == user_id )
                .map(|it| it.role_id)
                .collect::<Vec<RoleId>>();
        if roles.is_empty() {
            Err(ApiError::Removal)
        } else {
            Ok(roles)
        }
    }
    pub fn remove_guild<G: Into<GuildId>>(&mut self, guild_id: G) -> Result<Vec<Roles>> {
        let guild_id = guild_id.into();
        let roles =
            remove!(&mut self.0 => move |it: &Roles| it.guild_id == guild_id )
                .collect::<Vec<Roles>>();
        if roles.is_empty() {
            Err(ApiError::Removal)
        } else {
            Ok(roles)
        }
    }
}

#[cfg(test)]
mod tests {
    use tqdb::Database;
    use crate::api::*;

    fn create_test_db() -> RolesDatabase {
        RolesDatabase {
            0:
            Database::from(vec![
                Roles { guild_id: "1".into(), role_id: "1".into(), user_id: "1".into() },
                Roles { guild_id: "1".into(), role_id: "1".into(), user_id: "2".into() },
                Roles { guild_id: "1".into(), role_id: "1".into(), user_id: "3".into() },
                Roles { guild_id: "1".into(), role_id: "1".into(), user_id: "4".into() },
                Roles { guild_id: "1".into(), role_id: "1".into(), user_id: "5".into() },
                Roles { guild_id: "1".into(), role_id: "2".into(), user_id: "1".into() },
                Roles { guild_id: "1".into(), role_id: "2".into(), user_id: "2".into() },
                Roles { guild_id: "1".into(), role_id: "2".into(), user_id: "6".into() },
                Roles { guild_id: "2".into(), role_id: "1".into(), user_id: "1".into() },
                Roles { guild_id: "2".into(), role_id: "1".into(), user_id: "6".into() },
                Roles { guild_id: "2".into(), role_id: "3".into(), user_id: "7".into() },
            ])
        }
    }

    #[test]
    pub fn test_add_user_to_role() {
        let mut db = create_test_db();
        assert!(db.add_user_to_role("1", "1", "1").is_err());
        assert!(db.add_user_to_role("1", "1", "1000").is_ok());
    }

    #[test]
    pub fn test_show_roles_of_user() {
        let db = create_test_db();
        assert_eq!(
            db.show_roles_of_user("1", "2"),
            vec![&"1".to_string(), &"2".to_string()]
        )
    }

    #[test]
    pub fn test_show_users_of_role() {
        let db = create_test_db();
        assert_eq!(
            db.show_users_of_role("1", "2"),
            vec![&"1".to_string(), &"2".to_string(), &"6".to_string()]
        )
    }

    #[test]
    pub fn test_show_roles_of_guild() {
        let db = create_test_db();
        assert_eq!(
            db.show_roles_of_guild("1"),
            vec![&"1".to_string(), &"2".to_string()]
        );
        assert_eq!(
            db.show_roles_of_guild("2"),
            vec![&"1".to_string(), &"3".to_string()]
        );
    }

    #[test]
    pub fn test_remove_user_from_role() {
        let mut db = create_test_db();
        assert!(db.remove_user_from_role("1", "1", "2").is_ok());
        assert!(db.remove_user_from_role("1", "10000", "2").is_err());
    }

    #[test]
    pub fn test_remove_role() {
        let mut db = create_test_db();

        assert_eq!(
            db.remove_role("1", "2").unwrap(),
            vec!["1".to_string(), "2".to_string(), "6".to_string()]
        );
        assert!(db.remove_role("1", "10000").is_err());
    }

    #[test]
    pub fn test_remove_user() {
        let mut db = create_test_db();
        assert_eq!(
            db.remove_user("1", "2").unwrap(),
            vec!["1".to_string(), "2".to_string()]
        );
        assert!(db.remove_user("1", "200000").is_err());
    }

    #[test]
    pub fn test_remove_guild() {
        let mut db = create_test_db();
        assert_eq!(
            db.remove_guild("2").unwrap(),
            vec![
                Roles { guild_id: "2".into(), role_id: "1".into(), user_id: "1".into() },
                Roles { guild_id: "2".into(), role_id: "1".into(), user_id: "6".into() },
                Roles { guild_id: "2".into(), role_id: "3".into(), user_id: "7".into() },
            ]
        );
        assert!(db.remove_guild("10000").is_err());
    }
}