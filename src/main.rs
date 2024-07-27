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
    tittle: String,
    description: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Notes {
    id: String,
    description: String,
    title: Option<String>,
}
#[function_component]
fn App() -> Html {
    let note = use_state(|| Note {
        tittle: "".to_string(),
        description: "".to_string(),
    });
    let response = use_state(|| None::<String>);
    let notes = use_state(|| Vec::new()); // State to hold the list of notes
    let is_loading = use_state(|| false); // State to handle loading status

    let oninput_tittle = {
        let note = note.clone();
        Callback::from(move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event
                .target()
                .unwrap_throw()
                .dyn_into()
                .unwrap_throw();
            if target.name() == "tittle" {
                note.set(Note {
                    tittle: target.value(),
                    ..(*note).clone()
                });
                // web_sys::console::log_1(&target.value().into());
            }
        })
    };

    let oninput_description = {
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
                // web_sys::console::log_1(&target.value().into());
            }
        })
    };

    let on_submit = {
        let note = note.clone();
        let response = response.clone();
        Callback::from(move |_| {
            let note = (*note).clone();
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
                        // response.set(Some(text));  // Optionally update response state if needed
                    }
                    Err(err) => {
                        let error_message = err.to_string();
                        web_sys::console::log_1(&JsValue::from_str(&error_message));
                    }
                }
            };

            spawn_local(response_future);
        })
    };

    let fetch_notes = {
        let notes = notes.clone();
        let is_loading = is_loading.clone();
        Callback::from(move |_| {
            let notes = notes.clone();
            let is_loading = is_loading.clone();
            let fetch_future = async move {
                is_loading.set(true);
                match Request::get("http://localhost:8080/api")
                    .send()
                    .await
                {
                    Ok(response) => {
                        let text = response.text().await.unwrap_or_default();
                        web_sys::console::log_1(&JsValue::from_str(&format!("Notes: {:?}", text)));
                        let fetched_notes: Vec<Notes> = match serde_json::from_str(&text) {
                            Ok(notes) => notes,
                            Err(err) => {
                                web_sys::console::log_1(&JsValue::from_str(&format!("Failed to notes: {:?}", err)));
                                Vec::new()
                            }
                        };
                        notes.set(fetched_notes);
			web_sys::console::log_1(&JsValue::from_str(&format!("notes: {:?}", notes)));
                    }
                    Err(err) => {
                        let error_message = err.to_string();
                        web_sys::console::log_1(&JsValue::from_str(&format!("Failed to fetch notes: {}", error_message)));
                    }
                }
                is_loading.set(false);
            };
            spawn_local(fetch_future);
        })
    };

   html! {
    <div>
        <div>
            <input name="tittle" oninput={oninput_tittle} placeholder="tittle" />
        </div>
        <div>
            <input name="description" oninput={oninput_description} placeholder="description" />
        </div>
        <button onclick={on_submit}>{ "Add me" }</button>

        <div>
            <button onclick={fetch_notes}>{ "Load Notes" }</button>
            { 
                if *is_loading {
                    html! { <p>{ "Loading..." }</p> }
                } else {
                    html! {
                        <ul>
                            {
                                for notes.iter().map(|note| html! {
                                    <li>
                                       { note.title.as_ref().unwrap_or(&"No title".to_string()) } {" "} 
                                       { if note.description.is_empty() { "No description".to_string() } else { note.description.clone() } }
                                    </li>
                                })
                            }
                        </ul>
                    }
                }
            }
        </div>
    </div>
}

}

fn main() {
    yew::Renderer::<App>::new().render();
}
