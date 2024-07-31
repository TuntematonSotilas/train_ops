# Train Ops

A train game for Android.

Made with Tauri and Yew.

## Installation 

* Tauri CLI : `cargo install tauri-cli --version '^2.0.0-beta'`
* Trunk : `cargo install trunk` 
* Others prerequisites : [tauri-prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites/)

VSCode extensions : 
 
* [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) 
* [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
* [rust-yew](https://marketplace.visualstudio.com/items?itemName=TechTheAwesome.rust-yew) 

Android config : [tauri-android](https://v2.tauri.app/start/prerequisites/#android)

## Run 

cargo tauri dev

## Android 

* Init : `cargo tauri android init`
* Run : `cargo tauri android dev`
* Build : `cargo tauri android build`

## Sign the APK

* Generate the JKS file : [generate-key](https://developer.android.com/studio/publish/app-signing?hl=fr#generate-key)
* Copy the `key.jks` file in the folder `src-tauri/gen/android/app`
* Create the file `key.properties` in the folder `src-tauri/gen/android/app`
```
storePassword=
keyPassword=
keyAlias=key
storeFile=key.jks
```

## Generate icons 
`cargo tauri icon <path>` 

## Color Palette 

* #2b1c14
* #5e7676
* #e98554
* #ffbe7d
* #fdf8d0
* #074d45
* #258158
* #7bc572

| code  | color                                           |
| :---- |:------------------------------------------------|
| #2b1c14  | <span style="color:#2b1c14">&#9724;</span> |
| #F00  | <span style="color:rgb(200,0,0)">&#9724;</span> |