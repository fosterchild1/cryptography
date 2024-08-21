const alphabet: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
/*const alphabet_iter: Iterator<Item = Vec> = alphabet.iter();*/

pub fn main(string_: String) -> Vec<i8> {
    let split: Vec<char> = string_.chars().collect();
    let mut analysis: Vec<i8> = vec![0; 26];
    for character in split {
        let index = (character.to_ascii_lowercase() as usize) - ('a' as usize);
        analysis[index] += 1
    }
    return analysis
}