#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Add;

impl Guest for Add {
    fn compute(x: i32, y: i32) -> i32 {
        x + y
    }

    fn name() -> String {
        "Add".to_string()
    }
}

bindings::export!(Add with_types_in bindings);
