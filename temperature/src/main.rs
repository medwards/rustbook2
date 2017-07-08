// TODO:
// * don't keep calling env:args(), figure out how to keep ownership from moving
// * using match properly for units (maybe an enum?)
// * add Kelvin
use std::env;

fn main() {
    if env::args().count() != 3 {
        print_help();
        return;
    }

    let degrees: i32 = match env::args().nth(1).unwrap().trim().parse() {
        Ok(num) => num,
        Err(_) => {
            print_help();
            return;
        }
    };

    let unit = env::args().nth(2).unwrap().to_lowercase();

    if unit == "celcius" || unit == "c" {
        println!("{}°C is {}°F", degrees, celcius_to_fahrenheit(degrees));
    } else if unit == "fahrenheit" || unit == "f" {
        println!("{}°C is {}°F", degrees, fahrenheit_to_celcius(degrees));
    } else {
        print_help();
        return;
    }
}

fn celcius_to_fahrenheit(degrees: i32) -> i32 {
    let mut degrees = degrees as f32;
    degrees = (degrees * (9.0 / 5.0)) + 32.0;
    degrees as i32
}

fn fahrenheit_to_celcius(degrees: i32) -> i32 {
    let mut degrees = degrees as f32;
    degrees = (degrees - 32.0) * (5.0 / 9.0);
    degrees as i32
}

fn print_help() {
    println!("Usage: temperature DEGREES UNITS");
    println!("DEGREES is the temperature, UNITS is the temperature unit of DEGREES");
    println!("UNITS must be one of Celcius or Fahrenheit");
}
