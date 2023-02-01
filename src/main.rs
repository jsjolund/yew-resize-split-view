use lipsum::lipsum;
use yew::{function_component, html, Html};

use crate::split::{Axis, ResizeSplit};

mod split;

#[function_component(App)]
pub fn app() -> Html {
    let left = html! {<><h1>{"left"}</h1><p>{lipsum(7000)}</p></>};
    let top = html! {<><h1>{"top"}</h1><p>{lipsum(700)}</p></>};
    let bottom1 = html! {<><h1>{"bottom1"}</h1><p>{lipsum(70)}</p></>};
    let bottom2 = html! {<><h1>{"bottom2"}</h1><p>{lipsum(7000)}</p></>};

    let bottom = html! {
        <ResizeSplit
            axis={Axis::Vertical}
            height={"100%".to_string()}
            left={bottom1}
            right={bottom2}
        />
    };
    let right = html! {
        <ResizeSplit
            axis={Axis::Horizontal}
            height={"100%".to_string()}
            top={top}
            bottom={bottom}
        />
    };
    html! {
        <ResizeSplit
            axis={Axis::Vertical}
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
