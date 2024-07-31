use serde_json::*;
use std::collections::HashMap;

use crate::enums::lang::Lang;

pub fn get_translation() -> HashMap<String, Value> {
    let mut translations = HashMap::new();

    translations.insert(
    	// English to English
        Lang::EN.to_string(),
        serde_json::json!({
            "New Game": "New Game",
            "Settings": "Settings",
            "Save": "Save",
            "Exit": "Exit",
            "Connect": "Connect",
            "Username": "Username",
            "Password": "Password",
        }),
    );

    translations.insert(
    	// English to Spanish
        Lang::ES.to_string(),
        serde_json::json!({
            "New Game": "Nuevo juego",
            "Settings": "Ajustes",
            "Save": "Guardar",
            "Exit": "Dejar",
            "Connect": "Conectar",
            "Username": "Identificador",
            "Password": "Contraseña",
        }),
    );

    translations.insert(
    	// English to French
        Lang::FR.to_string(),
        serde_json::json!({
            "New Game": "Nouvelle Partie",
            "Settings": "Paramètres",
            "Save": "Enregistrer",
            "Exit": "Quitter",
            "Connect": "Connexion",
            "Username": "Identifiant",
            "Password": "Mot de passe",
        }),
    );

    translations.insert(
    	// English to German
        Lang::DE.to_string(),
        serde_json::json!({
            "New Game": "Neues Spiel",
            "Settings": "Einstellungen",
            "Save": "Speichern",
            "Exit": "Ausfahrt",
            "Connect": "Verbinden",
            "Username": "Identifikator",
            "Password": "Passwort",
        }),
    );
    
    translations
}