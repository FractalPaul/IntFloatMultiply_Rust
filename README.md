# IntFloatMultiply_Rust
Rust Int multiply Float to check for any errors in calculations.

Linux:
```terminal
$ rustc main.rs
$ ./main
```
```terminal
$ cargo build
$ cargo run
```

## Two number calculations..
To perform the combination of multiplications on just two numbers provide the two numbers as arguments:
```terminal
$ cargo run 3055 6111
```
## Range calculations...
To perform the range calculation just put a Max Value as a single argument.
```terminal
$ cargo run 7000
```
This will calculate the combinations of multiplications for both numbers ranging from 1 to Max Value

## Odd Ball Calculations...
To perform the Odd ball calculation execute without any parameters.
```terminal
$ cargo run
```
The Odd Ball calculation is a predetermined set of numbers that cause an error.  These are set to compare to other platforms and languages such as C or C#.
Odd ball calulations...
![IntFloat Odd Ball Rust](https://github.com/user-attachments/assets/d7641035-5ebf-41ac-858a-81bb95b4d33d)
