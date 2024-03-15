use std::fmt;
use std::ops::Deref;

use tracing::error;
use trading212::{Client, Target};
use yew::{hook, use_context, use_state, UseStateHandle};
use yew_hooks::use_local_storage;
use yew_router::hooks::use_navigator;
use yew_router::navigator::Navigator;

use crate::services::requests::set_token;
use crate::types::auth::{Token, UserInfo};
use crate::TOKEN_KEY;

/// State handle for the [`use_user_context`] hook.
pub struct Handle {
    inner: UseStateHandle<UserInfo>,
    history: Navigator,
}

impl Handle {
    pub fn navigate_to(&self, route: &crate::app::Route) {
        self.history.push(route);
    }

    pub fn logout(&self) {
        // Clear global token after logged out
        self.inner.set(UserInfo {
            token: None,
            client: None,
        });
        set_token(None);
        // Redirect to home page
        self.history.push(&crate::app::Route::Logout);
    }

    pub fn client(&self) -> Option<Client> {
        self.inner.client.clone()
    }

    pub fn login(&self, token: String, live: bool) {
        let client = trading212::Client::new(&token, trading212::Target::Live).unwrap();
        let t = Token {
            target: if live { Target::Live } else { Target::Demo },
            token,
        };
        self.inner.set(UserInfo {
            token: Some(t.clone()),
            client: Some(client),
        });

        set_token(Some(t.clone()));
        // Redirect to home page
        self.history.push(&crate::app::Route::Home);
    }
}

impl Deref for Handle {
    type Target = UserInfo;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Clone for Handle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            history: self.history.clone(),
        }
    }
}

impl PartialEq for Handle {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Handle")
            .field("value", &format!("{:?}", *self.inner))
            .finish()
    }
}

#[hook]
/// This hook is used to manage user context.
pub fn use_user_context() -> Handle {
    let inner = use_context::<UseStateHandle<UserInfo>>().unwrap();
    let history = use_navigator().unwrap();

    Handle { inner, history }
}

#[hook]
/// This hook is used to manage user context.
pub fn use_refresh_user_context() -> UseStateHandle<UserInfo> {
    let storage = use_local_storage::<Token>(TOKEN_KEY.to_string());
    let token: Option<Token> = (&*storage).clone();
    let client = match token.clone() {
        None => None,
        Some(t) => match Client::new(&t.token, t.target) {
            Ok(c) => Some(c),
            Err(e) => {
                error!("Error creating client: {:?}", e);
                None
            }
        },
    };

    #[allow(clippy::or_fun_call)]
    let user_ctx = use_context::<UseStateHandle<UserInfo>>().unwrap_or(use_state(|| UserInfo {
        token: token.clone(),
        client,
    }));

    user_ctx
}
