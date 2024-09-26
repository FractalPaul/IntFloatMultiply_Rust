use std::fs::File;
use std::io::Write;

fn main() {
    calc_oddballs();

    // Calculate the range of numbers iterating from 1 to 7000 for the integer and float values individually.
    let file_name = String::from("IntFloatError_Rust.csv");
    calc_number_range(10000, &file_name);
}

fn calc_oddballs() {
    println!("{}", produce_header());

    println!("{}", calc_multiplications(1683, 9971 as f32));
    println!("{}", calc_multiplications(2399, 6995 as f32));
    println!("{}", calc_multiplications(2401, 6997 as f32));
    println!("{}", calc_multiplications(2797, 5999 as f32));
    println!("{}", calc_multiplications(3055, 6111 as f32));
    println!("{}", calc_multiplications(3055, 6995 as f32));
}

fn calc_number_range(max_val: i32, file_name: &str) {
    println!("Calculating Int * Float for range from 1 to {} for both the integer (i32) and float(f32) values individually.", max_val);

    let mut file_output = File::create(file_name).expect("Creation of output file failed");
    let header = produce_header();

    file_output.write(header.as_bytes()).expect("Writing header line failed.");
    for _i in 1..=max_val {
        for _j in 1..=max_val {
            let output_line: String = calc_multiplications(_i, _j as f32);

            if output_line.contains("true") {
                file_output.write(output_line.as_bytes()).expect("Writing output line failed.");
            }
        }
    }

    drop(file_output);
    println!("File: {} written!", file_name);
}

fn calc_multiplications(int_value: i32, float_value: f32) -> String {
    let int_multiplied_float: f32 = int_mul_float(int_value, float_value); // Multiply integer with float.
    let int_float_to_int: i32 = int_mul_int(int_value, float_value); // Multiply int with casted int to float.
    let float_mul_float: f32 = float_mul_float(int_value, float_value); // Multiply int as float to float.
    let difference: f32 = int_substract_float(int_float_to_int, int_multiplied_float); // Subtract the difference.
    let is_err: bool = int_float_to_int.to_string() != int_multiplied_float.to_string();
    let int_mul_dbl = int_mul_double(int_value, float_value); // Multiply Int with double.
    let dbl_multiplied_dbl: f64 = dbl_mul_double(int_value, float_value); // Multiply Double to Double.

    let output_line = format!(
        "{},{},{},{},{},{},{},{},{}\n",
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
        "A~I,B~F,A~I * B~F,Diff(int*float - int*int),A~F * B~F,A~I * B~I,Is Error,A~I * B~Dbl,A~Dbl * B~Dbl\n"
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
