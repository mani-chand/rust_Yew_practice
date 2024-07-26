#[warn(unused_variables)]
#[warn(unused_mut)]
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use reqwasm::http::{Request, Response};
use wasm_bindgen_futures::spawn_local;
use serde_json;  // Import serde_json

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Note {
    title: String,
    description: String,
}

#[function_component]
fn App() -> Html {
    let note = use_state(|| Note {
        title: "".to_string(),
        description: "".to_string(),
    });
    let response = use_state(|| None::<String>);  // Specify the type for response state

    let oninput_title = {
        let note = note.clone();
        Callback::from(move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event
                .target()
                .unwrap_throw()
                .dyn_into()
                .unwrap_throw();
            if target.name() == "title" {
                note.set(Note {
                    title: target.value(),
                    ..(*note).clone()
                });
                web_sys::console::log_1(&target.value().into());
            }
        })
    };

    let oninput_age = {
        let note = note.clone();
        Callback::from(move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event
                .target()
                .unwrap_throw()
                .dyn_into()
                .unwrap_throw();
            if target.name() == "description" {
                note.set(Note {
                    description: target.value(),
                    ..(*note).clone()
                });
                web_sys::console::log_1(&target.value().into());
            }
        })
    };

    let on_submit = {
        let note = note.clone();
        let response = response.clone();
        Callback::from(move |_| {
            let note = (*note).clone(); // Cloning the note state to use inside the async block
            let response_future = async move {
                let request_body = match serde_json::to_string(&note) {
                    Ok(body) => body,
                    Err(err) => {
                        web_sys::console::log_1(&JsValue::from_str(&format!("Failed to serialize note: {:?}", err)));
                        return;
                    }
                };

                let request_result = Request::post("http://localhost:8080/api")
                    .header("Content-Type", "application/json")
                    .body(request_body)
                    .send()
                    .await;

                match request_result {
                    Ok(resp) => {
                        let text = resp.text().await.unwrap_or_default();
                        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(text.as_str()));
                        // response.set(Some(text));  // Correctly updating the state with response text
                    }
                    Err(err) => {
                        // Convert reqwasm::Error to a string for logging
                        let error_message = err.to_string();
                        web_sys::console::log_1(&JsValue::from_str(&error_message));
                    }
                }
            };

            spawn_local(response_future);
        })
    };

    html! {
        <div>
            <div>
                <input name="title" oninput={oninput_title} placeholder="title" />
            </div>
            <div>
                <input name="description" oninput={oninput_age} placeholder="description" />
            </div>
            <button onclick={on_submit}>{ "Add me" }</button>
            // {
            //     if let Some(response_text) = &*response {
            //         html! { <div>{ response_text.clone() }</div> }
            //     } else {
            //         html! {}
            //     }
            // }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
