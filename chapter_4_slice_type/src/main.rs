fn main() {
    let s = String::from("What is going on world?");
    let word_1a = first_word_without_slice(&s);
    // this also works as string slices are string literals
    let word_1b = first_word_without_slice(&s[..]);
    let word_2 = first_word_with_slice(&s);
    println!(
        "the index of the last letter of the first word in the sentence '{}' is:",
        s
    );
    println!("--> {}", word_1a);
    println!(
        "the index of the last letter of the first word in the sentence '{}' is:",
        s
    );
    println!("--> {}", word_1b);
    println!(
        "the index of the last letter of the first word in the sentence '{}' is:",
        s
    );
    println!("--> {}", word_2);
}

// function that finds the first word of a sentence
fn first_word_without_slice(s: &str) -> usize {
    let bytes = s.as_bytes();

    // here enumerate returns result as a tuple, index and item
    // (i, &item) deconstruct the tuple
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    //returns the index of the last letter of the first word
    s.len()
}

fn first_word_with_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
