use rust_decimal::prelude::ToPrimitive;
use trading212::models::time_event::Type;
use yew::{classes, function_component, html, use_state, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub exchange: trading212::models::exchange::Exchange,
}

#[function_component(Exchange)]
pub fn exchange(props: &Props) -> Html {
    let active = use_state(|| false);
    let active_class = if *active {
        (Some("show"), None)
    } else {
        (None, Some("collapsed"))
    };
    let exchange = props.exchange.clone();
    let name = exchange.name.clone();

    let status_class = bg_from_type(exchange.current_type());

    let next_status = exchange.next_event().map(|event| {
        let date = event.date;
        let difference = date - time::OffsetDateTime::now_utc();
        let total_seconds = difference.unsigned_abs().as_secs_f64().to_u64().unwrap();
        let days = total_seconds
            / time::convert::Second::per(time::convert::Day)
                .to_u64()
                .unwrap();
        let hours = total_seconds
            / time::convert::Second::per(time::convert::Hour)
                .to_u64()
                .unwrap()
            % time::convert::Hour::per(time::convert::Day)
                .to_u64()
                .unwrap();
        let minutes = total_seconds
            / time::convert::Second::per(time::convert::Minute)
                .to_u64()
                .unwrap()
            % time::convert::Minute::per(time::convert::Hour)
                .to_u64()
                .unwrap();
        let seconds = total_seconds
            % time::convert::Second::per(time::convert::Minute)
                .to_u64()
                .unwrap();
        let time_left = if days > 0 {
            format!("{:?}d {}:{:02}:{:02}", days, hours, minutes, seconds)
        } else if hours > 0 {
            format!("{}:{:02}:{:02}", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{:02}:{:02}", minutes, seconds)
        } else {
            format!("{:02}s", seconds)
        };

        let status = event.r#type;

        let class = bg_from_type(status);

        html! {
            <span class={classes!(class, "button", "p-2", "border-light", "rounded")}>
                <span>{ status.to_string() }{" in "}{ time_left }</span>
            </span>
        }
    });

    let onclick = { Callback::from(move |_| active.set(!*active)) };
    html!(<div class="accordion-item">
            <div class="accordion-header">
                <button class={classes!("accordion-button", active_class.1)} type="button" {onclick}>
                    <span class="me-2">{ name }</span>
                    <span class={classes!(status_class, "border", "me-2", "p-2", "border-light", "rounded-circle")}>
                        <span class="visually-hidden">{ exchange.current_type().to_string() }</span>
                    </span>
                    { next_status }
                </button>
            </div>
            <div class={classes!("accordion-collapse","collapse",active_class.0)}>
                <div class="accordion-body">
                </div>
            </div>
        </div>)
}

fn bg_from_type(event_type: Type) -> String {
    match event_type {
        Type::Open => "bg-success",
        Type::PreMarketOpen => "bg-warning",
        Type::AfterHoursOpen => "bg-danger",
        Type::BreakStart => "bg-danger-subtle",
        Type::BreakEnd => "bg-success-subtle",
        Type::Close => "bg-secondary",
        Type::AfterHoursClose => "bg-secondary",
        Type::Unknown => "bg-light",
    }
    .to_string()
}
