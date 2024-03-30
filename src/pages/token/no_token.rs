use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(NoToken)]
pub fn no_token() -> Html {
    let token = use_state(|| String::new());
    let proxy = use_state(|| "https://cors.redoc.ly/".to_string());
    let proxy_enabled = use_state(|| true);
    let live_environment = use_state(|| true);
    let user_ctx = crate::hooks::use_user_context::use_user_context();

    let oninput_token = {
        let token = token.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let current: String = input.value();
            token.set(current);
        })
    };

    let proxy_value = (*proxy).clone();

    let oninput_proxy = {
        let proxy = proxy.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let current: String = input.value();
            proxy.set(current);
        })
    };

    let live = *live_environment;

    let onclick = {
        let live_environment = live_environment.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let before = *live_environment;
            live_environment.set(!before);
        })
    };

    let proxy_e = *proxy_enabled;

    let onclick_proxy = {
        let proxy_enabled = proxy_enabled.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let before = *proxy_enabled;
            proxy_enabled.set(!before);
        })
    };

    let onsubmit = {
        let token = token.clone();
        let live = live_environment.clone();
        let proxy = proxy.clone();
        let proxy_enabled = proxy_enabled.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            user_ctx.login(
                (*token).clone(),
                (*live).clone(),
                if *proxy_enabled {
                    Some((*proxy).clone())
                } else {
                    None
                },
            );
        })
    };

    let live_classes = if live {
        vec!["btn-outline-success", "active"]
    } else {
        vec!["btn-outline-danger"]
    };

    let proxy_classes = if proxy_e {
        vec!["btn-outline-success", "active"]
    } else {
        vec!["btn-outline-danger"]
    };

    let login_enabled = !token.is_empty() && token.len() > 36;

    let login_classes = if login_enabled {
        vec!["btn-outline-success"]
    } else {
        vec!["btn-secondary"]
    };

    html!(
        <div class="container pb-0">
            <h2> {"No Token found"}</h2>
            <p>{"Please generate one in your Account settings"}</p>
            <form {onsubmit} class="mb-3">
                <div class="row mb-3">
                    <button {onclick} class={classes!("col-2", "btn", live_classes)}>{
                        if live {
                            "Live"
                        } else {
                            "Demo"
                        }
                    }</button>
                    <div class="input-group col">
                        <span class="input-group-text"><i class="fa-solid fa-key"></i></span>
                        <div class="form-floating">
                            <input id="token" type="text" class="form-control" placeholder="Token"
                                oninput={oninput_token}/>
                            <label for="token">{"Token"}</label>
                        </div>
                    </div>
                </div>
                <div class="row mb-3">
                    <button onclick={onclick_proxy} class={classes!("col-2", "btn", proxy_classes)}>{"Proxy"}</button>
                    <div class="input-group col">
                        <span class="input-group-text"><i class="fa-solid fa-server"></i></span>
                        <div class="form-floating">
                            <input id="proxy" type="text" value={proxy_value} class="form-control" placeholder="Proxy"
                                oninput={oninput_proxy} disabled={!proxy_e}/>
                            <label for="proxy">{"Proxy"}</label>
                        </div>
                    </div>
                </div>
                <div class="row">
                    <button class={classes!("col", "btn", login_classes)} disabled={!login_enabled}>{"Login"}</button>
                </div>
            </form>
        </div>
    )
}
