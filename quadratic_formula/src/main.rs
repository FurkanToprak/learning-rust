use std::io;

fn main() {
    let read_error = "Failed to read line.";
    let number_error = "Number expected.";
    let mut a_input = String::new();
    let mut b_input = String::new();
    let mut c_input = String::new();
    // Input message
    println!("Quadratic Formula Calculator");
    println!("-----------------------------");
    println!("Enter Coefficients (A*x^2 + B*x + C = 0):");
    println!("A = ");
    io::stdin()
        .read_line(&mut a_input)
        .expect(read_error);
    println!("B = ");
    io::stdin()
        .read_line(&mut b_input)
        .expect(read_error);
    println!("C = ");
    io::stdin()
        .read_line(&mut c_input)
        .expect(read_error);
    let a: f32 = a_input.trim().parse().expect(number_error);
    let b: f32 = b_input.trim().parse().expect(number_error);
    let c: f32 = c_input.trim().parse().expect(number_error);
    println!("{}*x^2 + {}*x + {} = 0", a_input, b_input, c_input);
    let solution_space_check = b.pow(2) - 4 * a * c;
    let solution1 = (-b + sqrt(solution_space_check) / (2 * a)
    let solution2 = (-b - sqrt(solution_space_check) / (2 * a)
}
