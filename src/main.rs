use std::fs::File;
use std::io::Write;
use std::env;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Parse the command line arguments
    let mut a: i32 = 0;
    let mut b: f32 = 0.0;
    let mut max_val: i32 = 0;
    let arg_cnt: usize = args.len();

    println!("input two whole numbers to multiply and see if the produce correct result.");
    if arg_cnt == 1 {
        println!("Usage: {} <A> <B>", args[0]);
        println!("A * B = result.");
        println!("Usage: {} <maxVal>", args[0]);
        println!(
            "If maxVal (int) is provided then both A and B will iterate from 1 to the maxVal (10,000)"
        );
        println!(
            "If A and B are not provided then a predetermined Odd ball list will be calculated."
        );
    }

    if arg_cnt == 3 {
        a = args[1].parse().expect("Please provide a valid integer for A");
        b = args[2].parse().expect("Please provide a valid float for B");
    } else if arg_cnt == 2 {
        max_val = args[1].parse().expect("Please provide a valid float for maxVal");
    }

    if a != 0 && b != 0.0 {
        do_combinations(a, b);
    } else if a == 0 && b == 0.0 && max_val == 0 {
        calc_oddballs();
    } else {
        // Calculate the range of numbers iterating from 1 to 7000 for the integer and float values individually.
        let file_name = String::from("IntFloatError_Rust.csv");
        calc_number_range(max_val, &file_name);
    }
}

// Given a predetermined list of odd ball numbers do the combination calculations and display them to the console.
fn calc_oddballs() {
    println!("Calculating Odd Ball numbers...");
    println!("{}", produce_header());

    let lines: Vec<String> = calc_combinations(1683, 9971 as f32);
    display_lines(lines);
    let lines: Vec<String> = calc_combinations(2399, 6995 as f32);
    display_lines(lines);
    let lines: Vec<String> = calc_combinations(2401, 6997 as f32);
    display_lines(lines);
    let lines: Vec<String> = calc_combinations(2797, 5999 as f32);
    display_lines(lines);
    let lines: Vec<String> = calc_combinations(3055, 6111 as f32);
    display_lines(lines);
    let lines: Vec<String> = calc_combinations(3055, 6995 as f32);
    display_lines(lines);
}

// Given two numbers calculate the combinations and display them to the console.
fn do_combinations(int_value: i32, float_value: f32) {
    let lines = calc_combinations(int_value, float_value);
    println!("{}", produce_header());
    display_lines(lines);
}

// Given the calculated lines as strings display them to the console.
fn display_lines(lines: Vec<String>) {
    let mut iter: std::vec::IntoIter<String> = IntoIterator::into_iter(lines);
    while let Some(s) = iter.next() {
        println!("{}", s);
    }
}

// Given the two numbers to multiply calculate the combinations of the numbers, swapped, and negative values.
fn calc_combinations(int_value: i32, float_value: f32) -> Vec<String> {
    let num_combos: usize = 5;
    let mut lines: Vec<String> = vec![String::new(); num_combos];

    lines[0] = calc_multiplications(int_value, float_value);
    lines[1] = calc_multiplications(float_value as i32, int_value as f32);
    lines[2] = calc_multiplications(-int_value, -float_value);
    lines[3] = calc_multiplications(-int_value, float_value);
    lines[4] = calc_multiplications(int_value, -float_value);

    return lines;
}

// Given a max value (10,000) calculate the multiplications combinations by iterating from 1 to max value for both numbers.
fn calc_number_range(max_val: i32, file_name: &str) {
    println!("Calculating Int * Float for range from 1 to {} for both the integer (i32) and float(f32) values individually.", max_val);

    let mut file_output = File::create(file_name).expect("Creation of output file failed");
    let header = produce_header();

    file_output.write(header.as_bytes()).expect("Writing header line failed.");
    for _i in 1..=max_val {
        for _j in 1..=max_val {
            let output_lines: Vec<String> = calc_combinations(_i, _j as f32);

            if contains_error(&output_lines) {
                for each_line in output_lines {
                    file_output.write(each_line.as_bytes()).expect("Writing output line failed.");
                }
            }
        }
    }

    drop(file_output);
    println!("File: {} written!", file_name);
}

// Given the list of calculated lines check to see if there is an error in the calculation by checking if there
// is a 'true' value.
fn contains_error(output_lines: &Vec<String>) -> bool {
    let mut is_err: bool = false;
    for each_line in output_lines {
        is_err |= each_line.contains("true");
    }

    return is_err;
}

// Given the two numbers perform the multiplications between int, float, double, check if the value doesn't match the int * int value.
fn calc_multiplications(int_value: i32, float_value: f32) -> String {
    let int_multiplied_float: f32 = int_mul_float(int_value, float_value); // Multiply integer with float.
    let int_float_to_int: i32 = int_mul_int(int_value, float_value); // Multiply int with casted int to float.
    let float_mul_float: f32 = float_mul_float(int_value, float_value); // Multiply int as float to float.
    let difference: f32 = int_substract_float(int_float_to_int, int_multiplied_float); // Subtract the difference.
    let is_err: bool = int_float_to_int.to_string() != int_multiplied_float.to_string();
    let int_mul_dbl = int_mul_double(int_value, float_value); // Multiply Int with double.
    let dbl_multiplied_dbl: f64 = dbl_mul_double(int_value, float_value); // Multiply Double to Double.

    let output_line = format!(
        "{},{},{},{},{},{},{},{},{}",
        int_value,
        float_value,
        int_multiplied_float,
        difference,
        float_mul_float,
        int_float_to_int,
        is_err,
        int_mul_dbl,
        dbl_multiplied_dbl
    );

    return output_line;
}

fn produce_header() -> String {
    return String::from(
        "A~I,B~F,A~I * B~F,Diff(int*float - int*int),A~F * B~F,A~I * B~I,Is Error,A~I * B~Dbl,A~Dbl * B~Dbl"
    );
}

fn int_mul_float(int_val: i32, float_val: f32) -> f32 {
    return (int_val as f32) * float_val;
}

fn int_mul_int(int_val: i32, float_val: f32) -> i32 {
    return int_val * (float_val as i32);
}

fn float_mul_float(int_val: i32, float_val: f32) -> f32 {
    return (int_val as f32) * float_val;
}

fn int_mul_double(int_val: i32, float_val: f32) -> f64 {
    return (int_val as f64) * (float_val as f64);
}

fn dbl_mul_double(int_val: i32, float_val: f32) -> f64 {
    return (int_val as f64) * (float_val as f64);
}

fn int_substract_float(int_val: i32, float_val: f32) -> f32 {
    return (int_val as f32) - float_val;
}
