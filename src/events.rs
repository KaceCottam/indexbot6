use std::str::FromStr;

use chrono::Utc;
use log::{error, info};
use poise::serenity_prelude::{
    Activity, Context as SerenityContext, GuildId, Interaction,
    InteractionApplicationCommandCallbackDataFlags, InteractionResponseType, Message, Ready, Role,
    RoleId, User, UserId,
};

use crate::{Data, Error};

pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            error!("Error in command `{}`: {:?}", ctx.command().name, error)
        }
        _ => error!("Other error: {:?}", error),
    }
}

pub async fn event_listener(
    ctx: &SerenityContext,
    event: &poise::Event<'_>,
    _framework: &poise::Framework<Data, Error>,
    user_data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::GuildMemberRemoval {
            guild_id,
            user,
            member_data_if_available: _,
        } => on_guild_member_removal(guild_id, user),
        poise::Event::GuildRoleDelete {
            guild_id,
            removed_role_id,
            removed_role_data_if_available,
        } => on_guild_role_delete(guild_id, removed_role_id, removed_role_data_if_available),
        poise::Event::Ready { data_about_bot } => on_ready(ctx, data_about_bot).await,
        poise::Event::Message { new_message } => on_message(&ctx, user_data, new_message).await?,
        poise::Event::InteractionCreate { interaction } => {
            on_interaction_create(ctx, user_data, interaction).await?
        }
        _ => {}
    }

    Ok(())
}

pub async fn on_interaction_create(
    ctx: &SerenityContext,
    user_data: &Data,
    interaction: &Interaction,
) -> Result<(), poise::serenity_prelude::SerenityError> {
    let m = match interaction.clone().message_component() {
        Some(m) => m,
        None => return Ok(()),
    };
    let guild_id = match m.guild_id {
        Some(id) => id,
        _ => return Ok(()),
    };
    let role_id = u64::from_str(m.data.custom_id.as_str()).expect("Custom id was not u64.");

    let response =
        match user_data
            .0
            .lock()
            .unwrap()
            .add_user_to_role(guild_id.0, role_id, m.user.id.0)
        {
            Ok(_) => {
                info!("({}) {} joined {}!", guild_id, m.user.id, role_id);
                "✅ Added you to the role!"
            }
            Err(_) => "❌ Failed to add you to the role. *Are you already in it?*",
        };

    m.create_interaction_response(ctx, |f| {
        f.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|f| {
                f.content(response)
                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
            })
    })
    .await?;
    Ok(())
}

pub async fn on_message(
    ctx: &&SerenityContext,
    user_data: &Data,
    new_message: &Message,
) -> Result<(), poise::serenity_prelude::SerenityError> {
    let guild_id = match new_message.guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    let roles = new_message
        .mention_roles
        .iter()
        .map(|id| id.to_role_cached(&ctx).unwrap().name);

    if roles.len() == 0 {
        return Ok(());
    }

    let userids: Vec<_> = new_message
        .mention_roles
        .iter()
        .flat_map(|id| {
            user_data
                .0
                .lock()
                .unwrap()
                .show_users_of_role(guild_id.0, id.0)
                .into_iter()
                .copied()
                .map(UserId::from)
                .collect::<Vec<_>>()
        })
        .collect();

    if userids.is_empty() {
        return Ok(());
    }

    let message = new_message
        .channel_id
        .create_public_thread(&ctx, new_message.id, |f| {
            f.name(format!(
                "[{}] {} Discussion",
                Utc::now().format("%v"),
                roles.collect::<Vec<String>>().join(", ")
            ))
            .auto_archive_duration(1440)
            .kind(poise::serenity_prelude::model::channel::ChannelType::PublicThread)
        })
        .await;

    if message.is_err() {
        error!(
            "Failed to make thread for mention in message ({})! {}",
            new_message.id,
            message.unwrap_err()
        );
        return Ok(());
    }

    let thread = message.unwrap();

    for id in userids.iter() {
        thread.id.add_thread_member(&ctx, *id).await?;
    }

    thread
        .send_message(&ctx, |m| m.content("@everyone"))
        .await?;

    info!(
        "Notified roles ({}) in guild ({})!",
        new_message
            .mention_roles
            .iter()
            .map(|it| it.to_string())
            .collect::<Vec<String>>()
            .join(" "),
        guild_id
    );
    Ok(())
}

// TODO
pub fn on_guild_member_removal(guild_id: &GuildId, user: &User) {
    info!(
        "Guild ({}) member left: {}#{} ({})",
        guild_id, user.name, user.discriminator, user.id
    )
}

// TODO
pub async fn on_ready(ctx: &SerenityContext, data_about_bot: &Ready) {
    info!(
        "Bot {}#{} ({}) connected!",
        data_about_bot.user.name, data_about_bot.user.discriminator, data_about_bot.user.id
    );
    info!("Application id: {}", data_about_bot.application.id);
    info!("----------");
    ctx.set_activity(Activity::listening("$help")).await;
}

// TODO
pub fn on_guild_role_delete(
    guild_id: &GuildId,
    removed_role_id: &RoleId,
    removed_role_data_if_available: &Option<Role>,
) {
    if let Some(role) = removed_role_data_if_available {
        info!(
            "Guild ({}) role deleted {} ({})",
            guild_id, role.name, removed_role_id
        )
    } else {
        info!("Guild ({}) role deleted ({})", guild_id, removed_role_id)
    }
}
