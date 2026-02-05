Simulate key press and mouse movement from your rust code.
note this version is only working for mac users.

crate : https://crates.io/crates/Janlib

How to use,

step 1 import the library in your Cargo.toml
```rust
[dependencies]
<<<<<<< HEAD
Janlib = "0.1.2" //latest
=======
Janlib = "0.1.0"
>>>>>>> origin/main
```
step 2 inside create JanKeymap obj and import the library
```rust
use Janlib::*;

let mut action: JanKeymap = Janlib::JanKeymap::new();
```
Here some feature - Docs and Quick Start
```rust
fn main() {

    let mut action: JanKeymap = Janlib::JanKeymap::new();

    // press keyboard touch 'a'
    action.KeyboardEvent(KeyJob::A);

    //press keyboard touch 'a' from a string
    action.KeyboardEvent(KeyJob::from_string("A".to_string()));

    //press keyboard touch based on the string order
    action.KeyboardStringSequance("Abcd".to_string());

    //change the cursor pos to x 100.0 y 200.0
    action.MouseMovement(MouseJobPos::Pos(100.0, 200.0));


    // Position loop: each vec contains ((x, y), duration before next position)
    event.MouseMovement(vec![
        vec![((400.0, 422.9), Duration::from_micros(100))], // top-left, wait 100µs
        vec![((600.0, 422.9), Duration::from_micros(4000))], // top-right, wait 4000µs
        vec![((600.0, 622.9), Duration::from_micros(2000))], // bottom-right, wait 2000µs
        vec![((400.0, 622.9), Duration::from_micros(5000))], // bottom-left, wait 5000µs
        vec![((400.0, 422.9), Duration::from_micros(100))],  // back to top-left, wait 100µs
    ]);

    // Single move without delay
    event.MouseMovement(vec![vec![((800.0, 111.0), Duration::from_millis(0))]]);

    //more feature coming soon
}
```
