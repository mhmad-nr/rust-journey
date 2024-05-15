pub fn main() {
    let number = String::from("hello");
    let second_number = number;

    // here an error is thrown
    // println!("The value of number is: {number}");
    println!("The value of number is: {second_number}");
    
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1);
    
    // s1 is still available

    println!("The length of '{}' is {}.", s1, len);


     let mut s = String::from("hello");

    change(&mut s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}