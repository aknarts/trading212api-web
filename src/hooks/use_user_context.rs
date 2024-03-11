use std::fmt;
use std::ops::Deref;

use yew::{hook, use_context, use_state, UseStateHandle};
use yew_hooks::use_local_storage;
use yew_router::hooks::use_navigator;
use yew_router::navigator::Navigator;

use crate::services::requests::set_token;
use crate::TOKEN_KEY;
use crate::types::auth::UserInfo;

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
        self.inner.set(UserInfo { token: None });
        set_token(None);
        // Redirect to home page
        self.history.push(&crate::app::Route::Logout);
    }

    pub fn login(&self, token: String) {
        // Clear global token after logged out
        self.inner.set(UserInfo {
            token: Some(token.clone()),
        });
        set_token(Some(token));
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
    let storage = use_local_storage::<String>(TOKEN_KEY.to_string());
    #[allow(clippy::or_fun_call)]
    let user_ctx = use_context::<UseStateHandle<UserInfo>>().unwrap_or(use_state(|| UserInfo {
        token: (&*storage).clone(),
    }));

    user_ctx
}
