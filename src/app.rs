use yew::prelude::*;
use yew_router::prelude::*;
use yew_i18n::I18nProvider;

use crate::enums::lang::Lang;
use crate::enums::route::Route;
use crate::components::{game_comp::GameComp, lang_comp::LangComp, login_comp::LoginComp};
use crate::services::translation::get_translation;

#[function_component(App)]
pub fn app() -> Html {

    let supported_languages = vec![
        Lang::EN.to_str(), 
        Lang::ES.to_str(), 
        Lang::FR.to_str(), 
        Lang::DE.to_str()
    ];
    
    let translations = get_translation();
    
    html! {
        <I18nProvider supported_languages={supported_languages} translations={translations} >
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </I18nProvider>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Login => html! { <LoginComp/> },
        Route::Game => html! { <GameComp/> },
        Route::Lang => html! { <LangComp/> },
    }
}
