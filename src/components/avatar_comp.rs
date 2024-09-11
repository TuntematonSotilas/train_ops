
use yew::prelude::*;
use yew_router::prelude::*;

use crate::enums::{avatar::Avatar, route::Route};

#[function_component(AvatarComp)]
pub fn avatar() -> Html {

    let navigator = use_navigator().unwrap();
    
    let avatar_click = Callback::from(move |avatar: &Avatar| {
        // Save the Avatar in the state
        //dispatch.reduce_mut(|state| state.lang = lang.clone());
        // Save the Avatar in local storage
        //storage::save(StorageKey::Lang, lang.to_str());
        // Save the Avatar in the database
        // Redirect
        navigator.push(&Route::Profil)
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