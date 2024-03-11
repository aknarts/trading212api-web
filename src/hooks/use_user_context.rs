use gloo_storage::LocalStorage;
use tracing::warn;
use yew::{hook, use_context, use_state, UseStateHandle};

use crate::types::auth::UserInfo;

#[hook]
/// This hook is used to manage user context.
pub fn use_refresh_user_context() -> UseStateHandle<UserInfo> {
    #[allow(clippy::or_fun_call)]
    let user_ctx = use_context::<UseStateHandle<UserInfo>>().unwrap_or(use_state(UserInfo::default));



    user_ctx
}