
#[macro_use]
extern crate rocket;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use rocket::http::{Cookie, Cookies};
use rocket::response::Redirect;

use rspotify::{
    model::{AdditionalType, Country, Market, artist},
    prelude::*,
    scopes, AuthCodeSpotify, Credentials, OAuth,
};
const CACHE_PATH: &str = ".spotify_cache/";

fn get_cache_path(cookies: &Cookies) -> PathBuf {
    let project_dir_path = env::current_dir().unwrap();
    let mut cache_path = project_dir_path;
    cache_path.push(CACHE_PATH);
    cache_path.push(cookies.get("uuid").unwrap().value());

    cache_path
}

fn create_cache_path_if_absent(cookies: &Cookies) -> PathBuf {
    let cache_path = get_cache_path(cookies);
    if !cache_path.exists() {
        let mut path = cache_path.clone();
        path.pop();
        fs::create_dir_all(path).unwrap();
    }
    cache_path
}

fn remove_cache_path(mut cookies: Cookies) {
    let cache_path = get_cache_path(&cookies);
    if cache_path.exists() {
        fs::remove_file(cache_path).unwrap()
    }
    cookies.remove(Cookie::named("uuid"))
}

fn check_cache_path_exists(cookies: &Cookies) -> bool {
    let cache_path = get_cache_path(cookies);
    cache_path.exists()
}

fn init_spotify(cookies: &Cookies) -> AuthCodeSpotify {
    let config = Config {
        token_cached: true,
        cache_path: create_cache_path_if_absent(cookies),
        ..Default::default()
    };

    // Please notice that protocol of redirect_uri, make sure it's http
    // (or https). It will fail if you mix them up.
    let oauth = OAuth {
        scopes: scopes!("user-read-currently-playing", "playlist-modify-private"),
        redirect_uri: "http://localhost:8000/callback".to_owned(),
        ..Default::default()
    };

    // Replacing client_id and client_secret with yours.
    let creds = Credentials::new(
        "e1dce60f1e274e20861ce5d96142a4d3",
        "0e4e03b9be8d465d87fc32857a4b5aa3",
    );

    AuthCodeSpotify::with_config(creds, oauth, config)
}

#[get("/callback?<code>")]
fn callback(cookies: Cookies, code: String) -> AppResponse {
    let spotify = init_spotify(&cookies);

    match spotify.request_token(&code) {
        Ok(_) => {
            println!("Request user token successful");
            AppResponse::Redirect(Redirect::to("/"))
        }
        Err(err) => {
            println!("Failed to get user token {:?}", err);
            let mut context = HashMap::new();
            context.insert("err_msg", "Failed to get token!");
            AppResponse::Template(Template::render("error", context))
        }
    }
}

#[get("/")]
fn index(mut cookies: Cookies) -> AppResponse {
    let mut context = HashMap::new();

    // The user is authenticated if their cookie is set and a cache exists for
    // them.
    let authenticated = cookies.get("uuid").is_some() && check_cache_path_exists(&cookies);
    if !authenticated {
        cookies.add(Cookie::new("uuid", generate_random_uuid(64)));

        let spotify = init_spotify(&cookies);
        let auth_url = spotify.get_authorize_url(true).unwrap();
        context.insert("auth_url", auth_url);
        return AppResponse::Template(Template::render("authorize", context));
    }

    let cache_path = get_cache_path(&cookies);
    let token = Token::from_cache(cache_path).unwrap();
    let spotify = AuthCodeSpotify::from_token(token);
    match spotify.me() {
        Ok(user_info) => {
            context.insert(
                "display_name",
                user_info
                    .display_name
                    .unwrap_or_else(|| String::from("Dear")),
            );
            AppResponse::Template(Template::render("index", context.clone()))
        }
        Err(err) => {
            context.insert("err_msg", format!("Failed for {err}!"));
            AppResponse::Template(Template::render("error", context))
        }
    }
}

#[get("/sign_out")]
fn sign_out(cookies: Cookies) -> AppResponse {
    remove_cache_path(cookies);
    AppResponse::Redirect(Redirect::to("/"))
}

#[get("/playlists")]
fn playlist(cookies: Cookies) -> AppResponse {
    let spotify = init_spotify(&cookies);
    if !spotify.config.cache_path.exists() {
        return AppResponse::Redirect(Redirect::to("/"));
    }

    let token = spotify.read_token_cache(false).unwrap();
    spotify.token = Arc::new(Mutex::new(token));
    let playlists = spotify
        .current_user_playlists()
        .take(50)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    if playlists.is_empty() {
        return AppResponse::Redirect(Redirect::to("/"));
    }

    AppResponse::Json(json!(playlists))
}

#[get("/me")]
fn me(cookies: Cookies) -> AppResponse {
    let spotify = init_spotify(&cookies);
    if !spotify.config.cache_path.exists() {
        return AppResponse::Redirect(Redirect::to("/"));
    }

    spotify.token = Arc::new(Mutex::new(spotify.read_token_cache(false).unwrap()));
    match spotify.me() {
        Ok(user_info) => AppResponse::Json(json!(user_info)),
        Err(_) => AppResponse::Redirect(Redirect::to("/")),
    }
}

#[command]
async fn placeholder(ctx: &Context, msg: &Message) -> CommandResult {
    // send a direct message to the user
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
    
    /* let _ = msg.author.dm(&ctx, |m| {
        m.content(&url)
    }).await?; */

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