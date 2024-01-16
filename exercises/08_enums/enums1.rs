// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Quit,
    Echo(i32),
    Move,
    ChangeColor(ColorTupleStruct),
}

#[derive(Debug)]
struct ColorTupleStruct(u8, u8, u8);

fn main() {
    let green = ColorTupleStruct(0, 255, 0);

    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(42));
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor(green));
}
