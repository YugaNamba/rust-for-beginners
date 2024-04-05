#[derive(Debug)]
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

impl Message {
    fn show(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let mut message = Message::Quit;
    message.show();
    message = Message::ChangeColor(0, 160, 255);
    message.show();
    message = Message::Move { x: 10, y: 30 };
    message.show();
    message = Message::Write("Hello!".to_string());
    message.show();
}
