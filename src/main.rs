#![warn(rustdoc::all)]
#![feature(async_closure)]
#![feature(label_break_value)]
#![feature(drain_filter)]

use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use chrono::Utc;
use dotenv as env;
use poise::serenity_prelude::{
    Activity, ButtonStyle, Color, Context as SerenityContext, CreateEmbed,
    InteractionApplicationCommandCallbackDataFlags, InteractionResponseType, Mentionable,
    MessageBuilder, ReactionType, Role, User, UserId,
};
use poise::{serenity_prelude as serenity, FrameworkOptions, PrefixFrameworkOptions};

use crate::api::RolesDatabase;

mod api;

type Data = std::sync::Mutex<RolesDatabase>;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

fn successful_interaction(
    f: impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |g| {
        f(g).color(Color::DARK_GREEN)
            .title(":white_check_mark: Success!")
            .footer(|f| f.text("For more help, type `$help`!"))
    }
}

fn unsuccessful_interaction(
    f: impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |g| {
        f(g).color(Color::DARK_RED)
            .title(":x: Failure")
            .footer(|f| f.text("If you think this is a mistake, please tell an admin!"))
    }
}

async fn join_role(ctx: &Context<'_>, role: &Role, content: Option<String>) -> Result<(), Error> {
    let choice = ctx.data().lock().unwrap().add_user_to_role(
        ctx.guild_id().unwrap(),
        role.id,
        ctx.author().id,
    );
    match choice {
        Ok(_) => {
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

            println!(
                "({}) {} joined {}!",
                role.guild_id,
                ctx.author().id,
                role.id
            );

            // wait 3 minutes
            tokio::spawn(tokio::time::sleep(Duration::from_secs(60 * 3))).await?;

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
                    f.embed(successful_interaction(|f| f.description(content.clone())))
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

            save_to_db(&ctx);
        }
        Err(_) => {
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
        }
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

/// Category
#[poise::command(slash_command, category = "game")]
pub async fn game(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Join or create the notification list for a game
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
    let role = match role {
        Ok(r) => r,
        _ => {
            ctx.send(|f| {
                f.embed(unsuccessful_interaction(|f| {
                    f.description("Failed to create the role!")
                }))
            })
            .await?;
            return Ok(());
        }
    };

    join_role(&ctx, &role, Some(content)).await
}

/// Display the members of a role
#[poise::command(slash_command, category = "game")]
pub async fn members(
    ctx: Context<'_>,
    #[description = "Selected role"] role: Role,
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
    #[description = "Selected role"] mut role: Role,
) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        _ => return Ok(()),
    };

    let choice =
        ctx.data()
            .lock()
            .unwrap()
            .remove_user_from_role(guild_id, role.id, ctx.author().id);

    let mut content = String::new();

    match choice {
        Ok(_) => {
            let members_empty = guild_id
                .members(ctx.discord(), None, None)
                .await?
                .into_iter()
                .filter(|m| m.roles.contains(&role.id))
                .next()
                .is_none();
            let subscribers = ctx
                .data()
                .lock()
                .unwrap()
                .show_users_of_role(guild_id, role.id)
                .len();
            let role_deleted = match (subscribers, members_empty) {
                (0, true) => match role.delete(ctx.discord()).await {
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

            println!("({}) {} left {}!", role.guild_id, ctx.author().id, role.id);

            save_to_db(&ctx)
        }
        Err(_) => {
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
        }
    }
    Ok(())
}

fn save_to_db(ctx: &Context) {
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

/// List the roles that a user will be notified for, or a guild if there is no user.
#[poise::command(slash_command, category = "game")]
pub async fn list(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<User>,
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

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error)
        }
        _ => println!("Other error: {:?}", error),
    }
}

async fn event_listener(
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
            ctx.set_activity(Activity::listening("$help")).await;
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
                    .kind(poise::serenity_prelude::model::channel::ChannelType::PublicThread)
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
        poise::Event::InteractionCreate { interaction } => {
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
                    .lock()
                    .unwrap()
                    .add_user_to_role(guild_id.0, role_id, m.user.id.0)
                {
                    Ok(_) => {
                        println!("({}) {} joined {}!", guild_id, m.user.id, role_id);
                        ":white_check_mark: Added you to the role!"
                    }
                    Err(_) => ":x: Failed to add you to the role. *Are you already in it?*",
                };

            m.create_interaction_response(ctx, |f| {
                f.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|f| {
                        f.content(response)
                            .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                    })
            })
            .await?;
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
        commands: vec![
            help(),
            register(),
            poise::Command {
                subcommands: vec![join(), create(), members(), list(), leave()],
                ..game()
            },
            deals(),
        ],
        prefix_options: PrefixFrameworkOptions {
            prefix: Some("$".into()),
            mention_as_prefix: false,
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
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
        .run()
        .await
        .expect("Client Error");
}
