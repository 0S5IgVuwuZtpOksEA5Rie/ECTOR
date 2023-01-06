use rand::{self, random};
use std::{self, io::stdin};

fn break_settings_string_to_array(settings_string: &str) -> (bool, u128, i128, i128, i128) {
    let settings_string_split: Vec<&str> = settings_string.split(";").collect();

    if settings_string_split.len() != 4 {
        return (false, 0, 0, 0, 0);
    }

    for i in 0..settings_string_split.len() {
        if settings_string_split[i].parse::<u128>().is_err()
            || settings_string_split[i].parse::<i128>().is_err()
        {
            return (false, 0, 0, 0, 0);
        }
    }

    return (
        true,
        settings_string_split[0].parse::<u128>().unwrap(),
        settings_string_split[1].parse::<i128>().unwrap(),
        settings_string_split[2].parse::<i128>().unwrap(),
        settings_string_split[3].parse::<i128>().unwrap(),
    );
}

fn generate_random_setting() -> (u128, i128, i128, i128) {
    let key: u128 = random();
    let key_scaler: i128 = random();
    let iterator_scaler: i128 = random();
    let position_scaler: i128 = random();

    return (key, key_scaler, iterator_scaler, position_scaler);
}

fn main() {
    println!("ECTOR (EnCrypTOR)");

    // main loop
    let mut run: bool = true;
    while run {
        let mut string_input: String = String::new();
        // encrypt or decrypt loop
        loop {
            println!("encrypt (e), decrypt (d):");
            stdin()
                .read_line(&mut string_input)
                .expect("input read fail");

            let string_input_slice: &str = string_input.trim();

            if string_input_slice == "e" || string_input_slice == "d" {
                break;
            }
            string_input = "".to_string();
            println!("invalid input");
        }

        let direction: &str = string_input.trim();

        // if to encrypt
        if direction == "e" {
            let mut settings: (bool, u128, i128, i128, i128);
            // get settings loop
            loop {
                let mut string_input: String = String::new();
                println!("insert settings, type 'r' for random settings:");
                stdin()
                    .read_line(&mut string_input)
                    .expect("input read fail");

                let string_input_slice: &str = string_input.trim();

                if string_input_slice == "r" {
                    let tmp_1: (u128, i128, i128, i128) = generate_random_setting();
                    settings = (true, tmp_1.0, tmp_1.1, tmp_1.2, tmp_1.3);
                    println!("{:?}", tmp_1);
                    break;
                } else {
                    let tmp_1: (bool, u128, i128, i128, i128) =
                        break_settings_string_to_array(&string_input_slice);
                    if tmp_1.0 == true {
                        settings = tmp_1;
                        break;
                    }
                }
            }
        } else {
        }

        // do again loop
        loop {
            println!("do again (y/n)?");
            let mut string_input: String = String::new();

            stdin()
                .read_line(&mut string_input)
                .expect("input read fail");

            let string_input_slice: &str = string_input.trim();

            if string_input_slice == "y" {
                break;
            } else if string_input_slice == "n" {
                run = false;
                break;
            } else {
                println!("invalid input")
            }
        }
    }
}
