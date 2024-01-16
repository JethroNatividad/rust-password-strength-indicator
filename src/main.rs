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

enum PasswordStrength {
    VeryWeak = 0,
    Weak = 1,
    Strong = 2,
    VeryStrong = 3,
}

fn get_password_strength(password: &str) -> PasswordStrength {
    // make multiple strings of number, letter, and special.
    // make bool variables has_number, has_letter, has_special
    // make a frequency counter for the characters in the password
    // loop through the keys of the counter.
    // if the key contains in number, make has_number true
    // if the key contains in letter, make has_letter true
    // if the key contains in special, make has_special true

    // if has_number is true and has_letter is false and has_special is false, return VeryWeak
    // if has_number is false and has_letter is true and has_special is false, return Weak
    // if has_number is true and has_letter is true and has_special is false, return Strong
    // if has_number is true and has_letter is true and has_special is true, return VeryStrong
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

fn main() {
    println!("Hello, world!");
}
