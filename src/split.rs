use gloo::events::EventListener;
use gloo_utils::window;
use stylist::Style;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{DomRect, HtmlElement};
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Clone, PartialEq)]
pub enum Axis {
    Vertical,
    Horizontal,
}

impl Axis {
    fn resize_dir(&self) -> &str {
        match self {
            Axis::Vertical => "width",
            Axis::Horizontal => "height",
        }
    }
    fn flex_dir(&self) -> &str {
        match self {
            Axis::Vertical => "row",
            Axis::Horizontal => "column",
        }
    }
    fn cursor(&self) -> &str {
        match self {
            Axis::Vertical => "col-resize",
            Axis::Horizontal => "row-resize",
        }
    }
    fn rect(&self, rect: &DomRect) -> i32 {
        match self {
            Axis::Vertical => rect.width() as i32,
            Axis::Horizontal => rect.height() as i32,
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub axis: Axis,
    #[prop_or_default]
    pub height: Option<String>,

    #[prop_or_default]
    pub left: Option<VNode>,
    #[prop_or_default]
    pub right: Option<VNode>,
    #[prop_or_default]
    pub top: Option<VNode>,
    #[prop_or_default]
    pub bottom: Option<VNode>,
}

#[function_component]
pub fn ResizeSplit(props: &Props) -> Html {
    let container = use_node_ref();
    let drag = use_node_ref();

    let is_resizing = use_mut_ref(|| false);
    let x = use_mut_ref(|| 0);
    let y = use_mut_ref(|| 0);
    let left_width = use_mut_ref(|| 50.0);
    let container_width = use_mut_ref(|| 0);

    let stopped_resizing = use_state(|| false);
    let new_left_width = use_state(|| *left_width.borrow_mut());

    let mut left_style = format!("{}: {}%;", props.axis.resize_dir(), *new_left_width);
    let mut right_style = String::from("flex: 1 1 0%;");
    let mut container_style = format!(
        "display: flex; flex: 1 1 0%; flex-direction: {};",
        props.axis.flex_dir()
    );

    if *is_resizing.borrow_mut() {
        let style = format!(
            "user-select:none; pointer-events:none; cursor:{};",
            props.axis.cursor()
        );
        left_style.push_str(&style);
        right_style.push_str(&style);
    }
    if let Some(height) = &props.height {
        container_style
            .push_str(format!("max-height:{height};min-height:{height};height:{height};").as_str());
    }

    let left_css = Style::new(left_style).expect("Failed to create left style");
    let right_css = Style::new(right_style).expect("Failed to create right style");
    let container_css = Style::new(container_style).expect("Failed to create cont. style");

    {
        // Create window mouse up listener
        let container_width = container_width.clone();
        let container = container.clone();
        let props = props.clone();
        use_effect_with_deps(
            |container| {
                let div = container
                    .cast::<HtmlElement>()
                    .expect("drag not attached to div element");
                let listener = EventListener::new(&window(), "mouseup", move |_| {
                    *container_width.borrow_mut() =
                        props.axis.rect(&div.get_bounding_client_rect());
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
        let props = props.clone();
        use_effect_with_deps(
            |container| {
                let div = container
                    .cast::<HtmlElement>()
                    .expect("drag not attached to div element");
                let listener = EventListener::new(&window(), "resize", move |_| {
                    *container_width.borrow_mut() =
                        props.axis.rect(&div.get_bounding_client_rect());
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
        let props = props.clone();
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
                        let size = props.axis.rect(&left_side.get_bounding_client_rect());
                        *left_width.borrow_mut() = size as f64;
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
        let props = props.clone();
        use_effect_with_deps(
            move |container| {
                let div = container
                    .cast::<HtmlElement>()
                    .expect("container not attached to div element");
                if *container_width.borrow_mut() == 0 {
                    *container_width.borrow_mut() =
                        props.axis.rect(&div.get_bounding_client_rect());
                }
                let listener =
                    Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(move |ev: MouseEvent| {
                        if *is_resizing.borrow_mut() {
                            let dx = ev.client_x() - *x.borrow_mut();
                            let dy = ev.client_y() - *y.borrow_mut();
                            let d = match props.axis {
                                Axis::Vertical => dx,
                                Axis::Horizontal => dy,
                            };
                            let w = ((*left_width.borrow_mut() + d as f64) * 100.0)
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

    match props.axis {
        Axis::Vertical => html! {
            <div ref={container} class={container_css}>
                <div class={format!("panel {}", left_css.get_class_name())} id="left">
                    { props.left.clone() }
                </div>
                <div ref={drag} class="drag" style="cursor:col-resize;" id="vertical"></div>
                <div class={format!("panel {}", right_css.get_class_name())} id="right">
                    { props.right.clone() }
                </div>
            </div>
        },
        Axis::Horizontal => html! {
            <div ref={container} class={container_css}>
                <div class={format!("panel {}", left_css.get_class_name())} id="top">
                    { props.top.clone() }
                </div>
                <div ref={drag} class="drag" style="cursor:row-resize;" id="horizontal"></div>
                <div class={format!("panel {}", right_css.get_class_name())} id="bottom">
                    { props.bottom.clone() }
                </div>
            </div>
        },
    }
}
