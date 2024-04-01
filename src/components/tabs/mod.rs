use yew::{
    classes, function_component, html, use_context, use_state, Callback, Children, Html,
    Properties, UseReducerHandle, UseStateHandle,
};
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
    pub tabs: Vec<String>,
}

/// Tabbed view
#[function_component(Tabs)]
pub fn tabs(props: &Props) -> Html {
    let active_tab = use_state(|| 0usize);

    let active_tab_id = *active_tab;

    let handle_tabs = Callback::from(move |id: usize| {
        active_tab.set(id);
    });
    html! {
        <>
            <ul class="nav nav-tabs">
                { for props.tabs.iter().enumerate().map(|(index, tab)| {
                        let class = if index==active_tab_id {
                                                    Some("active")
                                                } else {
                                                    None
                                                };
                        html! { <li class="nav-item"><a class={classes!("nav-link", class)} onclick={ let handle_tabs= handle_tabs.clone(); move |_| { handle_tabs.emit(index); }} href="#">{ tab }</a></li>
                    }
                }) }
            </ul>
            <div class="tab-content">
                { for props.children.iter().enumerate().map(|(index,child)| {
                    let class = if active_tab_id == index {
                        vec!["show", "active"]
                    } else {
                        vec![]
                    };
                    html! { <div class={classes!("tab-pane","fade", class)}>{ child }</div> }}) }
            </div>
        </>
    }
}
