// enums2.rs

#[derive(Debug)]
enum Message {
    Move { x: i32, y: i32 },
    ChangeColor(u8, u8, u8),
    Echo(String),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

pub fn run_enums2() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("Hello!")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }

    if let Message::Move { x, y } = &messages[0] {
        println!("Move x: {}", x);
        println!("Move y: {}", y);
    }
}
