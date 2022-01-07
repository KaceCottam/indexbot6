use dotenv as env;
use log::{error, info};
use poise::serenity_prelude::{Color, CreateEmbed};

use crate::Context;

pub fn successful_interaction(
    f: impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |g| {
        f(g).color(Color::DARK_GREEN)
            .title("✅ Success!")
            .footer(|f| f.text("For more help, type `$help`!"))
    }
}

pub fn unsuccessful_interaction(
    f: impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |g| {
        f(g).color(Color::DARK_RED)
            .title("❌ Failure")
            .footer(|f| f.text("If you think this is a mistake, please tell an admin!"))
    }
}

pub fn save_to_db(ctx: &Context) {
    match ctx
        .data()
        .0
        .lock()
        .unwrap()
        .save(env::var("BOT_ROLES_DB").unwrap())
    {
        Err(e) => error!("Error! {}", e),
        Ok(_) => info!("Saved to {}.", env::var("BOT_ROLES_DB").unwrap()),
    }
}
