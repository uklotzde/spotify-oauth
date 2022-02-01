//! # Spotify OAuth
//!
//! An implementation of the Spotify Authorization Code Flow in Rust.
//!
//! # Basic Example
//!
//! ```no_run
//! use std::{io::stdin, str::FromStr, error::Error};
//! use spotify_oauth::{convert_callback_into_token, AppClient, SpotifyAuth, SpotifyCallback, SpotifyScope, SurfClient};
//! use url::Url;
//!
//! #[async_std::main]
//! async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
//!
//!     // Setup Spotify Auth URL
//!     let auth = SpotifyAuth {
//!         response_type : "code".to_string(),
//!         scope : vec![SpotifyScope::Streaming],
//!         show_dialog : false,
//!         app_client : AppClient {
//!           id: "YOUR_SPOTIFY_CLIENT_ID".to_string(),
//!           secret : "YOUR_SPOTIFY_CLIENT_SECRET".to_string(),
//!         },
//!         redirect_uri : Url::parse("http://localhost:8080/callback").unwrap(),
//!         state : "-use-a-radom-string-".to_string()
//!     };
//!     let auth_url = auth.authorize_url()?;
//!
//!     // Open the auth URL in the default browser of the user.
//!     open::that(auth_url)?;
//!
//!     println!("Input callback URL:");
//!     let mut buffer = String::new();
//!     stdin().read_line(&mut buffer)?;
//!
//!     let callback = SpotifyCallback::from_str(buffer.trim())?;
//!     // Convert the given callback URL into a token.
//!     let token = convert_callback_into_token(SurfClient, callback, &auth.app_client, auth.redirect_uri).await?;
//!
//!     println!("Token: {:#?}", token);
//!
//!     Ok(())
//! }
//! ```

mod app_client;
mod auth;
mod callback;
mod error;
mod fetch;
mod scope;
#[cfg(feature = "surf")]
mod surf;
mod token;
mod util;

use crate::error::*;

pub use crate::{app_client::*, auth::*, callback::*, fetch::*, scope::*, token::*, util::*};

#[cfg(feature = "surf")]
pub use crate::surf::*;

const SPOTIFY_AUTH_URL: &str = "https://accounts.spotify.com/authorize";
