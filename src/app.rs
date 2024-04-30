use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(command: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct CommandArgs<'a> {
    param: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let run_local = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }
            let args = to_value(&CommandArgs { param: &name }).unwrap();
            let result = invoke("run_local", args).await;
            logging::log!("result from leptos: {:?}", result);

            //set_greet_msg.set(result["stdout"]);
        });
    };

    view! {
        <main class="container">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>

            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <p>
                "Recommended IDE setup: "
                <a href="https://code.visualstudio.com/" target="_blank">"VS Code"</a>
                " + "
                <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">"Tauri"</a>
                " + "
                <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">"rust-analyzer"</a>
            </p>

            <form class="row" on:submit=run_local>
                <input
                    id="greet-input"
                    placeholder="Enter a command..."
                    autocorrect="off"
                    autocapitalize="off"
                    autocomplete="off"
                    on:input=update_name
                />
                <button type="submit">"Run Local"</button>
            </form>
            // <form class="row"
            //     on:submit=run_local>
            //     <input
            //         id="cmd-input"
            //         autocorrect="off"
            //         autocapitalize="off"
            //         autocomplete="off"
            //         placeholder="Enter command..."
            //         on:input=update_cmd
            //     />
            //     <button type="submit">"Run Local"</button>
            // </form>
            <p><b>{ move || greet_msg.get() }</b></p>
        </main>
    }
}
