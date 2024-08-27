pub fn get_from_analysis(analysis: Vec<(String, i32)>, idx: usize) -> String {
    // this is ugly
    let mut actual_index = idx.clone();
    if idx >= analysis.len() {
        actual_index = 0;
    }
    return (analysis[actual_index].0.parse::<i8>().unwrap() as u8 as char).to_string();
}