use std::io;

fn main() {
    println!("Please input a temperature...");

    // declare variable temperature as string
    let mut temperature = String::new();

    // read from the terminal
    io::stdin()
        // everything you type is stored in a reference of mutable var temperature
        .read_line(&mut temperature)
        .expect("Failed to real line");

    // get themeprature and unit
    let (temperature, unit) = get_temperature_from_string(temperature.trim());
    // if unit is celsius
    if unit == "celsius" {
        println!(
            "{}C is equal to {}F",
            temperature,
            // covnert celsius to fahrenheit
            celsius_to_fahrenheit(temperature)
        );
    // if unit is fahrenheit
    } else if unit == "fahrenheit" {
        println!(
            "{}F is euqal to {}C",
            temperature,
            // convert fahrenheit to celsius
            fahrenheit_to_celsius(temperature)
        );
    }
}

fn get_temperature_from_string(temp_string: &str) -> (f64, &str) {
    // declate all variables
    let temp_string_bytes = temp_string.as_bytes();
    let mut temperature: &str = "";
    let mut unit = "fahrenheit";

    for (i, &item) in temp_string_bytes.iter().enumerate() {
        // if you find F or f
        if item == b'F' || item == b'f' {
            // store only the number
            temperature = &temp_string[0..i];
            break;
        // if you find C or c
        } else if item == b'C' || item == b'c' {
            // store only the number
            temperature = &temp_string[0..i];
            unit = "celsius";
            break;
        }
    }

    // make temperature a f64 from string
    let temperature: f64 = match temperature.parse() {
        Ok(num) => num,
        Err(_) => panic!("invalid temperature"),
    };

    // return the temperature as number and what unit is it
    return (temperature, unit);
}

// convert fahrenheit to celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0)
}

// convert celsius to fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0) / 5.0 + 32.0
}
