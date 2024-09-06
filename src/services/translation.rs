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
            "Language": "Language",
            "Welcome": "Welcome",
            "Exit": "Exit",
            "Save": "Save",
            "Login": "Login",
            "Logout": "Logout",
            "Username": "Username",
            "Password": "Password",
            "Username or password invalid" : "Username or password invalid",
            "Profile": "Profile",
        }),
    );

    translations.insert(
    	// English to Spanish
        Lang::ES.to_string(),
        serde_json::json!({
            "New Game": "Nuevo juego",
            "Language": "Idioma",
            "Welcome": "Bienvenido",
            "Save": "Guardar",
            "Exit": "Dejar",
            "Login": "Conectar",
            "Logout": "Desconectar",
            "Username": "Identificador",
            "Password": "Contraseña",
            "Username or password invalid" : "Identificador o contraseña inválidos",
            "Profile": "Perfil",
        }),
    );

    translations.insert(
    	// English to French
        Lang::FR.to_string(),
        serde_json::json!({
            "New Game": "Nouvelle Partie",
            "Language": "Langue",
            "Welcome": "Bienvenu",
            "Save": "Enregistrer",
            "Exit": "Quitter",
            "Login": "Connexion",
            "Logout": "Déconnexion",
            "Username": "Identifiant",
            "Password": "Mot de passe",
            "Username or password invalid" : "Identifiant ou mot de passe invalide",
            "Profile": "Profil",
        }),
    );

    translations.insert(
    	// English to German
        Lang::DE.to_string(),
        serde_json::json!({
            "New Game": "Neues Spiel",
            "Language": "Sprache",
            "Welcome": "Willkommen",
            "Save": "Speichern",
            "Exit": "Ausfahrt",
            "Login": "Einloggen",
            "Logout": "Ausloggen",
            "Username": "Identifikator",
            "Password": "Passwort",
            "Username or password invalid" : "Identifikator oder passwort ungültig",
            "Profile": "Profil",
        }),
    );
    
    translations
}