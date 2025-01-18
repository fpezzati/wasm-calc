mkdir wasm-calc
cargo component new calc --command
cd calc
cargo component build --release
wasmtime run target/wasm32-wasip1/release/calc.wasm
---
Hello, world!
---
When I use --command to create the main app, calc, no .wit files were created. But when I run
---
cargo component new add --lib
___
to create the add operator, world.wit file came out. Command also scaffolds some code solid with the .wit file, I change it accordigly to my .wit.

cargo component build --release
