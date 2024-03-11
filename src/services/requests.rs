use std::sync::RwLock;

use gloo_storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use tracing::{debug, error};

use crate::TOKEN_KEY;
use crate::types::auth::Token;

lazy_static! {
    pub static ref TOKEN: RwLock<Option<Token>> = {
        LocalStorage::get(TOKEN_KEY)
            .map_or_else(|_| RwLock::new(None), |token| RwLock::new(Some(token)))
    };
}

/// Set jwt token to local storage.
pub fn set_token(token: Option<Token>) {
    token.clone().map_or_else(
        || {
            LocalStorage::delete(TOKEN_KEY);
        },
        |t| {
            LocalStorage::set(TOKEN_KEY, t).expect("failed to set");
        },
    );
    match TOKEN.write() {
        Ok(mut t) => {
            *t = token;
        }
        Err(e) => {
            error!("Error setting token: {:?}", e);
        }
    }
}

/// Get jwt token from lazy static.
pub fn get_token() -> Option<Token> {
    match TOKEN.read() {
        Ok(t) => t.clone(),
        Err(e) => {
            error!("Error getting token: {:?}", e);
            None
        }
    }
}
