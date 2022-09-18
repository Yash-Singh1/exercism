use unicode_reverse::reverse_grapheme_clusters_in_place;

pub fn reverse(input: &str) -> String {
    // input.chars().rev().collect()
    let mut string = input.to_string().clone();
    reverse_grapheme_clusters_in_place(string.as_mut());
    string
}
