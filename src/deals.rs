use crate::util::unsuccessful_interaction;
use crate::{Context, Error};
use dotenv as env;
use poise::serenity_prelude;
use serde::de::DeserializeOwned;
use serde::Deserialize;

// https://api.isthereanydeal.com/v02/game/plain/?key=8c6c9916595b10f45501ace208c34d19e8f1dc6d&title=arma%203
/// Format the is there any deal get url for a get request searching deals for a game.
pub fn format_itad_plain_uri(search_text: &str) -> String {
    format!(
        "https://api.isthereanydeal.com/v02/game/plain/?key={}&title={}",
        env::var("ITAD_API_KEY").expect("Expected IsThereAnyDeal API Key to be in .env file!"),
        percent_encoding::percent_encode(
            search_text.as_bytes(),
            percent_encoding::NON_ALPHANUMERIC
        )
    )
}

// https://api.isthereanydeal.com/v01/game/prices/?key=&plains=arma3&country=US&shops=steam
/// Format the is there any deal get url for a get request searching deals for a game.
pub fn format_itad_deal_uri(plain: &str) -> String {
    format!(
        "https://api.isthereanydeal.com/v01/game/prices/?key={}&plains={}&country=US", // &shops=steam%20gog%20humble%20fanatical%20greenmangaming
        env::var("ITAD_API_KEY").expect("Expected IsThereAnyDeal API Key to be in .env file!"),
        plain
    )
}

// may need later so keeping it in
// #[derive(Deserialize, Debug)]
// pub struct ItadShop {
//     id: String,
//     name: String,
// }

#[derive(Deserialize, Debug)]
pub struct ItadDeal {
    price_new: f64,
    price_old: f32,
    price_cut: i64,
    url: String,
    // shop: ItadShop,
    // drm: Vec<String>,
}

async fn do_http_request<T: DeserializeOwned>(uri: &str, json_data_pointer: &str) -> Option<T> {
    // first we have to get the plain identifier for the game
    let body = reqwest::get(uri).await.ok()?.text().await.ok()?;

    // body is json
    let body: serde_json::Value = match serde_json::from_str(body.as_str()) {
        Ok(content) => content,
        _ => return None,
    };

    let ptr = match body.pointer(json_data_pointer) {
        Some(s) => s,
        _ => return None,
    };

    serde_json::from_value(ptr.clone()).ok()
}

/// Fetch game deals from isthereanydeal.com for a game
#[poise::command(slash_command)]
pub async fn deals(
    ctx: Context<'_>,
    #[description = "Selected Game"] game: String,
) -> Result<(), Error> {
    'no_error: {
        let plain_id: String =
            match do_http_request(format_itad_plain_uri(&game).as_str(), "/data/plain").await {
                Some(s) => s,
                _ => break 'no_error,
            };

        let deal: ItadDeal = match do_http_request(
            format_itad_deal_uri(plain_id.as_str()).as_str(),
            format!("/data/{}/list/0", plain_id).as_str(),
        )
        .await
        {
            Some(s) => s,
            _ => break 'no_error,
        };

        let color = match deal.price_cut {
            0..=24 => serenity_prelude::Color::DARK_RED,
            25..=49 => serenity_prelude::Color::GOLD,
            _ => serenity_prelude::Color::DARK_GREEN,
        };

        if deal.price_cut != 0 {
            ctx.send(|f| {
                f.embed(|f| {
                    f.title(format!("Deals for {}", game))
                        .color(color)
                        .description("Looking at isthereanydeal.com, there is a deal!")
                        .field("New Price", format!("${}", deal.price_new), true)
                        .field("Old Price", format!("${}", deal.price_old), true)
                        .field("Price Cut", format!("{}%", deal.price_cut), true)
                        .field("Link", deal.url, true)
                })
            })
            .await?;
        } else {
            ctx.send(|f| {
                f.embed(|f| {
                    f.title(format!("Deals for {}", game))
                        .color(color)
                        .description(
                            "Looking at isthereanydeal.com, there are no deals at the moment.",
                        )
                        .field("Old Price", format!("${}", deal.price_old), true)
                        .field("Link", deal.url, true)
                })
            })
            .await?;
        }

        return Ok(());
    }

    // out of no error, assume error has occurred
    ctx.send(|f| {
        f.embed(unsuccessful_interaction(|f| {
            f.description("Something has went wrong, is isthereanydeal.com down?")
        }))
    })
    .await?;
    Ok(())
}
