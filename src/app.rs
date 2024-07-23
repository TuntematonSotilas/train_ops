use yew::prelude::*;
use yew_router::prelude::*;
use yew_i18n::I18nProvider;

use crate::enums::route::Route;
use crate::components::{menu_comp::MenuComp, game_comp::GameComp, setting_comp::SettingComp};
use crate::services::translation::get_translation;

#[function_component(App)]
pub fn app() -> Html {

    let supported_languages = vec!["en", "es", "fr"];
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
        Route::Menu => html! { <MenuComp/> },
        Route::Game => html! { <GameComp/> },
        Route::Setting => html! { <SettingComp/> },
    }
}
