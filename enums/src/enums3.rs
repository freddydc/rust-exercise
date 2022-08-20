// enums3.rs

pub fn run_enums3() {
    let mut state = State {
        quit: false,
        position: Point { x: 0, y: 0 },
        color: (0, 0, 0),
    };

    state.process(Message::ChangeColor((255, 0, 255)));
    state.process(Message::Echo(String::from("Good morning!")));
    state.process(Message::Move(Point { x: 10, y: 15 }));
    state.process(Message::Quit);

    println!("{:#?}", state);
}

enum Message {
    ChangeColor((u8, u8, u8)),
    Move(Point),
    Echo(String),
    Quit,
}

#[derive(Debug)]
struct Point {
    x: u8,
    y: u8,
}

#[derive(Debug)]
struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, m: String) {
        println!("{}", m);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        use Message::*;
        match message {
            ChangeColor((red, green, blue)) => self.change_color((red, green, blue)),
            Move(Point { x, y }) => self.move_position(Point { x, y }),
            Echo(msg) => self.echo(msg),
            Quit => self.quit(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
        };

        state.process(Message::ChangeColor((255, 0, 255)));
        state.process(Message::Echo(String::from("Good morning!")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }
}
