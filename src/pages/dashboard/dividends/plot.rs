use yew::{
    classes, function_component, html, use_context, use_state, Callback, Html, UseReducerHandle,
    UseStateHandle,
};

use crate::types::data::APIData;

#[function_component(DividendsPlot)]
pub fn dividends_plot() -> Html {
    let theme = use_context::<UseStateHandle<crate::types::theme::Theme>>().expect("no ctx found");
    let api = use_context::<UseReducerHandle<APIData>>().expect("no ctx found");
    let data = (*api).clone();

    let mut plot = yew_plotly::plotly::Plot::new();
    let mut values = Vec::new();
    data.dividends
        .sum_by_month()
        .iter()
        .for_each(|(month, sum)| {
            values.push((month.to_string().clone(), sum.clone()));
        });
    values.sort_by(|a, b| a.0.cmp(&b.0));
    let mut y_sum = Vec::new();
    let mut previous_sum = 0.0;
    let x = values
        .iter()
        .map(|(month, _)| month.clone())
        .collect::<Vec<String>>();
    let y = values
        .iter()
        .map(|(_, sum)| {
            previous_sum += sum;
            y_sum.push(previous_sum);
            sum.clone()
        })
        .collect::<Vec<f32>>();
    let trace = yew_plotly::plotly::Bar::new(x.clone(), y).name("Monthly");
    plot.add_trace(trace);
    let trace = yew_plotly::plotly::Scatter::new(x, y_sum)
        .name("Sum")
        .y_axis("y2");
    plot.add_trace(trace);
    let template = if (*theme).get_dark() {
        (*yew_plotly::plotly::layout::themes::PLOTLY_DARK).clone()
    } else {
        (*yew_plotly::plotly::layout::themes::PLOTLY_WHITE).clone()
    };
    let layout = yew_plotly::plotly::Layout::new()
        .template(template)
        .auto_size(true)
        .margin(yew_plotly::plotly::layout::Margin::new().auto_expand(true))
        .drag_mode(yew_plotly::plotly::layout::DragMode::False)
        .x_axis(
            yew_plotly::plotly::layout::Axis::new()
                .title(yew_plotly::plotly::common::Title::new("Date")),
        )
        .y_axis(
            yew_plotly::plotly::layout::Axis::new()
                .title(yew_plotly::plotly::common::Title::new("Monthly Dividends")),
        )
        .y_axis2(
            yew_plotly::plotly::layout::Axis::new()
                .overlaying("y")
                .title(yew_plotly::plotly::common::Title::new("Sum"))
                .side(yew_plotly::plotly::common::AxisSide::Right),
        )
        .legend(yew_plotly::plotly::layout::Legend::new())
        .paper_background_color(yew_plotly::plotly::common::color::NamedColor::Transparent)
        .plot_background_color(yew_plotly::plotly::common::color::NamedColor::Transparent);
    plot.set_layout(layout);

    html!(
        <div class="d-flex p-2 justify-content-center">
            <yew_plotly::Plotly plot={plot}/>
        </div>
    )
}
