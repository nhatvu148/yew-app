use gloo::console::log;
use serde::{Serialize, Deserialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name = "Nhat Vu";
    let my_object = MyObject {
        username: name.to_owned(),
        favorite_language: "Rust".to_owned(),
    };

    log!(name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());
    let class = "my_title";
    let message: Option<&str> = None;// Some("I'm a message");
    html! {
        <>
            <h1 class={class}>{"Hello World!!!"}</h1>
            if class == "my_titles" {
                <p>{"Hi there!"}</p>
            } else {
                <p>{"I'm not a titles"}</p>
            }

            if let Some(message) = message {
                <p>{message}</p>
            } else {
                <p>{"No messages to see today"}</p>
            }
        </>
    }
}