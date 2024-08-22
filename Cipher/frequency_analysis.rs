use std::collections::HashMap;

pub fn main(string_: String) -> HashMap<String, i8> {
    // create a table for every ascii character that is 1 byte long
    let mut analysis: HashMap<String, i8> = HashMap::new();

    for character in string_.chars() {
        let non_ascii = character.to_string();
        let get = analysis.get(&non_ascii);
        
        let count = if get == None {
            0
        }
        else {
            *get.unwrap() as i8
        };
        analysis.insert(non_ascii, count + 1);
    }
    return analysis;
}