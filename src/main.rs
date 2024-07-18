use std::vec;

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;
#[function_component]
fn App() -> Html {
    let name = use_state(|| String::new());
    let mut items = vec![];
    let oninput = Callback::from({
        let name = name.clone();
        move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event
                .target()
                .unwrap_throw()
                .dyn_into()
                .unwrap_throw();
            web_sys::console::log_1(&target.value().into()); // <- can console the value.
            name.set(target.value());
        }
    });
    let mut new_items = items.clone();
    let onclick = std::iter::from_fn(move || {
        new_items.push(name.clone());
        // web_sys::console::log_1(&greeting.into()); // if uncommented will print
    });

    html! {
        <div>
            <input {oninput}  />
            <button {onclick} >{ format!("Add me") }</button>
            <p>{"name: "}<h5>{&*name}</h5></p>
            {
            items.into_iter().map(|name| {
                html!{<div key={name.len()}>{ format!("Hello, I'am {:?}!",name) }</div>}
            }).collect::<Html>()
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}