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
            "Welcome": "Welcome",
            "Exit": "Exit",
            "Save": "Save",
            "Login": "Login",
            "Logout": "Logout",
            "Username": "Username",
            "Password": "Password",
            "Username or password invalid" : "Username or password invalid",
        }),
    );

    translations.insert(
    	// English to Spanish
        Lang::ES.to_string(),
        serde_json::json!({
            "New Game": "Nuevo juego",
            "Settings": "Ajustes",
            "Welcome": "Bienvenido",
            "Save": "Guardar",
            "Exit": "Dejar",
            "Login": "Conectar",
            "Logout": "Desconectar",
            "Username": "Identificador",
            "Password": "Contraseña",
            "Username or password invalid" : "Identificador o contraseña inválidos",
        }),
    );

    translations.insert(
    	// English to French
        Lang::FR.to_string(),
        serde_json::json!({
            "New Game": "Nouvelle Partie",
            "Settings": "Paramètres",
            "Welcome": "Bienvenu",
            "Save": "Enregistrer",
            "Exit": "Quitter",
            "Login": "Connexion",
            "Logout": "Déconnexion",
            "Username": "Identifiant",
            "Password": "Mot de passe",
            "Username or password invalid" : "Identifiant ou mot de passe invalide",
        }),
    );

    translations.insert(
    	// English to German
        Lang::DE.to_string(),
        serde_json::json!({
            "New Game": "Neues Spiel",
            "Settings": "Einstellungen",
            "Welcome": "Willkommen",
            "Save": "Speichern",
            "Exit": "Ausfahrt",
            "Login": "Einloggen",
            "Logout": "Ausloggen",
            "Username": "Identifikator",
            "Password": "Passwort",
            "Username or password invalid" : "Identifikator oder passwort ungültig",
        }),
    );
    
    translations
}