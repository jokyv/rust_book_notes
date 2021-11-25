fn main() {
    // this is stored on the stack and is immutable
    let _s = "hello";
    // new_s can be mutable now, stored in the heap
    let mut new_s = String::from("Hello");
    // proof that it can be modified ;)
    new_s.push_str(", this is John");
    println!("our string is: {}", new_s);

    // do some stuff here with s as is in scope

    // y and x have the same value because they are allocated in the stack
    // in this case y is a copy of x
    // this is valid
    let x = 5;
    let _y = x;
    // but you can not do the following because s1 is allocated in the heap
    // that is because different variables pointing to the same location
    let s1: String = String::from("Hello");
    let s2 = s1;
    // s1 is gone and has been replaced by s2
    // you can not print s1
    println!(
        "the s1 value is gone here: can only print s2 which is: {}",
        s2
    );
    // however you can use clone in order to overcome this
    let s1: String = String::from("new_hello");
    let s2 = s1.clone();
    println!("can print s1 and s2 because s2 is the clone of s1");
    println!("s1 = {}, s2 = {}", s1, s2);

    // onwership and functions
    let s = String::from("s_comes_to_scope_again"); // s comes into scope
    takes_ownwership(s); // s moves into function
                         // s is not valid here, as nothing is returned from the function
    let x = 5; // x comes into scope
    makes_copy(x); // x is moved into the function
                   // i32 is copy so it can still be used here
                   // no problem with primatives
    println!("you can use x here and its value is: {}", x);

    let s1 = gives_ownership();
    // s2 comes into scope
    let s2 = String::from("hello world this is s2");
    // now you can print s3 because takes_and_gives_back function returns something
    // here s3 has become s2 because this is what the function does
    let s3 = takes_and_gives_back(s2);
    // note that you can not print s2
    println!("s1: {}, s3: {}", s1, s3);

    // here the function takes ownership of s3
    let (s4, len) = calculate_length_takes_ownership(s3);
    println!("the length of '{}' is {}", s4, len);

    // reference allow you to refer to some value without taking ownership
    // the below function borrowed s4 and automatically give it back (no need to return it)
    let len_of_s4 = calculate_length_with_reference(&s4);
    // function has not got ownership of s4 so you can still use s4
    println!("the length of '{}' is {}", s4, len_of_s4);

    // at any given time, you can have either one mutable reference
    // or any number of immutable references
    // references must always be valid
}

// s is not longer valid here, goes out of scope

// supporting functions
fn takes_ownwership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// this function will move its return value into
// the function that calls it
fn gives_ownership() -> String {
    // some_string comes into scope
    let some_string = String::from("hello world");
    // it returns some_string and moves out
    // to the calling function
    some_string
}

// will take a string as a variable
// will return the same string back
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// function that calculates the length of a string
fn calculate_length_takes_ownership(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// same as above function but this time takes a reference instead
fn calculate_length_with_reference(s: &String) -> usize {
    s.len()
}
