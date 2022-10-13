use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use rspotify::{
    model::{AdditionalType, Country, Market, artist},
    prelude::*,
    scopes, AuthCodeSpotify, Credentials, OAuth,
};


#[command]
async fn placeholder(ctx: &Context, msg: &Message) -> CommandResult {
    // Set RSPOTIFY_CLIENT_ID and RSPOTIFY_CLIENT_SECRET in an .env file (after
    // enabling the `env-file` feature) or export them manually:
    //
    // export RSPOTIFY_CLIENT_ID="your client_id"
    // export RSPOTIFY_CLIENT_SECRET="secret"
    //
    // These will then be read with `from_env`.
    //
    // Otherwise, set client_id and client_secret explictly:
    //
    // ```
    // let creds = Credentials::new("my-client-id", "my-client-secret");
    // ```
    let creds = Credentials::from_env().unwrap();

    // Same for RSPOTIFY_REDIRECT_URI. You can also set it explictly:
    //
    // ```
    // let oauth = OAuth {
    //     redirect_uri: "http://localhost:8888/callback".to_string(),
    //     scopes: scopes!("user-read-recently-played"),
    //     ..Default::default(),
    // };
    // ```
    let oauth = OAuth::from_env(scopes!("user-read-currently-playing")).unwrap();

    let mut spotify = AuthCodeSpotify::new(creds, oauth);

    // Obtaining the access token
    let url = spotify.get_authorize_url(false).unwrap();
    // This function requires the `cli` feature enabled.
    spotify.prompt_for_token(&url).await.unwrap();
    // Running the requests
    let market = Market::Country(Country::Spain);
    let additional_types = [AdditionalType::Episode];
    let artists = spotify
        .current_playing(Some(&market), Some(&additional_types))
        .await;

    
    println!("Response: {artists:#?}");
    let song = artists.unwrap().unwrap().item.unwrap();
    let resp = format!("Response: {song:?}");
    let respn: &str = &resp[..];
    msg.channel_id.say(&ctx.http, respn).await?;
    Ok(())
}