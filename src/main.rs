mod vertical;

use vertical::VerticalSplit;
use yew::{function_component, html, Html};

#[function_component(App)]
pub fn app() -> Html {
    let left1 = html! {"left1"};
    let right1 = html! {"right"};

    let left0 = html! {"left0"};
    let right0 = html! {
        <VerticalSplit
            height={"100vh".to_string()}
            left={left1}
            right={right1}
        />
    };
    html! {
        <VerticalSplit
            height={"100vh".to_string()}
            left={left0}
            right={right0}
        />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
