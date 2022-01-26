# PWA Shell Generator

Generates the scaffolding for a Progressive Web App (PWA).
By default a BodeJS based dev setup is included.

# Getting started

A local installation of Rust is needed to build the generator.
Then consult `--help` for pointers.

The default icon is taken from:
https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Progressive_Web_Apps_Logo.svg/1920px-Progressive_Web_Apps_Logo.svg.png

# Generated folder structure

```
"name"
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
|-- .gitignore (optional)
|-- package.json (optional)
|-- gulpfile.js (optional)
|-- README.md
```