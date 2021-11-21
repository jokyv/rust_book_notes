fn main() {
    // how to declare const variables
    const MAX_POINTS: u32 = 100_000;
    println!("the value of const MAX_POINTS is: {}", MAX_POINTS);
    // x is mutable now
    let mut x = 5;
    println!("the value of x is: {}", x);
    // x can be changed because x is mutable
    x = 6;
    println!("can mutate x because we said mut x");
    println!("the value of x is: {}", x);
    // can also do shadowing, does not require mut
    let shadow_x = 5;
    let shadow_x = shadow_x + 1;
    let shadow_x = shadow_x * 2;
    println!("the value of shadow_x is: {}", shadow_x);
    // integer types
    let can_not_be_negative_x: u32 = 5;
    println!(
        "the value of can_not_be_negative_x is: {}",
        can_not_be_negative_x
    );
    let can_be_negative_x: i32 = -5;
    println!("the value of can_be_negative_x is: {}", can_be_negative_x);
    // tuples
    // tuples have fixed length
    // tuples can have different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructure a tuple
    let (tup_1, tup_2, tup_3) = tup;
    println!("the values of tup are: {}, {}, {}", tup_1, tup_2, tup_3);
    // or can use a period followed by the index
    println!(
        "print tup using index instead: {}, {}, {}",
        tup.0, tup.1, tup.2
    );
    // array
    // array can only have a signle type
    // use underscope in front ot the variable if you do not want to use it
    let _months = ["January", "February", "March", "April"];
    // access arry elements of an array using indexing
    // _months[0] or _months[1]

    // functions
    // functions without semin colon at the end called statements
    fn plus_one(x: i32) -> i32 {
        x + 1
    }
    // assign to a variable what function is going to give
    let number_five = plus_one(4);
    println!("the number_five variable got the value of: {}", number_five);

    // if expressions
    fn greater_than_five(x: i32) {
        if x > 5 {
            println!("number {} is greater than five", x);
        } else {
            println!("number {} is not greater than five", x);
        }
    }
    // will print that is not greater than five
    greater_than_five(4);

    // using if in a let statement
    let num = 5;
    let is_positive = if num > 0 { true } else { false };
    println!("the value of is_positive is: {}", is_positive);

    // this does not work!
    // better with matching!
    // fn greater_than_five_using_matching(num: u32) {
    //     match num {
    //         num % 4 == 0 => println!("number is divisible by 4"),
    //         num % 2 == 0 => println!("number is divisible by 2"),
    //         num % 3 == 0 => println!("number is divisible by 3"),
    //         _ => println!("number is NOT divisible by 2,3,4"),
    //     };
    // }
    // greater_than_five_using_matching(50);

    // loops
    // infinite loop
    // loop {// do something here}
    // count to ten
    fn count_to_ten() {
        let mut counter = 0;
        loop {
            counter += 1;
            println!("print counter: {}", counter);
            if counter == 10 {
                break;
            }
        }
    }
    count_to_ten();
    // countdown using while loop
    fn countdown_from(mut num: u32) {
        while num != 0 {
            println!("{}!", num);
            num -= 1;
        }
        println!("we are LIFTING OFF!!")
    }
    // countdown starting from 7
    countdown_from(7);
    // countdown using for loop
    fn countdown_from_using_for_loop() {
        // rev will reverse the order
        for number in (1..=5).rev() {
            println!("{}!", number)
        }
        println!("we are LIFTING OFF!!")
    }
    // countdown starting from 5
    countdown_from_using_for_loop();
}
