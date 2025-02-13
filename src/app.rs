use leptos::*;
use leptos::ev::SubmitEvent;
use leptos::html::ElementChild;
use leptos::attr::global::{PropAttribute, GlobalAttributes};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, Debug)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct ChatArgs<'a> {
    message: &'a str,
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
fn ChatWindow() -> impl IntoView {
    let (messages, set_messages) = signal(Vec::<ChatMessage>::new());
    let (input, set_input) = signal(String::new());

    let send = move |_| {
        let msg = input.get();
        if msg.is_empty() { return; }

        let mut new_messages = messages.get();
        new_messages.push(ChatMessage {
            role: "user".into(),
            content: msg.clone(),
        });
        set_messages.set(new_messages.clone());
        set_input.set(String::new());

        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&ChatArgs { message: &msg }).unwrap();
            if let Some(response) = invoke("chat", args).await.as_string() {
                let mut msgs = messages.get();
                msgs.push(ChatMessage {
                    role: "assistant".into(),
                    content: response,
                });
                set_messages.set(msgs);
            }
        });
    };

    view! {
        <div class="chat-container">
            <h2>"Nexa Chat"</h2>
            <div class="messages">
                <For
                    each=move || messages.get()
                    key=|msg| msg.content.clone()
                    children=move |msg| view! {
                        <div class=format!("message {}", msg.role)>
                            {msg.content}
                        </div>
                    }
                />
            </div>
            <div class="input-area">
                <input
                    type="text"
                    placeholder="Type your message..."
                    prop:value=move || input.get()
                    on:input=move |ev| set_input.set(event_target_value(&ev))
                />
                <button on:click=send>"Send"</button>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    view! {
        <main class="container">
            <h1>"Welcome to Tauri + Leptos"</h1>

            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>

            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>

            <p>{ move || greet_msg.get() }</p>

            <ChatWindow/>
        </main>
    }
}
