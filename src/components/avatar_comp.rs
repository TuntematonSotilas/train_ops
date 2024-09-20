
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::use_store;

use crate::{enums::{avatar::Avatar, route::Route, storage_keys::StorageKey}, services::storage, states::app_state::AppState};

#[function_component(AvatarComp)]
pub fn avatar() -> Html {

    let navigator = use_navigator().unwrap();
    
    let (state, dispatch) = use_store::<AppState>();
        
    let avatar_click = Callback::from(move |avatar: &Avatar| {
        let user_opt = state.user.clone();
        if let Some(mut user) = user_opt {
            user.avatar = avatar.clone();
            // Save the User in the state
            let user_st = user.clone();
            dispatch.reduce_mut(|state| state.user = Some(user_st));
            // Save the User in local storage
            let user_sto = user.clone();
            storage::save(StorageKey::User, user_sto.to_json().as_str());
            // Save the Avatar in the database
            // Redirect
            navigator.push(&Route::Profil)
        }
        
    });

    html! {
        <div class="container">
            <div class="row">
                <h1>{ "Avatar" }</h1>
            </div>
            
            {
                Avatar::iterator().map(|avatar|
                {
                    html!{
                        <div class="row">
                            <div class="avatar" onclick={
                                let avatar_click = avatar_click.clone();
                                move |_| avatar_click.emit(&avatar)}>
                                    <img class="avatar__img" src={format!("/public/img/avatars/{0}.png", avatar.to_str())}/>
                            </div>
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}