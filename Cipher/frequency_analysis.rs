use std::collections::BTreeMap;
use std::iter::FromIterator;

pub fn main(string_: String) -> Vec<(String, i8)> {
    // create a table for every ascii character that is 1 byte long
    let mut analysis: BTreeMap<String, i8> = BTreeMap::new();

    for character in string_.chars() {
        let non_ascii = character.to_string();
        *analysis.entry(non_ascii).or_insert(0) += 1;
    }

    let mut v: Vec<(String, i8)> = Vec::from_iter(analysis);
    v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    return v;
}