# PWA Shell Generator

Generate the skeleton for a progressive web app (PWA).

# API

initiPWA name [imagePath] [options]

## "name"
Name and folder of the new PWA (folder must not exist).

## "imagePath"
Path to image to be used as icons (will be resized accordingly).
A defaultimage is used if not provided.

The default icon is taken from:
https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Progressive_Web_Apps_Logo.svg/1920px-Progressive_Web_Apps_Logo.svg.png

## Options:
- wakelock: bool, default: false
- orientation: [
    any (default)
    natural
    landscape
    landscape-primary
    landscape-secondary
    portrait
    portrait-primary
    portrait-secondary
]
- bg_color: String, default: "#000"
- theme_color: String, default: "#fff"
- desc: String, default: A generated pwa

## Example call

`initPWA foo-bar ./icon.png --wakelock theme_color:green, bg_color: blue, desc:"foo bar", orientation:portrait`

# Generated folder structure:

```
"name"
|-- docs
|-- src
|  |-- images
|    |-- icon-192.png
|    |-- icon-512.png
|  |-- js
|    |-- service-worker-init.js
|  |-- index.css
|  |-- index.html
|  |-- index.js
|  |-- manifest.json
|  |-- service-worker.js
|-- .gitignore
|-- README.md
```