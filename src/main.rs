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

fn get_password_strength(password: &str) -> PasswordStrength {}
#[cfg(test)]
mod tests {
    use super::*;

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
