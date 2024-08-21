pub fn main(string_: String) -> Vec<i8> {
    // create a table for every ascii character that is 1 byte long
    let mut analysis: Vec<i8> = vec![0; 128];

    for character in string_.chars() {
        analysis[character.to_ascii_lowercase() as usize;] += 1;
    }
    return analysis;
}