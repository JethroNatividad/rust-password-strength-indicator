use std::collections::HashMap;
use std::io;
use std::io::Write;

// Write a program that determines a passwords strength
// • A very weak password contains only numbers and is
// fewer than eight characters.
// • A weak password contains only letters and is fewerthan
// eight characters.
// • A strong password contains letters and at least one
// number and is at least eight characters.
// • A very strong password contains letters, numbers, and
// special characters and is at least eight characters.

// Inputs: password
// Process: checks password strength
// Outputs: The password '{}' is a {very weak | weak | strong | very strong} password.

// The password '12345' is a very weak password.
// The password 'abcdef' is a weak password.
// The password 'abc123xyz' is a strong password.
// The password '1337h@xor!' is a very strong password.

#[derive(PartialEq, Debug)]
enum PasswordStrength {
    VeryWeak = 0,
    Weak = 1,
    Strong = 2,
    VeryStrong = 3,
}

fn get_password_strength(password: &str) -> PasswordStrength {
    // make multiple strings of number, letter, and special.
    const NUMBERS: &str = "0123456789";
    const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const SPECIAL_CHARACTERS: &str = "!@#$%^&*()_-+=[]{}|;:'\",.<>?/";
    // make bool variables has_number, has_letter, has_special
    let mut has_number: bool = false;
    let mut has_letter: bool = false;
    let mut has_special_characters: bool = false;
    // make a frequency counter for the characters in the password
    let mut password_chars: HashMap<String, i64> = HashMap::new();
    for c in password.chars() {
        *password_chars.entry(c.to_string()).or_insert(0) += 1;
    }
    // loop through the keys of the counter.
    for key in password_chars.keys() {
        // if the key contains in number, make has_number true
        if NUMBERS.contains(key) {
            has_number = true;
        }
        // if the key contains in letter, make has_letter true
        if LETTERS.contains(key) {
            has_letter = true;
        }
        // if the key contains in special, make has_special true
        if SPECIAL_CHARACTERS.contains(key) {
            has_special_characters = true;
        }
    }

    // A very strong password contains letters, numbers, and special characters and is at least eight characters.
    if has_number & has_letter & has_special_characters & (password.len() > 7) {
        return PasswordStrength::VeryStrong;
    }
    // A strong password contains letters and at least one number and is at least eight characters.
    else if has_number & has_letter & (password.len() > 7) {
        return PasswordStrength::Strong;
    }
    // A weak password contains only letters and is fewer than eight characters
    else if has_letter {
        return PasswordStrength::Weak;
    }
    // A very weak password contains only numbers and is fewer than eight characters.
    else {
        return PasswordStrength::VeryWeak;
    }
}

#[cfg(test)]
mod tests {
    use super::{get_password_strength, PasswordStrength};

    #[test]
    fn test_get_password_strength() {
        assert_eq!(get_password_strength("12345"), PasswordStrength::VeryWeak);
        assert_eq!(get_password_strength("abcdef"), PasswordStrength::Weak);
        assert_eq!(get_password_strength("abc123xyz"), PasswordStrength::Strong);
        assert_eq!(
            get_password_strength("1337h@xor!"),
            PasswordStrength::VeryStrong
        );
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    // get input_password, "Enter Password: "
    let input_password: String = get_input("Enter Password: ");
    // get password strength
    let password_strength: &str = match get_password_strength(&input_password) {
        PasswordStrength::Strong => "strong",
        PasswordStrength::VeryStrong => "very strong",
        PasswordStrength::Weak => "weak",
        PasswordStrength::VeryWeak => "very weak",
    };
    // The password '{}' is a {very weak | weak | strong | very strong} password.
    println!(
        "The password '{}' is a {} password.",
        input_password, password_strength
    );
}
