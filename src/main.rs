use rand::{self, random};
use std::{self, io::stdin};

const STD_ALPHANUMERIC: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
const SPECIAL_CHARACTERS: &str = "`~!@#$%^&*()_-+=[]\\{{}|;:',.<>/?\"";

fn break_settings_string_to_tuple(settings_string: &str) -> (bool, u128, i128, i128, i128) {
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

fn get_rerun_or_close() -> bool {
    loop {
        let mut string_input: String = String::new();
        println!("do again (y/n)?");

        stdin()
            .read_line(&mut string_input)
            .expect("input read fail");

        let string_input_slice: &str = string_input.trim();

        if string_input_slice == "y" {
            return true;
        } else if string_input_slice == "n" {
            return false;
        } else {
            println!("invalid input")
        }
    }
}

fn generate_random_setting() -> (u128, i128, i128, i128) {
    // tuple structure:  (valid, key, key scaler, iterator scaler, position scaler)
    return (
        random::<u128>(),
        random::<i128>(),
        random::<i128>(),
        random::<i128>(),
    );
}

fn calculate_character_randomise(settings: (bool, u128, i128, i128, i128)) {}

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
                    let generated_random_setting: (u128, i128, i128, i128) =
                        generate_random_setting();
                    settings = (
                        true,
                        generated_random_setting.0,
                        generated_random_setting.1,
                        generated_random_setting.2,
                        generated_random_setting.3,
                    );
                    println!("{:?}", generated_random_setting);
                    break;
                } else {
                    let user_input_setting: (bool, u128, i128, i128, i128) =
                        break_settings_string_to_tuple(&string_input_slice);
                    if user_input_setting.0 == true {
                        settings = user_input_setting;
                        break;
                    }
                }
            }
        } else {
        }

        run = get_rerun_or_close()
    }
}
