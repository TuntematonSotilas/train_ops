use yew::prelude::*;
use yewdux::prelude::*;
use crate::states::app_state::AppState;

#[function_component(App)]
pub fn app() -> Html {
    let (ostate, dispatch) = use_store::<AppState>();
    
    let onclick = dispatch.reduce_mut_callback(|state| state.is_paused = !state.is_paused);

    html! {
        <main class="container">
         
            <button onclick={onclick}>{"Pause"}</button>
      
            <p><b>{ &ostate.is_paused }</b></p>
        </main>
    }
}
