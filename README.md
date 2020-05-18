# ttype

A quick crate that creates a macro to print or return the type of a variable.

Right now this depends on the unstable fetaures `type_name_of_val()` so it will require both Rust Nightly and the `#![feature(type_name_of_val)]` attribute for your debugging needs.

Example:

```rust
#![feature(type_name_of_val)]
#[macro_use]
use ttype::ptype;

fn main() {
    let x = "Hello, world";

    //prints &str
    ptype!(&x);
}
```
