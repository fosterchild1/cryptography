const DICTIONARY: [char; 36] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn main(string_: String) -> Vec<i8> {
    let mut analysis: Vec<i8> = vec![0; 36];
    for character in string_.chars() {
        let lowercase_char = character.to_lowercase().next().unwrap();
        let index = if lowercase_char.is_ascii_alphabetic() {
            (lowercase_char as usize) - ('a' as usize)
        } else if lowercase_char.is_ascii_digit() {
            26 + (lowercase_char as usize) - ('0' as usize)
        } else {
            continue;
        };
        analysis[index] += 1;
    }
    return analysis;
}