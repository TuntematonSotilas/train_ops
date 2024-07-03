
use yew::prelude::*;

#[function_component(MapComp)]
pub fn map() -> Html {

    let tiles = (1..=25).collect::<Vec<_>>();

    html! {
        <div class="map">
            <div class="wrapper">
                <div class="map-ctn">
                    { 
                        tiles.iter().map(|_| { 
                            html!{
                                <div class="tile"></div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>
        </div>
    }
}