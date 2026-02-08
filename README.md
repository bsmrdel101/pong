# Getting Started

Start by running the project with `cargo make run`

## Commands

- `cargo make build` -> Builds release binary
- `cargo make build-web` -> Builds wasm pkg in /engine
- `cargo make run` -> Run debug
- `cargo make run-prod` -> Run release

# Web Build

## Plain HTML

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Learn WGPU</title>
    <style>
      * {
        padding: 0;
        margin: 0;
      }
      canvas {
        background-color: black;
        width: 100%;
        height: 100%;
      }
    </style>
  </head>

  <body id="wasm-example">
    <canvas id="canvas"></canvas>
    <script type="module">
      import init from "./pkg/engine.js";
      init().then(() => {
        console.log("WASM Loaded");
      });
    </script>
  </body>
</html>
```

## Framework

```js
const init = await import('./pkg/game.js');
init().then(() => console.log("WASM Loaded"));
```
