use std::time::Duration;

use futures::{stream, StreamExt};
use log::info;
use poise::serenity_prelude::{
    ButtonStyle, Color, MessageBuilder, ReactionType, Role, RoleId, User, UserId,
};

use crate::api;
use crate::util::*;
use crate::{Context, Error};

async fn join_role(ctx: &Context<'_>, role: &Role, content: Option<String>) -> Result<(), Error> {
    let choice = ctx.data().0.lock().unwrap().add_user_to_role(
        ctx.guild_id().unwrap(),
        role.id,
        ctx.author().id,
    );
    if choice.is_err() {
        let message = MessageBuilder::new()
            .push("Failed to add ")
            .user(ctx.author())
            .push(" to role ")
            .role(role)
            .push_line("!")
            .push_italic("Are you already in the role?")
            .build();

        ctx.send(|f| f.embed(unsuccessful_interaction(|f| f.description(message))))
            .await?;
        return Ok(());
    }

    let message = MessageBuilder::new()
        .push("Added ")
        .user(ctx.author())
        .push(" to role ")
        .role(role.clone())
        .push("!")
        .build();

    let content = match content {
        Some(c) => format!("{}\n{}", c, message),
        None => message,
    };

    let m = ctx
        .send(|f| {
            f.embed(successful_interaction(|f| f.description(content.clone())))
                .components(|f| {
                    f.create_action_row(|f| {
                        f.create_button(|f| {
                            f.custom_id(role.id)
                                .emoji(ReactionType::from('🔔'))
                                .style(ButtonStyle::Primary)
                                .label("Join this role!")
                        })
                    })
                })
        })
        .await?;

    info!(
        "({}) {} joined {}!",
        role.guild_id,
        ctx.author().id,
        role.id
    );

    save_to_db(ctx.data());

    // wait 30 minutes
    tokio::spawn(tokio::time::sleep(Duration::from_secs(60 * 30))).await?;

    // turn off the button
    'timeout: {
        let m = match m {
            Some(m) => m,
            _ => break 'timeout,
        };
        let mut m = match m.message().await {
            Ok(m) => m,
            _ => break 'timeout,
        };

        m.edit(ctx.discord(), |f| {
            f.embed(successful_interaction(|f| {
                f.description(content.clone())
                    .footer(|f| f.text("Button timed out! Do a new command."))
            }))
            .components(|f| {
                f.create_action_row(|f| {
                    f.create_button(|f| {
                        f.custom_id(role.id)
                            .emoji(ReactionType::from('🔔'))
                            .style(ButtonStyle::Primary)
                            .label("Join this role!")
                            .disabled(true)
                    })
                })
            })
        })
        .await?;
    }
    Ok(())
}

/// Join the notification list for a role
#[poise::command(slash_command, category = "game")]
pub async fn join(
    ctx: Context<'_>,
    #[description = "Selected role"] role: Role,
) -> Result<(), Error> {
    join_role(&ctx, &role, None).await
}

/// Interact with game roles
#[poise::command(slash_command, category = "game")]
pub async fn game(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Join or create the notification list for a role
#[poise::command(slash_command, category = "game")]
pub async fn create(
    ctx: Context<'_>,
    #[description = "Selected game"] game: String,
) -> Result<(), Error> {
    let guild = match ctx.guild() {
        Some(g) => g,
        None => return Ok(()),
    };

    let mut content = String::new();

    let role = match guild.role_by_name(&game) {
        Some(role) => Ok(role.clone()),
        None => {
            content += format!("Created a new role {}!", game).as_str();
            guild
                .create_role(ctx.discord(), |f| f.name(&game).mentionable(true))
                .await
        }
    };

    if role.is_ok() {
        join_role(&ctx, &role.unwrap(), Some(content)).await?;
        return Ok(());
    }

    ctx.send(|f| {
        f.embed(unsuccessful_interaction(|f| {
            f.description("Failed to create the role!")
        }))
    })
    .await?;

    Ok(())
}

/// Leave the notification list for a role
#[poise::command(slash_command, category = "game")]
pub async fn leave(
    ctx: Context<'_>,
    #[description = "Selected role"] mut role: Role,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        _ => return Ok(()),
    };

    let choice =
        ctx.data()
            .0
            .lock()
            .unwrap()
            .remove_user_from_role(guild_id, role.id, ctx.author().id);

    let mut content = String::new();

    if choice.is_err() {
        let message = MessageBuilder::new()
            .push("Failed to remove ")
            .user(ctx.author())
            .push(" from role ")
            .role(role.clone())
            .push_line("!")
            .push_italic("Are you already not in the role?")
            .build();

        ctx.send(|f| f.embed(unsuccessful_interaction(|f| f.description(message))))
            .await?;
        return Ok(());
    }

    let members_empty = guild_id
        .members(ctx.discord(), None, None)
        .await?
        .into_iter()
        .find(|m| m.roles.contains(&role.id));
    let subscribers = ctx
        .data()
        .0
        .lock()
        .unwrap()
        .show_users_of_role(guild_id, role.id)
        .len();

    let role_deleted = match (subscribers, members_empty) {
        (0, None) => match role.delete(ctx.discord()).await {
            Ok(_) => {
                content += "💀 Role was deleted!";
                true
            }
            Err(_) => {
                content += "❌ Role can be deleted, but wasn't!";
                false
            }
        },
        _ => false,
    };

    let message = match (
        role_deleted,
        MessageBuilder::new()
            .push_line(content)
            .push("✅ Removed ")
            .user(ctx.author())
            .push(" from role "),
    ) {
        (true, m) => m.push(&role.name),
        (false, m) => m.role(&role),
    }
    .push("!")
    .build();

    ctx.send(|f| f.embed(successful_interaction(|f| f.description(message))))
        .await?;

    info!("({}) {} left {}!", role.guild_id, ctx.author().id, role.id);

    save_to_db(ctx.data());
    Ok(())
}

/// Display the members of a role
#[poise::command(slash_command, category = "game", ephemeral = true)]
pub async fn members(
    ctx: Context<'_>,
    #[description = "Selected role"] role: Role,
) -> Result<(), Error> {
    let mut users: Vec<_> = ctx
        .data()
        .0
        .lock()
        .unwrap()
        .show_users_of_role(role.guild_id, role.id)
        .into_iter()
        .copied()
        .map(UserId::from)
        .collect();

    users.dedup();

    let users: Vec<_> = stream::iter(users)
        .filter_map(|u| async move { u.to_user_cached(ctx.discord()).await })
        .collect()
        .await;

    let mb = users
        .into_iter()
        .fold(&mut MessageBuilder::new(), |mb, u| {
            mb.user(u);
            mb
        })
        .build();

    ctx.send(|f| {
        f.embed(|f| {
            f.title(format!("Users subscribed to {}:", role.name))
                .color(Color::DARK_GREEN)
                .description(mb)
        })
    })
    .await?;

    Ok(())
}

/// List the roles that a user will be notified for, or a guild if there is no user.
#[poise::command(slash_command, category = "game")]
pub async fn list(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<User>,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => return Ok(()),
    };

    let mut roles: Vec<api::RoleId> = user.as_ref().map_or_else(
        || {
            ctx.data()
                .0
                .lock()
                .unwrap()
                .show_roles_of_guild(guild_id)
                .into_iter()
                .copied()
                .collect()
        },
        |u| {
            ctx.data()
                .0
                .lock()
                .unwrap()
                .show_roles_of_user(guild_id, u.id)
                .into_iter()
                .copied()
                .collect()
        },
    );

    roles.sort();
    roles.dedup();

    let roles = roles
        .into_iter()
        .filter_map(|r| RoleId::from(r).to_role_cached(ctx.discord()));

    let title = match &user {
        Some(user) => user.name.clone(),
        None => ctx.guild().unwrap().name,
    };

    let mb = roles
        .fold(&mut MessageBuilder::new(), |mb, u| {
            mb.role(u);
            mb
        })
        .build();

    ctx.send(|f| {
        f.embed(|f| {
            f.title(format!("Roles of {}:", title))
                .color(Color::DARK_GREEN)
                .description(mb)
        })
    })
    .await?;

    Ok(())
}

/// Invite users to a role
#[poise::command(prefix_command, category = "game")]
pub async fn invite(
    ctx: Context<'_>,
    #[description = "Selected Role"] role: Role,
    #[description = "Selected Users"] users: Vec<User>,
) -> Result<(), Error> {
    let mut choices = users.into_iter().map(|u| {
        (
            u.clone(),
            ctx.data().0.lock().unwrap().add_user_to_role(
                ctx.guild_id().unwrap(),
                role.clone().id,
                u.id,
            ),
        )
    });

    if choices.any(|(_, r)| r.is_err()) {
        let message = MessageBuilder::new()
            .push("Failed to add someone to role ")
            .role(role.clone())
            .push_line("!")
            .push_italic("Are they already in the role?")
            .build();
        ctx.send(|f| f.embed(unsuccessful_interaction(|f| f.description(message))))
            .await?;
    }

    let added_users = choices.map(|(u, _)| u);

    let mut message = MessageBuilder::new();

    message.push("Added ");

    for u in added_users {
        message.user(u);
    }

    let message = message
        .push(" to role ")
        .role(role.clone())
        .push("!")
        .build();

    let m = ctx
        .send(|f| {
            f.embed(successful_interaction(|f| f.description(message.clone())))
                .components(|f| {
                    f.create_action_row(|f| {
                        f.create_button(|f| {
                            f.custom_id(role.id)
                                .emoji(ReactionType::from('🔔'))
                                .style(ButtonStyle::Primary)
                                .label("Join this role!")
                        })
                    })
                })
        })
        .await?;

    info!(
        "({}) {} joined {}!",
        role.guild_id,
        ctx.author().id,
        role.id
    );

    save_to_db(ctx.data());

    // wait 30 minutes
    tokio::spawn(tokio::time::sleep(Duration::from_secs(60 * 30))).await?;

    // turn off the button
    'timeout: {
        let m = match m {
            Some(m) => m,
            _ => break 'timeout,
        };
        let mut m = match m.message().await {
            Ok(m) => m,
            _ => break 'timeout,
        };

        m.edit(ctx.discord(), |f| {
            f.embed(successful_interaction(|f| {
                f.description(message.clone())
                    .footer(|f| f.text("Button timed out! Do a new command."))
            }))
            .components(|f| {
                f.create_action_row(|f| {
                    f.create_button(|f| {
                        f.custom_id(role.id)
                            .emoji(ReactionType::from('🔔'))
                            .style(ButtonStyle::Primary)
                            .label("Join this role!")
                            .disabled(true)
                    })
                })
            })
        })
        .await?;
    }
    Ok(())
}
