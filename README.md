# Tauri + Yew

This template should help get you started developing with Tauri and Yew.

## Installation 

* Tauri CLI : `cargo install tauri-cli --version '^2.0.0-beta'`
* Trunk : `cargo install trunk` 

Others prerequisites : 
[Tauri](https://tauri.app/v1/guides/getting-started/prerequisites/)

## Recommended IDE Setup

VSCode extensions : 
 
* [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) 
* [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
* [rust-yew](https://marketplace.visualstudio.com/items?itemName=TechTheAwesome.rust-yew) 

## Mobile config 

https://v2.tauri.app/fr/start/prerequisites/#configuration-pour-mobile


## Run 

cargo tauri dev

## Android 

* Init : `cargo tauri android init`
* Run : `cargo tauri android dev`
* Build : `cargo tauri android build`

## Sign the APK

* Generate the JKS file : [generate-key](https://developer.android.com/studio/publish/app-signing?hl=fr#generate-key)
* Copy the JKS file in the folder `src-tauri/gen/android/app`
* Create the file `key.properties` in the folder `src-tauri/gen/android/app`
```
storePassword=
keyPassword=
keyAlias=
storeFile=
```
* Update the file `src-tauri/gen/android/app/build.gradle.kts` : 
```
val keyProperties = Properties().apply {
    val propFile = file("key.properties")
    if (propFile.exists()) {
        propFile.inputStream().use { load(it) }
    }
}
signingConfigs {
    create("release") {
        keyAlias = keyProperties.getProperty("keyAlias")
        keyPassword = keyProperties.getProperty("keyPassword")
        storeFile = file(keyProperties.getProperty("storeFile"))
        storePassword = keyProperties.getProperty("storePassword")
    }
}
buildTypes {
        getByName("release") {
            signingConfig = signingConfigs.getByName("release")
            ...
        }
}
```