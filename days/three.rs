const TARGET: i32 = 347991;

fn main() {
    let mut square = 1;
    let mut square_steps = 0;
    while square * square < TARGET {
        square += 2;
        square_steps += 1;
    }
    let diff = (square * square) - ((square-2) * (square-2));
    let segment = diff / 4;
    let offset = TARGET - ((square-2) * (square-2));
    let result = ((offset - (segment/2)) % segment) + square_steps;
    
    println!("Result: {}", result);
}
