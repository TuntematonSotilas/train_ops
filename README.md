# Train Ops

A train game for Android.

Made with Tauri and Yew.

## Installation 

* Tauri CLI : `cargo install tauri-cli --version "^2.0.0"`
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
* Create the file `key.properties` in the folder `src-tauri/gen/android/`
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
    <div style="background:#2b1c14;color:#ccc">brown</div>
    <div style="background:#5e7676">grey</div>
    <div style="background:#3e4949;color:#ccc">dark-grey</div>
    <div style="background:#e98554;">dark-orange</div>
    <div style="background:#ffbe7d;">orange</div>
    <div style="background:#fdf8d0;">beige</div>
    <div style="background:#0a3631;color:#ccc">dark-green</div>
    <div style="background:#175e3e;color:#ccc">green</div>
    <div style="background:#7bc572;">light-green</div>
</div>

### Map

<div style="display:flex; flex-direction:column; width:10rem; color:black; border-radius:5px; text-align:center">
    <div style="background:#99863a;">dirt</div>
    <div style="background:#785f28;color:#ccc">dark-dirt</div>
    <div style="background:#2027ff;color:#ccc">sea</div>
    <div style="background:#0000b0;color:#ccc">dark-sea</div>
</div>

### Sprites

[SimCity-2000](https://www.spriters-resource.com/ms_dos/simcity2000/)