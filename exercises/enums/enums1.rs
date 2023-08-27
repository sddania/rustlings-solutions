// enums1.rs
//
// No hints this time! ;)
 
#[derive(Debug)]
enum Message {
    // define a few types of messages as used below
    Quit = 0,
    Echo = 2,
    Move = 4,
    ChangeColor = 8,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
