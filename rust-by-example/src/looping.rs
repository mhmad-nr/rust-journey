pub fn main() {
    for i in 0..5 {
        println!("first={}", i);
    }

    let array: [u8; 5] = [1, 2, 3, 4, 5];

    for item in array {
        println!("second={}", item);
    }

    let mut sum: u8 = 0;

    for item in array {
        sum += item;
    }
    assert_eq!(sum, 15);
    println!("third={}", sum);
}
