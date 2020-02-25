extern crate dep;

use dep::ExampleDep;

fn main() {
    ExampleDep::some_fn(0);
    println!("Hello, world!");
}
