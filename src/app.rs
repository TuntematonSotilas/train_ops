use yew::prelude::*;
use yew_router::prelude::*;
use yew_i18n::I18nProvider;

use serde_json::*;
use std::collections::HashMap;

use crate::enums::route::Route;
use crate::components::{menu_comp::MenuComp, game_comp::GameComp, setting_comp::SettingComp};

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

fn get_translation() -> HashMap<String, Value> {
    let mut translations = HashMap::new();

    translations.insert(
    	// EN to EN
        "en".to_string(),
        serde_json::json!({
            "New Game": "New Game",
        }),
    );

    translations.insert(
    	// EN to FR
        "fr".to_string(),
        serde_json::json!({
            "New Game": "Nouvelle Partie",
        }),
    );

    translations.insert(
    	// EN to ES
        "es".to_string(),
        serde_json::json!({
            "New Game": "Nuevo juego",
        }),
    );

    log::info!("{:?}", translations);

    translations

}