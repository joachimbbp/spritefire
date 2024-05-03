# Spritefire

Use spritefire cli using

```
cargo run -p spritefire --release write <assets-path> <db-path>
cargo run -p spritefire --release read <db-path> <img-path>
cargo run -p spritefire --release debug <db-path>
```

To run the web version, you need to install wasm-pack and then do

```
cd spritefire_web
wasm-pack build --target web
```

Then host a server inside the spritefire_web directory. You can use the built in python server with

```
python3 -m http.server
```
