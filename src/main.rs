use fancy_regex::Regex;
use levenstein_spam::get_brands;
use levenstein_spam::levenshtein_distance;

use std::io;

fn main() {
    let mut input_text = String::new();
    println!("Enter Email: ");
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");

    let re = Regex::new(
        r"[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)",
    )
    .unwrap();

    let mut domain_bodies = Vec::new();
    for mat in re.find_iter(input_text.as_str()) {
        domain_bodies.push(mat.unwrap().as_str());
    }

    let top_level =
        Regex::new(r"(?:[^.]+)\.(gov\.uk|ac\.uk|co\.uk|com|org|net|us|uk|ch|eu|de)(?:\/|$)")
            .unwrap();

    let mut domains = Vec::new();
    for domain_body in domain_bodies {
        for mat in top_level.find_iter(domain_body) {
            let sld = mat.as_ref().unwrap().as_str().split(".").next().unwrap();
            domains.push(sld);
        }
    }

    for sld in domains {
        for brand in get_brands() {
            let distance = levenshtein_distance(brand, sld);

            if distance <= 2 && distance != 0 {
                println!("{} could be a spam attack on {}", sld, brand);
            }
        }
    }
}
