fn main(){
    // add_two_numbers(1, 2);
    println!("Your guess is: {}", add_two_numbers(2, 2));
}
fn add_two_numbers(x: i32, y: i32) -> i32 {
    x + y
}