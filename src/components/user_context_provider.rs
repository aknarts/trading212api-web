//! User context provider.
use crate::types::auth::UserInfo;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

const TOKEN_KEY: &str = "token.trading212api.self";

/// User context provider.
#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = crate::hooks::use_user_context::use_refresh_user_context();


    html!(
        <ContextProvider<UseStateHandle<UserInfo>> context={user_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserInfo>>>
    )
}
