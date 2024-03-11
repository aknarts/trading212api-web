use yew::{function_component, html, Html, Suspense};

use crate::pages::dashboard::cash::CashCard;

pub mod cash;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let fallback = html!(
        <>
            <cash::CashCardFallback />
        </>
    );

    html!(
        <>
            <Suspense {fallback}>
                <CashCard />
            </Suspense>
        </>
    )
}
