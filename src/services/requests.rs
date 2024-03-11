use std::sync::{LockResult, RwLock};
use gloo_storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use tracing::{debug, error};

pub const API_ROOT: &str = "http://localhost:8081/";
const TOKEN_KEY: &str = "token.trading212api.self";

lazy_static! {
    /// Jwt token read from local storage.
    pub static ref TOKEN: RwLock<Option<String>> = {
        LocalStorage::get(TOKEN_KEY).map_or_else(|_| RwLock::new(None), |token| RwLock::new(Some(token)))
    };
}

/// Set jwt token to local storage.
pub fn set_token(token: Option<String>) {
    token.clone().map_or_else(
        || {
            LocalStorage::delete(TOKEN_KEY);
        },
        |t| {
            LocalStorage::set(TOKEN_KEY, t).expect("failed to set");
        },
    );
    let mut token_lock = TOKEN.write();
    match token_lock {
        Ok(mut t) => {
            *t = token;
        }
        Err(e) => {
            error!("Error setting token: {:?}", e);
        }
    }
}

/// Get jwt token from lazy static.
pub fn get_token() -> Option<String> {
    let token_lock = TOKEN.read();
    match token_lock {
        Ok(t) => {
            debug!("Token: {:?}", t);
            t.clone()
        }
        Err(e) => {
            error!("Error getting token: {:?}", e);
            None
        }
    }
}