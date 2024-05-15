pub fn main() {
    let number = String::from("hello");
    let second_number = number;

    // here an error is thrown
    // println!("The value of number is: {number}");
    println!("The value of number is: {second_number}");


}

fn calculate_length(s: &String) -> usize {
    s.len()
}
