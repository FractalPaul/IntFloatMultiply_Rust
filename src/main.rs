use std::fs::File;
use std::io::Write;

fn main() {
    let file_name = String::from("IntFloatError_Rust.csv");
    calc_number_range(7000, &file_name);
}

fn calc_number_range(max_val: i32, file_name: &str) {
    println!("Calculating Int * Float for range from 1 to {}", max_val);

    let mut float_value: f32 = 0.0; // Example float value
    let mut int_value: i32 = 0; // first int variable to multiply with
    let mut file_output = File::create(file_name).expect("Creation of output file failed");

    file_output
        .write(
            "Iter,Int,Float,Int x Float,Int x Int(from float),Difference, Double x Double\n".as_bytes()
        )
        .expect("Writing header line failed.");
    for _i in 1..=max_val {
        int_value += 1;
        for _j in 1..=max_val {
            float_value += 1.0; // 1.0;

            let int_multiplied_float = (int_value as f32) * float_value; // Multiply integer with float.
            let int_float_to_int = int_value * (float_value as i32); // Multiply int with casted int to float.
            let difference = (int_multiplied_float as i32) - int_float_to_int; // Subtract the difference.

            if difference != 0 || int_multiplied_float.to_string() != int_float_to_int.to_string() {
                let dbl_multiplied_dbl: f64 = (int_value as f64) * (float_value as f64); // Multiply Double to Double.
                // println!(
                //     "Iteration: {}, {}(int) * {}(float) = {}(int as float), int * float as int = {}, Difference: {}",
                //     _i,
                //     int_value,
                //     float_value,
                //     int_multiplied_float,
                //     int_float_to_int,
                //     difference
                // );

                let output_line = format!(
                    "#{},{},{},{},{},{},{}\n",
                    _i,
                    int_value,
                    float_value,
                    int_multiplied_float,
                    int_float_to_int,
                    difference,
                    dbl_multiplied_dbl
                );

                file_output
                    .write(output_line.as_bytes())
                    .expect("Writing output line failed.");
            }
        }
        float_value = 0.0;
    }

    drop(file_output);
    println!("File: {} written!", file_name);
}
