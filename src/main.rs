use levenstein_spam::get_brands;
use levenstein_spam::levenshtein_distance;

use std::io;

fn main() {
    // Read input text
    let mut input_text = String::new();
    println!("Enter a long text:");
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");

    // Trim whitespaces
    for word in input_text.split_whitespace() {
        for brand in get_brands() {
            let distance = levenshtein_distance(brand, word);

            if distance <= 2 && distance != 0 {
                println!("{} could be a spam attack on {}", word, brand);
            }
        }
    }

    // Example comparison
}
