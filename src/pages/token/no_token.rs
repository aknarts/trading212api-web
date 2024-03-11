use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::services::requests::set_token;

#[function_component(NoToken)]
pub fn no_token() -> Html {
    let token = use_state(|| String::new());
    let oninput_token = {
        let token = token.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let current: String = input.value();
            token.set(current);
        })
    };

    let onsubmit = {
        let token = token.clone();
        Callback::from(move |e: SubmitEvent| {
            // e.prevent_default();
            set_token(Some((*token).clone()));
        })
    };

    html!(
        <>
            <h2> {"No Token found"}</h2>
            <p>{"Please generate it in your Account settings"}</p>
            <form {onsubmit} class="input-group mb-3">
                <span class="input-group-text"><i class="fa-solid fa-key"></i></span>
                <div class="form-floating">
                    <input id="token" type="text" class="form-control" placeholder="Token"
                        oninput={oninput_token}/>
                    <label for="token">{"Token"}</label>
                </div>
            </form>
        </>
    )
}