use rand::random;
use std::io::stdin;

fn input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut string_input: String = String::new();

    stdin()
        .read_line(&mut string_input)
        .expect("input read error");

    string_input = String::from(string_input.trim());

    return string_input;
}

// True:  encrypt,  False:  decrypt
fn get_direction() -> bool {
    loop {
        let string_input: String = input("encrypt or decrypt?  (e/d)");

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
        let string_input: String = input("close?  (y/n)");

        if string_input == String::from("y") {
            return false;
        } else if string_input == String::from("n") {
            return true;
        } else {
            println!("invalid input");
        }
    }
}

fn interpret_user_setting() {}

fn read_user_setting(direction: bool) -> i128 {
    loop {
        if direction == true {
            print!("type \"g\" for generated settings");
        }
        let input: String = input("insert settings:");

        if direction == true {
            if input == String::from("r") {
                return generate_random_setting();
            } else {
                return interpret_user_setting();
            }
        } else {
            return interpret_user_setting();
        }

        println!("invalid input");
    }
}

fn generate_random_setting() -> i128 {
    return random::<i128>();
}

fn main() {
    println!("ECTOR EnCrypTOR");

    let mut run: bool = true;
    while run {
        let encrypt: bool = get_direction();

        if encrypt == true {
            let key: i128 = read_user_setting(true);
        } else {
            let key: i128 = read_user_setting(false);
        }

        run = get_rerun_or_close();
    }
}
