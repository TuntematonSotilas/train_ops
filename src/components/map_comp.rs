
use yew::prelude::*;
use yewdux::prelude::*;

use crate::states::map_state::{MapState, Tile};

const BOARD_LEGHT: usize = 20;

#[function_component(MapComp)]
pub fn map() -> Html {

    let (state, dispatch) = use_store::<MapState>();
    let state_tile = state.clone();

    if !state.is_init {
        let nb_tiles = BOARD_LEGHT * BOARD_LEGHT;
        let tiles = vec![Tile::default(); nb_tiles];
        let mut ntiles = Vec::<Tile>::new();
        for (i, tile) in &mut tiles.clone().iter_mut().enumerate() {
            tile.index = i;
            ntiles.push(*tile);
        }
        dispatch.reduce_mut(|map| map.tiles = ntiles);
        dispatch.reduce_mut(|map: &mut MapState| map.is_init = true);
    }

    let tile_click = Callback::from(move |index: usize| {
        if state_tile.is_build_mode {
            dispatch.reduce_mut(|map| map.tiles[index].is_rail = true);
        }
    });


    html! {
        <div class="map map__height">
            <div class="map__rotate">
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
    }
}