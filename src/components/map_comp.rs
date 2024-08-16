
use yew::prelude::*;
use yewdux::prelude::*;

use crate::states::map_state::{MapState, Tile};

#[function_component(MapComp)]
pub fn map() -> Html {

    let (state, dispatch) = use_store::<MapState>();

    if !state.is_init {
        let tiles = vec![Tile::default(); 25];
        let mut ntiles = Vec::<Tile>::new();
        for (i, tile) in &mut tiles.clone().iter_mut().enumerate() {
            tile.index = i;
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
                                <div class={classes!(
                                        "tile",
                                        tile.is_rail.then_some("tile--rail"))}
                                    onclick={
                                        let tile_click = tile_click.clone();
                                        let tile = *tile;
                                        move |_| tile_click.emit(tile.index)}>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                 </div>
            </div>
        </div>
    }
}