use tracing::debug;
use yew::{function_component, hook, html, Html, HtmlResult};
use yew::suspense::{SuspensionResult, use_future, UseFutureHandle};

#[function_component(CashCardFallback)]
pub fn cash_fallback() -> Html {
    html!(
        <div class="card">
            <div class="card-body">
                <h5 class="card-title">{"Value"}</h5>
                <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Total: "}<span class="badge text-bg-secondary p-2">{"4"}</span></span></div>
                <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Invested: "}<span class="badge text-bg-secondary p-2">{"5"}</span></span></div>
                <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Free: "}<span class="badge text-bg-secondary p-2">{"6"}</span></span></div>
                <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Pie Free: "}<span class="badge text-bg-secondary p-2">{"7"}</span></span></div>
                <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Blocked: "}<span class="badge text-bg-secondary p-2">{"8"}</span></span></div>
                <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"PPL: "}<span class="badge text-bg-secondary p-2">{"9"}</span></span></div>
                <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Result: "}<span class="badge text-bg-secondary p-2">{"10"}</span></span></div>
            </div>
        </div>
    )
}

#[hook]
fn use_account_cash() -> SuspensionResult<
    UseFutureHandle<Result<trading212::models::cash::Cash, trading212::error::Error>>,
> {
    let user_ctx = crate::hooks::use_user_context::use_user_context();
    // user_ctx
    use_future(|| async move {
        if let Some(c) = user_ctx.client() {
            c.get_account_cash().await
        } else {
            Err(trading212::error::Error::NoClient)
        }
    })
}

#[function_component(CashCard)]
pub fn cash() -> HtmlResult {
    let res = use_account_cash()?;
    let html_result = match *res {
        Ok(ref cash) => {
            html!(
                <div class="card">
                    <div class="card-body">
                        <h5 class="card-title">{"Value"}</h5>
                        <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Total: "}<span class="badge text-bg-secondary p-2">{cash.total}</span></span></div>
                        <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Invested: "}<span class="badge text-bg-secondary p-2">{cash.invested}</span></span></div>
                        <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Free: "}<span class="badge text-bg-secondary p-2">{cash.free}</span></span></div>
                        <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Pie Free: "}<span class="badge text-bg-secondary p-2">{cash.pie_cash}</span></span></div>
                        <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Blocked: "}<span class="badge text-bg-secondary p-2">{cash.blocked.unwrap_or(0.0)}</span></span></div>
                        <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"PPL: "}<span class="badge text-bg-secondary p-2">{cash.ppl}</span></span></div>
                        <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Result: "}<span class="badge text-bg-secondary p-2">{cash.result}</span></span></div>
                    </div>
                </div>
            )
        }

        Err(ref e) => {
            match e {
                trading212::error::Error::Token => {
                    debug!("Authorization issue: {}", e);
                }
                _ => {
                    debug!("Failed to complete request: {e}");
                }
            };
            html!(500)
        }
    };

    Ok(html_result)
}
