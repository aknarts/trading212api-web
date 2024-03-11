use yew::{function_component, html, Html};

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

// #[hook]
// fn use_investments_list() -> SuspensionResult<UseFutureHandle<Result<Vec<Investment>, Error>>> {
//     use_future(|| async move { get_investments_list().await })
// }

// #[function_component(CashCard)]
// pub fn cash() -> HtmlResult {
//     html!(
//         <div class="card">
//             <div class="card-body">
//                 <h5 class="card-title">{"Value"}</h5>
//                 <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Total: "}<span class="badge text-bg-secondary p-2">{"4"}</span></span></div>
//                 <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Invested: "}<span class="badge text-bg-secondary p-2">{"5"}</span></span></div>
//                 <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Free: "}<span class="badge text-bg-secondary p-2">{"6"}</span></span></div>
//                 <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Pie Free: "}<span class="badge text-bg-secondary p-2">{"7"}</span></span></div>
//                 <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Blocked: "}<span class="badge text-bg-secondary p-2">{"8"}</span></span></div>
//                 <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"PPL: "}<span class="badge text-bg-secondary p-2">{"9"}</span></span></div>
//                 <div class="d-inline pe-1"><span class="badge text-bg-primary fs-6">{"Result: "}<span class="badge text-bg-secondary p-2">{"10"}</span></span></div>
//             </div>
//         </div>
//     )
// }
