use levenstein_spam::levenshtein_distance;

use std::io;

fn main() {
    // Read input text
    let mut input_text = String::new();
    println!("Enter a long text:");
    io::stdin().read_line(&mut input_text).expect("Failed to read line");

    // Trim whitespaces
    let input_text = input_text.trim();

    // Example comparison
    let s1 = "kitten";
    let distance = levenshtein_distance(s1, input_text);
    println!("Levenshtein distance between '{}' and entered text is: {}", s1, distance);
}
