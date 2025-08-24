fn main() {
    let rect = Rectangle { width: 10, heigh: 15};
    dbg!(&rect);

}

#[derive(Debug)] //Aqui estamos fazendo o struct Rectangle derivar o traço Debug
struct Rectangle {
    width: i32,
    heigh: i32,
}
