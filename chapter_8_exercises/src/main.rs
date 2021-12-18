use std::collections::HashMap;

fn main() {
    // EXERCISE 1
    let numbers = vec![1, 2, 100, 6, 5, 4, 7, 9, 8, 10, 3, 3, 3];

    // using average_num as a name otherwise if same name as function
    // the variable with name average will shadow the function with name average
    let average_num = average(&numbers);
    println!("the average i got from the vec is: {}", average_num);

    let median_num = median(&numbers);
    println!("the median i got from the vec is: {}", median_num);

    let mode_num = mode(&numbers);
    println!("the mode i got from the vec is: {}", mode_num);

    // EXERCISE 2
    let original_word = String::from("aSingapore");
    let pig_latin_version = convert_to_pig_latin(&original_word);
    println!("{} in pig-latin is {}", original_word, pig_latin_version);
}

fn convert_to_pig_latin(word: &str) -> String {
    let vowels = ["a", "e", "i", "o", "u"];
    // divide one string slice into two at an index
    let (first, rest) = word.split_at(1);
    // if vowel contains the first letter from word
    let is_vowel = vowels.contains(&first);
    // if it contains do this
    if is_vowel {
        return format!("{}-{}", word, "ah");
    }
    // if it does not contain then do that
    format!("{}-{}ay", rest, first)
}

fn average(numbers: &[i32]) -> f64 {
    // sum the numbers of a vector
    // devide by the length of the vector
    let mut sum = 0.0;
    for num in numbers {
        sum += *num as f64;
    }
    // .len() gives a usize type so we need to convert it to f64
    sum / numbers.len() as f64
}

fn median(numbers: &[i32]) -> f64 {
    // sort the vector and get the middle number
    // if the vec has an even length return the mean of the 2 middle numbers
    // to_vec copies the vec to a new vec
    let mut sorted_vec = numbers.to_vec();
    // sort the vec
    sorted_vec.sort();
    println!("the sorted vec is: {:?}", sorted_vec);
    // len returns a usize
    let middle = sorted_vec.len() / 2;
    // if sorted vec length no residual return the below
    if sorted_vec.len() % 2 == 0 {
        return average(&vec![sorted_vec[middle], sorted_vec[middle - 1]]);
    }
    // otherwise return the below
    sorted_vec[middle] as f64
}

fn mode(numbers: &[i32]) -> i32 {
    // count which element appears the most
    // declare a hashmap that will store the results
    let mut map = HashMap::new();
    // loop through the vec
    for num in numbers {
        // for each element if doesnt exist in hashmap
        // add that element to the hashmap and increase count by 1
        // count does not need to be mutable as it is a reference from mut map
        let count = map.entry(num).or_insert(0);
        // i do not need the reference but the value - so i dereference it
        *count += 1;
    }
    println!("Hashmap of occurances {:?}", map);
    let mut max_value = 0;
    let mut mode = 0;
    for (key, value) in map {
        if value > max_value {
            max_value = value;
            mode = *key;
        }
    }
    mode
}
