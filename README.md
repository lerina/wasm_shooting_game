# wasm_shooting_game

This is a Rust/Wasm version of the wonderful  
"How to Shoot Bullets in JavaScript - Game Dev" by Adam Alinauskas.  
[YouTube: Coding with Adam](https://www.youtube.com/watch?v=i7FzA4NavDs)

Given Rust borrow-checker "constraints" the code is inspired by the JavaScript version rather than a direct port.

The html and JavaScript is limited to this:

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>Rust wasm no bundle</title>
  </head>
  <body>
  <canvas id="canvas" style="outline: none" tabindex="0" height="600" width="600">
    Your browser does not support the Canvas.
  </canvas>
    <script type="module">
    import init from "./pkg/wasm_shooting_game.js";

      async function run() {
        await init();
      }

      run();
    </script>
  </body>
</html>
```
