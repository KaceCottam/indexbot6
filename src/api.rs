#![macro_use]
#![warn(rustdoc::all)]

use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use thiserror::Error;
use tqdb::{remove, search, Database, Query};

pub type Snowflake = u64;
pub type GuildId = Snowflake;
pub type RoleId = Snowflake;
pub type UserId = Snowflake;

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone, Default)]
pub struct Roles {
    guild_id: GuildId,
    role_id: RoleId,
    user_id: UserId,
}

impl Roles {
    pub fn new(guild_id: GuildId, role_id: RoleId, user_id: UserId) -> Self {
        Roles {
            guild_id,
            role_id,
            user_id,
        }
    }
}

unsafe impl Send for Roles {}
unsafe impl Sync for Roles {}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct RolesDatabase(Database<Roles>);

unsafe impl Send for RolesDatabase {}
unsafe impl Sync for RolesDatabase {}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Duplicate item inserted")]
    Insertion,
    #[error("Couldn't read file")]
    BadRead,
    #[error("Item doesn't exist")]
    Removal,
    #[error("Couldn't save the database")]
    BadSave,
}

type Result<T> = core::result::Result<T, ApiError>;

impl RolesDatabase {
    pub fn try_from<'a>(filename: impl Into<&'a Path>) -> Result<Self> {
        let file = File::open(filename.into()).map_err(|_| ApiError::BadRead)?;
        let br = BufReader::new(file);
        let db = Database::try_from(br).map_err(|_| ApiError::BadRead)?;
        Ok(Self { 0: db })
    }
    pub fn add_user_to_role<G: Into<GuildId>, R: Into<RoleId>, U: Into<UserId>>(
        &mut self,
        guild_id: G,
        role_id: R,
        user_id: U,
    ) -> Result<()> {
        self.0
            .insert_unique(Roles::new(guild_id.into(), role_id.into(), user_id.into()))
            .map_err(|_| ApiError::Insertion)
    }
    pub fn show_roles_of_user<G: Into<GuildId>, U: Into<UserId>>(
        &self,
        guild_id: G,
        user_id: U,
    ) -> Vec<&RoleId> {
        let guild_id = guild_id.into();
        let user_id = user_id.into();
        search!(&self.0 => move |it: &Roles| it.guild_id == guild_id && it.user_id == user_id)
            .map(|it| &it.role_id)
            .collect()
    }
    pub fn show_users_of_role<G: Into<GuildId>, R: Into<RoleId>>(
        &self,
        guild_id: G,
        role_id: R,
    ) -> Vec<&UserId> {
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
        let mut res = search!(&self.0 => move |it: &Roles| it.guild_id == guild_id)
            .map(|it| &it.role_id)
            .collect::<Vec<&UserId>>();
        res.dedup();
        res
    }
    pub fn remove_user_from_role<G: Into<GuildId>, R: Into<RoleId>, U: Into<UserId>>(
        &mut self,
        guild_id: G,
        role_id: R,
        user_id: U,
    ) -> Result<UserId> {
        let guild_id = guild_id.into();
        let role_id = role_id.into();
        let user_id = user_id.into();
        remove!(&mut self.0 => move |it: &Roles| it.guild_id == guild_id && it.role_id == role_id && it.user_id == user_id )
            .next()
            .map(|it| it.user_id)
            .ok_or(ApiError::Removal)
    }
    pub fn remove_role<G: Into<GuildId>, R: Into<RoleId>>(
        &mut self,
        guild_id: G,
        role_id: R,
    ) -> Result<Vec<UserId>> {
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
    pub fn remove_user<G: Into<GuildId>, U: Into<UserId>>(
        &mut self,
        guild_id: G,
        user_id: U,
    ) -> Result<Vec<RoleId>> {
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
        let roles = remove!(&mut self.0 => move |it: &Roles| it.guild_id == guild_id )
            .collect::<Vec<Roles>>();
        if roles.is_empty() {
            Err(ApiError::Removal)
        } else {
            Ok(roles)
        }
    }
    pub fn save<P: AsRef<Path>>(&self, filename: P) -> Result<()> {
        self.0.save_to_file(filename).or(Err(ApiError::BadSave))
    }
}

#[cfg(test)]
mod tests {
    use crate::api::*;
    use tqdb::Database;

    fn create_test_db() -> RolesDatabase {
        RolesDatabase {
            0: Database::from(vec![
                Roles::new(1, 1, 1),
                Roles::new(1, 1, 2),
                Roles::new(1, 1, 3),
                Roles::new(1, 1, 4),
                Roles::new(1, 1, 5),
                Roles::new(1, 2, 1),
                Roles::new(1, 2, 2),
                Roles::new(1, 2, 6),
                Roles::new(2, 1, 1),
                Roles::new(2, 1, 6),
                Roles::new(2, 3, 7),
            ]),
        }
    }

    #[test]
    pub fn test_add_user_to_role() {
        let mut db = create_test_db();
        assert!(db.add_user_to_role(1u64, 1u64, 1u64).is_err());
        assert!(db.add_user_to_role(1u64, 1u64, 1000u64).is_ok());
    }

    #[test]
    pub fn test_show_roles_of_user() {
        let db = create_test_db();
        assert_eq!(db.show_roles_of_user(1u64, 2u64), vec![&1u64, &2u64])
    }

    #[test]
    pub fn test_show_users_of_role() {
        let db = create_test_db();
        assert_eq!(db.show_users_of_role(1u64, 2u64), vec![&1u64, &2u64, &6u64])
    }

    #[test]
    pub fn test_show_roles_of_guild() {
        let db = create_test_db();
        assert_eq!(db.show_roles_of_guild(1u64), vec![&1u64, &2u64]);
        assert_eq!(db.show_roles_of_guild(2u64), vec![&1u64, &3u64]);
    }

    #[test]
    pub fn test_remove_user_from_role() {
        let mut db = create_test_db();
        assert!(db.remove_user_from_role(1u64, 1u64, 2u64).is_ok());
        assert!(db.remove_user_from_role(1u64, 10000u64, 2u64).is_err());
    }

    #[test]
    pub fn test_remove_role() {
        let mut db = create_test_db();

        assert_eq!(db.remove_role(1u64, 2u64).unwrap(), vec![1u64, 2u64, 6u64]);
        assert!(db.remove_role(1u64, 10000u64).is_err());
    }

    #[test]
    pub fn test_remove_user() {
        let mut db = create_test_db();
        assert_eq!(db.remove_user(1u64, 2u64).unwrap(), vec![1u64, 2u64]);
        assert!(db.remove_user(1u64, 200000u64).is_err());
    }

    #[test]
    pub fn test_remove_guild() {
        let mut db = create_test_db();
        assert_eq!(
            db.remove_guild(2u64).unwrap(),
            vec![
                Roles::new(2, 1, 1),
                Roles::new(2, 1, 6),
                Roles::new(2, 3, 7),
            ]
        );
        assert!(db.remove_guild(10000u64).is_err());
    }
}
