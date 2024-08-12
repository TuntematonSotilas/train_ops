
use yew::prelude::*;
use crate::components::{user_comp::UserComp, map_comp::MapComp};

#[function_component(GameComp)]
pub fn game() -> Html {

    html! {
        <div>
            <UserComp/>
            <MapComp/> 
        </div>
    }
}