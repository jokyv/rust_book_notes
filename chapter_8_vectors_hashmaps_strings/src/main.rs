use std::collections::HashMap;

fn main() {
    // VECTORS
    println!(">>>>>>>>>>>>>>>>>>>>>");
    println!("VECTORS");
    basic_vectors();
    add_values_to_vector();
    iterate_over_vector();
    vector_using_enum();

    // STRINGS
    println!(">>>>>>>>>>>>>>>>>>>>>");
    println!("STRINGS");
    basic_strings();
    combine_strings_with_plus();
    combine_strings_with_format();

    // HASH MAPS
    println!(">>>>>>>>>>>>>>>>>>>>>");
    println!("HASHMAPS");
    basic_hash_map();
    // create variable scores1 with what the below funciton returns
    let scores1 = vector_of_tuples_to_hashmap();
    iterate_hashmap(scores1);
}

// function takes a hashmap and iterates the keys, values and prints them
fn iterate_hashmap(scores: HashMap<String, i32>) {
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    println!("");
}

// create a hashmap from two vectors using zip
fn vector_of_tuples_to_hashmap() -> HashMap<String, i32> {
    let teams = vec![String::from("Blue"), String::from("Green")];
    let their_scores = vec![50, 95];
    // we can use <_,_> because rust will infer the type as <String, i32>
    let scores: HashMap<_, _> = teams.into_iter().zip(their_scores.into_iter()).collect();
    println!("construct scores hashmap with zip: {:?}", scores);
    println!("");
    // return the hashmap
    scores
}

// how to basic construct a hashmap
fn basic_hash_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Green"), 75);
    scores.insert(String::from("Red"), 45);
    println!("the hashmap scores is {:?}", scores);
    println!("");
}

fn combine_strings_with_format() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // format! does not take ownership
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);
    println!("")
}

// basic way to combine strings
fn combine_strings_with_plus() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("Singapore!");
    // we need one as a reference
    let s3 = s1 + &s2;
    println!("the s3 constructed with plus and is: {}", s3);
    println!("");
}

// basic way to construct strings
fn basic_strings() {
    let data = "initial contents";
    let s1 = data.to_string();
    // the above is equivalent to...
    let s2 = "initial contents".to_string();
    // also it can be...
    let s3 = String::from("initial contents");
    println!("printing basic strings {} {} {}", s1, s2, s3);
    println!("");
}

fn basic_vectors() {
    // explicity declaring the type
    let _v: Vec<i32> = Vec::new();
    // letting rust infer the type
    // rust infers the type from the data inside the vector
    let v_inferred = vec![1, 2, 3];
    println!("the v_inferred is: {:?}", v_inferred);
    // get the third place from the vector
    let third: &i32 = &v_inferred[2];
    println!("the third element of v_inferred is {}", third);
    println!("")
}

fn add_values_to_vector() {
    // add values to a vector
    let mut v_mutable = Vec::new();
    v_mutable.push(5);
    v_mutable.push(6);
    v_mutable.push(7);
    v_mutable.push(8);
    v_mutable.push(9);
    println!("the v_mutable is: {:?}", v_mutable);
    println!("")
}

// iterate over a vector and print its values
fn iterate_over_vector() {
    let v = vec![100, 50, 2];
    // use reference as we do not want to take ownership
    for i in &v {
        println!("element of the vector is {}", i);
    }
    println!("")
}

// vector can only store same type data
// use enum to store different types in a vector!
fn vector_using_enum() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // construct a vector from enum
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(5.67),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    println!("the vector from enum has values: {:?}", row);
    println!("")
}
