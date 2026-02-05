Simulate key press and mouse movement from your rust code.
note this version is only working for mac users.

How to use,

step 1 import the library in your Cargo.toml
```rust
[dependencies]
<<<<<<< HEAD
Janlib = "0.1.1" //latest
=======
Janlib = "0.1.0"
>>>>>>> origin/main
```
step 2 inside create JanKeymap obj and import the library
```rust
use Janlib::*;

let mut action: JanKeymap = Janlib::JanKeymap::new();
```
Here some feature
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

    //more feature coming soon
}
```
