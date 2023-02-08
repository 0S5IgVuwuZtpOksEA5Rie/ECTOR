use rand;
use std::io::stdin;

macro_rules! input {
    ($prompt: block) => {
        println!($prompt);

        let mut string_input: String = String::new();

        stdin()
            .read_line(&mut string_input)
            .expect("input read error");

        string_input = String::from(string_input.trim());

        return string_input;
    };
}

fn get_direction() -> bool {
    loop {
        let string_input: String = input!("encrypt or decrypt?  (e/d)");

        if string_input == String::from("e") {
            return true;
        } else if string_input == String::from("d") {
            return false;
        } else {
            println!("invalid input");
        }
    }
}

fn get_rerun_or_close() -> bool {
    loop {
        println!("close?  (y/n)");

        let mut string_input: String = String::new();

        stdin()
            .read_line(&mut string_input)
            .expect("input read error");

        string_input = String::from(string_input.trim());

        if string_input == String::from("y") {
            return false;
        } else if string_input == String::from("n") {
            return true;
        } else {
            println!("invalid input");
        }
    }
}

fn read_user_setting() {}

fn generate_random_setting() {}

fn main() {
    println!("ECTOR EnCrypTOR");

    let mut run: bool = true;
    while run {
        let encrypt: bool = get_direction();

        if encrypt == true {
        } else {
        }

        run = get_rerun_or_close();
    }
}
