#![warn(rustdoc::all)]
#![feature(async_closure)]

use std::path::PathBuf;

use chrono::Utc;
use dotenv as env;
use poise::serenity_prelude::{Activity, Mentionable, UserId};
use poise::{serenity_prelude as serenity, FrameworkOptions, PrefixFrameworkOptions};

use crate::api::RolesDatabase;

mod api;

type Data = std::sync::Mutex<RolesDatabase>;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Join the notification list for a role
#[poise::command(slash_command, category = "game")]
pub async fn join(
    ctx: Context<'_>,
    #[description = "Selected role"] role: serenity::Role,
) -> Result<(), Error> {
    let choice = ctx.data().lock().unwrap().add_user_to_role(
        ctx.guild_id().unwrap(),
        role.id,
        ctx.author().id,
    );
    match choice {
        Ok(_) => {
            ctx.say(format!(
                "Added {} to role {}!",
                ctx.author().name,
                role.name
            ))
            .await?;
            println!(
                "({}) {} joined {}!",
                role.guild_id,
                ctx.author().id,
                role.id
            );
            match ctx
                .data()
                .lock()
                .unwrap()
                .save(env::var("BOT_ROLES_DB").unwrap())
            {
                Err(e) => println!("Error! {}", e),
                Ok(_) => println!("Saved to {}.", env::var("BOT_ROLES_DB").unwrap()),
            }
        }
        Err(_) => {
            ctx.say("Error joining role!").await?;
        }
    }
    Ok(())
}

/// Category
#[poise::command(slash_command, category = "game")]
pub async fn game(
    ctx: Context<'_>,
    #[description = "Selected game"] game: String,
) -> Result<(), Error> {
    ctx.say(format!(
        "{} wants to create/join game {}!",
        ctx.author().name,
        game
    ))
    .await?;
    Ok(())
}

/// Join or create the notification list for a game
#[poise::command(slash_command, category = "game")]
pub async fn create(
    ctx: Context<'_>,
    #[description = "Selected game"] game: String,
) -> Result<(), Error> {
    ctx.say(format!(
        "{} wants to create/join game {}!",
        ctx.author().name,
        game
    ))
    .await?;
    Ok(())
}

/// Display the members of a role
#[poise::command(slash_command, category = "game")]
pub async fn members(
    ctx: Context<'_>,
    #[description = "Selected role"] role: serenity::Role,
) -> Result<(), Error> {
    ctx.say(format!(
        "{} wants to see the role {}!",
        ctx.author().name,
        role.mention()
    ))
    .await?;
    Ok(())
}

/// Leave the notification list for a role
#[poise::command(slash_command, category = "game")]
pub async fn leave(
    ctx: Context<'_>,
    #[description = "Selected role"] role: serenity::Role,
) -> Result<(), Error> {
    ctx.say(format!(
        "{} wants to leave the role {}!",
        ctx.author().name,
        role.mention()
    ))
    .await?;
    Ok(())
}

/// List the roles that a user will be notified for, or a guild if there is no user.
#[poise::command(slash_command, category = "game")]
pub async fn list(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    if let Some(user) = user {
        ctx.say(format!(
            "{} wants list the roles of user {}!",
            ctx.author().name,
            user.mention()
        ))
        .await?;
    } else {
        ctx.say(format!(
            "{} wants list the roles of the server!",
            ctx.author().name
        ))
        .await?;
    }
    Ok(())
}

/// Fetch game deals from isthereanydeal.com for a game
#[poise::command(slash_command)]
pub async fn deals(
    ctx: Context<'_>,
    #[description = "Selected Game"] game: String,
) -> Result<(), Error> {
    ctx.say(format!(
        "{} wants to see deals for the game \"{}\"!",
        ctx.author().name,
        game
    ))
    .await?;
    Ok(())
}

/// Show this menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: " Type $help command for more info on a command. You can edit your message to the bot and the bot will edit its response.",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}

/// Register slash commands in this guild or globally
///
/// Run with no arguments to register in guild, run with argument "global" to register globally.
#[poise::command(prefix_command, hide_in_help)]
async fn register(ctx: Context<'_>, #[flag] global: bool) -> Result<(), Error> {
    poise::builtins::register_application_commands(ctx, global).await?;
    Ok(())
}

async fn on_error(error: Error, ctx: poise::ErrorContext<'_, Data, Error>) {
    match ctx {
        poise::ErrorContext::Setup => panic!("Failed to start bot: {:?}", error),
        poise::ErrorContext::Command(ctx) => {
            println!("Error in command `{}`: {:?}", ctx.command().name(), error)
        }
        _ => println!("Other error: {:?}", error),
    }
}

async fn event_listener(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: &poise::Framework<Data, Error>,
    user_data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::GuildMemberRemoval {
            guild_id,
            user,
            member_data_if_available: _,
        } => {
            println!(
                "Guild ({}) member left: {}#{} ({})",
                guild_id, user.name, user.discriminator, user.id
            )
        }
        poise::Event::GuildRoleDelete {
            guild_id,
            removed_role_id,
            removed_role_data_if_available,
        } => {
            if let Some(role) = removed_role_data_if_available {
                println!(
                    "Guild ({}) role deleted {} ({})",
                    guild_id, role.name, removed_role_id
                )
            } else {
                println!("Guild ({}) role deleted ({})", guild_id, removed_role_id)
            }
        }
        poise::Event::Ready { data_about_bot } => {
            println!(
                "Bot {}#{} ({}) connected!",
                data_about_bot.user.name, data_about_bot.user.discriminator, data_about_bot.user.id
            );
            println!("Application id: {}", data_about_bot.application.id);
            println!("----------");
            ctx.set_activity(Activity::listening("/help")).await;
        }
        poise::Event::Message { new_message } => {
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

            let userids = new_message
                .mention_roles
                .iter()
                .flat_map(|id| {
                    user_data
                        .lock()
                        .unwrap()
                        .show_users_of_role(guild_id.0, id.0)
                        .into_iter()
                        .copied()
                        .collect::<Vec<_>>()
                })
                .map(UserId::from)
                .collect::<Vec<_>>();

            if userids.is_empty() {
                return Ok(());
            }

            let thread = match new_message
                .channel_id
                .create_public_thread(&ctx, new_message.id, |f| {
                    f.name(format!(
                        "[{}] {} Discussion",
                        Utc::now().format("%v"),
                        roles.collect::<Vec<String>>().join(", ")
                    ))
                    .auto_archive_duration(1440)
                    .kind(serenity::model::channel::ChannelType::PublicThread)
                })
                .await
            {
                Ok(t) => t,
                Err(e) => {
                    println!(
                        "Failed to make thread for mention in message ({})! {}",
                        new_message.id, e
                    );
                    return Ok(());
                }
            };

            for id in userids.iter() {
                thread.id.add_thread_member(&ctx, *id).await?;
            }

            thread
                .send_message(&ctx, |m| m.content("@everyone"))
                .await?;

            println!(
                "Notified roles ({}) in guild ({})!",
                new_message
                    .mention_roles
                    .iter()
                    .map(|it| it.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
                guild_id
            );
        }
        _ => {}
    }

    Ok(())
}

async fn command_check(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx.guild_id().is_some())
}

#[tokio::main]
async fn main() {
    env::dotenv().expect("Error loading environment file.");
    let db_file: PathBuf = env::var("BOT_ROLES_DB")
        .expect("Expected BOT_ROLES_DB to be set in environment.")
        .into();
    let db = RolesDatabase::try_from(db_file.as_path()).unwrap_or_default();
    let options = FrameworkOptions {
        prefix_options: PrefixFrameworkOptions {
            prefix: Some("$".into()),
            mention_as_prefix: false,
            ..Default::default()
        },
        on_error: |error, ctx| Box::pin(on_error(error, ctx)),
        listener: |ctx, event, framework, user_data| {
            Box::pin(event_listener(ctx, event, framework, user_data))
        },
        command_check: Some(|ctx| Box::pin(command_check(ctx))),
        ..Default::default()
    };
    poise::Framework::build()
        .token(env::var("BOT_TOKEN").expect("Expected BOT_TOKEN to be set in environment."))
        .user_data_setup(move |_ctx, _ready, _framework| {
            Box::pin(async move { Ok(std::sync::Mutex::from(db)) })
        })
        .options(options)
        .command(game(), |f| {
            f.subcommand(join(), |f| f)
                .subcommand(create(), |f| f)
                .subcommand(members(), |f| f)
                .subcommand(list(), |f| f)
                .subcommand(leave(), |f| f)
        })
        .command(register(), |f| f)
        .command(deals(), |f| f)
        .command(help(), |f| f)
        .run()
        .await
        .expect("Client Error");
}
