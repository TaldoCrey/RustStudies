
use List::{Cons, Nil};
fn main() {
    let b = Box::new(5); // Box Smart Pointer
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

}

enum List {
    Cons(i32, Box<List>), //Because of Box smart pointer fixed size, recursive enums, that usually causes trouble 
    Nil,                  // Can be implemented
}


