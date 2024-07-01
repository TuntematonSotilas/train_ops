use std::rc::Rc;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yewdux::use_store;
use crate::stores::app_store::AppStore;
use yewdux::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let dispatch = use_dispatch::<AppStore>();
    let state: Rc<AppStore> = dispatch.get();

    //let onclick = dispatch.reduce_mut_callback(|is_paused| state.is_paused);

    html! {
        <main class="container">
         
            <button>{"Pause"}</button>
      
            <p><b>{ &state.is_paused }</b></p>
        </main>
    }
}
