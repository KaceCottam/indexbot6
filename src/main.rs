#![warn(rustdoc::all)]
#![feature(async_closure)]
#![feature(label_break_value)]
#![feature(drain_filter)]

use dotenv as env;
use log::LevelFilter;
use poise::{FrameworkOptions, PrefixFrameworkOptions};
use std::path::PathBuf;

use crate::api::RolesDatabase;
use crate::deals::*;
use crate::events::*;
use crate::game::*;

mod api;
mod deals;
mod events;
mod game;
mod util;

#[derive(Debug)]
pub struct Data(std::sync::Mutex<RolesDatabase>);
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

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

// TODO admin forcejoin and forceleave user role

async fn command_check(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx.guild_id().is_some())
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter(Some("indexbot6"), LevelFilter::Info)
        .init();

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
            Box::pin(async move {
                Ok(Data {
                    0: std::sync::Mutex::from(db),
                })
            })
        })
        .options(options)
        .run()
        .await
        .expect("Client Error");
}
