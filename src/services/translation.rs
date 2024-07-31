use serde_json::*;
use std::collections::HashMap;

use crate::enums::lang::Lang;

pub fn get_translation() -> HashMap<String, Value> {
    let mut translations = HashMap::new();

    translations.insert(
    	// EN to EN
        Lang::EN.to_string(),
        serde_json::json!({
            "New Game": "New Game",
            "Settings": "Settings",
            "Save": "Save",
            "Exit": "Exit",
        }),
    );

    translations.insert(
    	// EN to ES
        Lang::ES.to_string(),
        serde_json::json!({
            "New Game": "Nuevo juego",
            "Settings": "Ajustes",
            "Save": "Guardar",
            "Exit": "Dejar",
        }),
    );

    translations.insert(
    	// EN to FR
        Lang::FR.to_string(),
        serde_json::json!({
            "New Game": "Nouvelle Partie",
            "Settings": "Paramètres",
            "Save": "Enregistrer",
            "Exit": "Quitter",
        }),
    );

    translations.insert(
    	// EN to DE
        Lang::DE.to_string(),
        serde_json::json!({
            "New Game": "Neues Spiel",
            "Settings": "Einstellungen",
            "Save": "Speichern",
            "Exit": "Ausfahrt",
        }),
    );
    
    translations
}