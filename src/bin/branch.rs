mod private {
    pub struct Initial {
        _x: (),
    }

    impl Initial {
        pub fn new() -> Self {
            Self { _x: () }
        }

        pub fn to_text(self, text: String) -> Text {
            Text { text }
        }

        pub fn to_number(self, num: u32) -> Number {
            Number { num }
        }
    }

    pub struct Number {
        num: u32,
    }

    impl Number {
        pub fn num(&self) -> u32 {
            self.num
        }

        pub fn print(&self) {
            println!("NUMBER: {}", self.num);
        }
    }

    pub struct Text {
        text: String,
    }

    impl Text {
        pub fn print(&self) {
            println!("TEXT: {}", self.text);
        }
    }
}
use std::io::Write;

use private::{Initial, Number, Text};

fn main() {
    // Compile time known flow: great!
    let one = Initial::new();
    let two: Text = one.to_text("hello".to_string());
    two.print();

    let one = Initial::new();
    let two: Number = one.to_number(1234);
    two.print();

    // runtime, we need to handle the control flow ourselves
    let one = Initial::new();
    let mut user_input = String::new();
    print!("(branching 1) Type something:\n> ");
    let _ = std::io::stdout().lock().flush();
    let _ = std::io::stdin().read_line(&mut user_input).unwrap();

    match user_input.trim().parse::<u32>() {
        Ok(num) => {
            let two = one.to_number(num);
            two.print();
        },
        Err(_) => {
            let two = one.to_text(user_input.clone());
            two.print();
        },
    }

    //////////////////////////////////////////////////////////////////////////
    // You CAN'T do this!
    //

    // let two = match user_input.trim().parse::<u32>() {
    //     Ok(num) => one.to_number(num),
    //     Err(_) => one.to_text(user_input),
    // };
    // two.print();

    //
    //////////////////////////////////////////////////////////////////////////

    // You could do this:
    //
    // But now we've defeated the whole point of type states!
    enum Either {
        Number(Number),
        Text(Text),
    }

    impl Either {
        fn print(&self) {
            match self {
                Either::Number(n) => n.print(),
                Either::Text(t) => t.print(),
            }
        }
    }

    let one = Initial::new();
    let mut user_input = String::new();
    print!("(branching 2) Type something:\n> ");
    let _ = std::io::stdout().lock().flush();
    let _ = std::io::stdin().read_line(&mut user_input).unwrap();

    let two: Either = match user_input.trim().parse::<u32>() {
        Ok(num) => Either::Number(one.to_number(num)),
        Err(_) => Either::Text(one.to_text(user_input)),
    };
    two.print();

    //////////////////////////////////////////////////////////////////////////
    // You CAN'T do this!
    //

    // let mut state = Initial::new().to_number(0);
    // loop {
    //     let mut user_input = String::new();
    //     state.print();
    //     print!("Type something:\n> ");
    //     let _ = std::io::stdout().lock().flush();
    //     let _ = std::io::stdin().read_line(&mut user_input).unwrap();

    //     let one = Initial::new();
    //     let parse = user_input.trim().parse::<u32>();
    //     let two = match (parse, &state) {
    //         (Ok(num), Number { num: oldnum }) => one.to_number(num.wrapping_add(oldnum)),
    //         (Ok(num), Text { .. }) => one.to_number(num),
    //         (Err(_), _) => one.to_text(user_input),
    //     };
    //     state = two;
    // }

    //
    //////////////////////////////////////////////////////////////////////////

    // Instead you must do this!
    let mut state = Either::Number(Initial::new().to_number(0));
    loop {
        let mut user_input = String::new();
        print!("(looping) ");
        state.print();
        print!("(looping) Type something:\n> ");
        let _ = std::io::stdout().lock().flush();
        let _ = std::io::stdin().read_line(&mut user_input).unwrap();

        let one = Initial::new();
        let parse = user_input.trim().parse::<u32>();
        let two = match (parse, &state) {
            (Ok(num), Either::Number(oldnum)) => Either::Number(one.to_number(oldnum.num().wrapping_add(num))),
            (Ok(num), Either::Text(Text { .. })) => Either::Number(one.to_number(num)),
            (Err(_), _) => Either::Text(one.to_text(user_input)),
        };
        state = two;
    }
}
