fn main() {
    let mut float_value: f32 = 0.0; // Example float value
    let mut int_value: i32 = 0; // first int variable to multiply with

    let max_val: i32 = 5000; // maximum value to calculate to.
    
    for _i in 1..=max_val {
        for _j in 1..=max_val {
            float_value += 1.0;
            int_value += 1;

            let int_multiplied_float = (int_value as f32) * float_value; // Multiply integer with float
            let int_float_to_int = int_value * (float_value as i32); // Multiply int with casted int to float
            let difference = int_multiplied_float - (int_float_to_int as f32); // Subtract the difference

            if difference != 0.0 {
                println!(
                    "Iteration: {}, int * float: {}, int as float * int as float: {}, Difference: {}",
                    _i,
                    int_multiplied_float,
                    int_float_to_int,
                    difference
                );
            }
            float_value = 0.0;
        }
    }
}
