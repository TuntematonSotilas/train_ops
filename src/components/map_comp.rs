
use yew::prelude::*;
use yewdux::prelude::*;

use crate::states::map_state::{MapState, Tile};

#[function_component(MapComp)]
pub fn map() -> Html {

    let (state, dispatch) = use_store::<MapState>();

    if !state.is_init {
        
        log::info!("use_effect");
        let tiles = vec![Tile::default(); 25];
        let mut index = 0;
        let mut ntiles = Vec::<Tile>::new();
        for tile in &mut tiles.clone() {
            tile.index = index;
            index += 1;
            ntiles.push(*tile);
        }
        dispatch.reduce_mut(|map| map.tiles = ntiles);
        dispatch.reduce_mut(|map: &mut MapState| map.is_init = true);
    }

    let tile_click = Callback::from(move |index: usize| {
        dispatch.reduce_mut(|map| map.tiles[index].is_rail = true);
    });


    html! {
        <div class="map">
            <div class="wrapper">
                <div class="map-ctn">
                    { 
                        state.tiles.iter().map(|tile| {
                            html!{
                                <div class={classes!("tile",
                                        tile.is_rail.then(|| Some("tile__rail")))}
                                    onclick={
                                        let tile_click = tile_click.clone();
                                        let tile = tile.clone();
                                        move |_| tile_click.emit(tile.index)}> 

                                        {tile.index}
                                        {tile.is_rail}
                                </div>
                            }
                        }).collect::<Html>()
                    }
                 </div>
            </div>
        </div>
    }
}