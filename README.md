mkdir wasm-calc
cargo component new calc --command
cd calc
cargo component build --release
wasmtime run target/wasm32-wasip1/release/calc.wasm

`Hello, world!`

When I use --command to create the main app, calc, no .wit files were created. But when I run

`cargo component new add --lib`

to create the add operator, world.wit file came out. Command also scaffolds some code solid with the .wit file, I change it accordigly to my .wit.

`cargo component build --release`

## 20250119
I think I mess solution by creating calc as wasi component but using as a rust application. By using it as rust application, basically using cargo run, I got this:
```
Hello, world!
thread 'main' panicked at src/main.rs:40:10:
Failed to instantiate component: component imports instance `wasi:cli/environment@0.2.0`, but a matching implementation was not found in the linker

Caused by:
    0: instance export `get-environment` has the wrong type
    1: function implementation is missing
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
Application panicked when calling function. Running it with wasmtime:
```
wasmtime run target/wasm32-wasip1/release/calc.wasm
```
gives no panick but only the `Hello, world!` output. That was because I did not rebuild the component, when I build it again I get back this error:
```
$ cargo component build --release
error: failed to create a target world for package `calc` (/home/fpezzati/workspace/wasm-calc/calc/Cargo.toml)

Caused by:
    0: failed to parse local target from directory `/home/fpezzati/workspace/wasm-calc/calc/wit`
    1: failed to start resolving path: /home/fpezzati/workspace/wasm-calc/calc/wit/operator.wit
    2: package identifier `calc:operator` does not match previous package name of `calc:main`
            --> /home/fpezzati/workspace/wasm-calc/calc/wit/operator.wit:1:9
             |
           1 | package calc:operator;
             |         ^------------

```
I think I mess components, which should be composed, with vanilla rust trying to load a component programmatically.

## 20250119
Still on the 'missing wasi:cli/environment' weird issue.

Important note: the bindgen! call is key to generate stubs accordignly to wit. I probably had to build as soon as I added the bindgen call and then add code to instantiate my component.
I did, driven by cut-n-paste, to add everything at once and I got confused by compile errors.

## 20250122
Got it! Key concept is that .wit files are some sort of abstractions, they don't specify all the required dependencies the
rust app needs to load the wasip2 component. So, I added `wasmtime_wasi` as cargo dependency, then I added
```
wasmtime_wasi::add_to_linker_sync(&mut linker);
```
in my host app. That does the trick passing wasmtime all the mandatory but untold wasip2 world shipped into the crate, allowing
me to successfully instantiate component.
