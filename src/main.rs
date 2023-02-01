use yew::{function_component, html, Html};

use crate::components::{horizontal::HorizontalSplit, vertical::VerticalSplit};

mod components;

#[function_component(App)]
pub fn app() -> Html {
    let left = html! {"left"};
    let top = html! {"top"};
    let bottom1 = html! {"bottom1"};
    let bottom2 = html! {"bottom2"};

    let bottom = html! {
        <VerticalSplit
            height={"100%".to_string()}
            left={bottom1}
            right={bottom2}
        />
    };

    let right = html! {
        <HorizontalSplit
            height={"100%".to_string()}
            top={top}
            bottom={bottom}
        />
    };
    html! {
        <VerticalSplit
            height={"100vh".to_string()}
            left={left}
            right={right}
        />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
