# Train Ops

A train game for Android.

Made with Tauri and Yew.

## Installation 

* Tauri CLI : `cargo install tauri-cli --version "^2.0.0-rc"`
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

### UI

<div style="display:flex; flex-direction:column; width:10rem; color:black; border-radius:5px; text-align:center">
    <div style="background:#2b1c14;color:#ccc">#2b1c14</div>
    <div style="background:#5e7676">#5e7676</div>
    <div style="background:#e98554;">#e98554</div>
    <div style="background:#ffbe7d;">#ffbe7d</div>
    <div style="background:#fdf8d0;">#fdf8d0</div>
    <div style="background:#0a3631;color:#ccc">#0a3631</div>
    <div style="background:#175e3e;color:#ccc">#175e3e</div>
    <div style="background:#7bc572;">#7bc572</div>
</div>

### Map


<div style="display:flex; flex-direction:column; width:10rem; color:black; border-radius:5px; text-align:center">
    <div style="background:#99863a;">#99863a</div>
</div>
