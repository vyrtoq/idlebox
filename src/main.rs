use std::io;
fn main() {

    let mut input = String::new();
    println!("Please input the number of the element of the Fibonacci sequence you want to see: ");
    io::stdin().read_line( &mut input )
        .expect( "Failed to read line" );

    let input: u32 = match input.trim().parse(){
        Ok(num) => num,
        Err(_) => 0,
    };
    
    let ordinal: &str = match input {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    };
    println!("The {}{} element of the Fibonacci sequence is: {}", input, ordinal, fibonacci(input));

}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}