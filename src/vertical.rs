use gloo::events::EventListener;
use gloo_utils::window;
use stylist::Style;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub height: String,
    #[prop_or_default]
    pub left: VNode,
    #[prop_or_default]
    pub right: VNode,
}

#[function_component]
pub fn VerticalSplit(props: &Props) -> Html {
    let container = use_node_ref();
    let drag = use_node_ref();

    let is_resizing = use_mut_ref(|| false);
    let x = use_mut_ref(|| 0);
    let y = use_mut_ref(|| 0);
    let left_width = use_mut_ref(|| 50.0);
    let container_width = use_mut_ref(|| 0);

    let stopped_resizing = use_state(|| false);
    let new_left_width = use_state(|| *left_width.borrow_mut());

    let mut right_style = String::from("flex: 1 1 0%;");
    let mut left_style = format!("height:{}; width: {}%;", props.height, *new_left_width);

    if *is_resizing.borrow_mut() {
        right_style.push_str("user-select:none; pointer-events:none; cursor:col-resize;");
        left_style.push_str("user-select:none; pointer-events:none; cursor:col-resize;");
    }

    let right_css = Style::new(right_style).expect("Failed to create right style");
    let left_css = Style::new(left_style).expect("Failed to create left style");
    let drag_css = Style::new("cursor:col-resize;").expect("Failed to create drag style");
    {
        // Create window resize listener
        let container_width = container_width.clone();
        let container = container.clone();
        use_effect_with_deps(
            |container| {
                let div = container
                    .cast::<HtmlElement>()
                    .expect("drag not attached to div element");
                let listener = EventListener::new(&window(), "mouseup", move |_| {
                    *container_width.borrow_mut() = div.get_bounding_client_rect().width() as i32;
                });
                move || {
                    drop(listener);
                }
            },
            container,
        );
    }
    {
        // Create window resize listener
        let container_width = container_width.clone();
        let container = container.clone();
        use_effect_with_deps(
            |container| {
                let div = container
                    .cast::<HtmlElement>()
                    .expect("drag not attached to div element");
                let listener = EventListener::new(&window(), "resize", move |_| {
                    *container_width.borrow_mut() = div.get_bounding_client_rect().width() as i32;
                });
                move || {
                    drop(listener);
                }
            },
            container,
        );
    }
    {
        // Create mouse down listener
        let x = x.clone();
        let y = y.clone();
        let drag = drag.clone();
        let left_width = left_width.clone();
        let is_resizing = is_resizing.clone();
        use_effect_with_deps(
            |drag| {
                let div = drag
                    .cast::<HtmlElement>()
                    .expect("drag not attached to div element");
                let left_side = div.previous_element_sibling().unwrap();
                let listener =
                    Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(move |ev: MouseEvent| {
                        *x.borrow_mut() = ev.client_x();
                        *y.borrow_mut() = ev.client_y();
                        *left_width.borrow_mut() = left_side.get_bounding_client_rect().width();
                        *is_resizing.borrow_mut() = true;
                    }));
                div.add_event_listener_with_callback(
                    "mousedown",
                    listener.as_ref().unchecked_ref(),
                )
                .unwrap();
                move || {
                    drop(listener);
                }
            },
            drag,
        );
    }
    {
        // Create mouse move listener
        let container = container.clone();
        let is_resizing = is_resizing.clone();
        use_effect_with_deps(
            move |container| {
                let div = container
                    .cast::<HtmlElement>()
                    .expect("container not attached to div element");
                if *container_width.borrow_mut() == 0 {
                    *container_width.borrow_mut() = div.get_bounding_client_rect().width() as i32;
                }
                let listener =
                    Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(move |ev: MouseEvent| {
                        if *is_resizing.borrow_mut() {
                            let dx = ev.client_x() - *x.borrow_mut();
                            let _dy = ev.client_y() - *y.borrow_mut();
                            let w = ((*left_width.borrow_mut() + dx as f64) * 100.0)
                                / *container_width.borrow_mut() as f64;
                            new_left_width.set(w);
                        }
                    }));
                div.add_event_listener_with_callback(
                    "mousemove",
                    listener.as_ref().unchecked_ref(),
                )
                .unwrap();
                move || {
                    drop(listener);
                }
            },
            container,
        );
    }
    {
        // Create mouse up listener
        let container = container.clone();
        use_effect_with_deps(
            move |container| {
                let div = container
                    .cast::<HtmlElement>()
                    .expect("container not attached to div element");
                let listener = Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(move |_| {
                    *is_resizing.borrow_mut() = false;
                    stopped_resizing.set(true);
                }));
                div.add_event_listener_with_callback("mouseup", listener.as_ref().unchecked_ref())
                    .unwrap();
                move || {
                    drop(listener);
                }
            },
            container,
        );
    }
    html! {
        <div ref={container} class="container" style="display: flex;">
            <div class={left_css} id="left_panel">
            { props.left.clone() }
            </div>
            <div ref={drag} class={drag_css} id="drag_vertical"></div>
            <div class={right_css} id="right_panel">
            { props.right.clone() }
            </div>
        </div>
    }
}
