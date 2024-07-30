use serde_json::*;
use std::collections::HashMap;

pub fn get_translation() -> HashMap<String, Value> {
    let mut translations = HashMap::new();

    translations.insert(
    	// EN to EN
        "en".to_string(),
        serde_json::json!({
            "New Game": "New Game",
            "Settings": "Settings",
            "Save": "Save",
            "Exit": "Exit",
        }),
    );

    translations.insert(
    	// EN to ES
        "es".to_string(),
        serde_json::json!({
            "New Game": "Nuevo juego",
            "Settings": "Ajustes",
            "Save": "Guardar",
            "Exit": "Dejar",
        }),
    );

    translations.insert(
    	// EN to FR
        "fr".to_string(),
        serde_json::json!({
            "New Game": "Nouvelle Partie",
            "Settings": "Paramètres",
            "Save": "Enregistrer",
            "Exit": "Quitter",
        }),
    );

    translations.insert(
    	// EN to DE
        "de".to_string(),
        serde_json::json!({
            "New Game": "Neues Spiel",
            "Settings": "Einstellungen",
            "Save": "Speichern",
            "Exit": "Ausfahrt",
        }),
    );
    
    translations
}