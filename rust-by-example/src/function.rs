fn add(first: u128, second: u128) -> u128 {
    return first + second;
}

fn multiply(first: u128, second: u128) -> u128 {
    first * second
}

fn minus(first: u128, second: u128) -> u128 {
    second - first
}

pub fn main() {
    let mut add_result: u128 = 0;
    let mut mul_result: u128 = 0;
    let mut min_result: u128 = 0;

    println!(
        "first add = {add}
        first mul = {mul}
        first min = {min}",
        add = add_result,
        mul = mul_result,
        min = min_result
    );

    let first_arq: u128 = 5;
    let second_arq: u128 = 10;

    add_result = add(first_arq, second_arq);
    assert_eq!(add_result, 15);

    mul_result = multiply(first_arq, second_arq);
    assert_eq!(mul_result, 50);

    min_result = minus(first_arq, second_arq);
    assert_eq!(min_result, 5);

    println!(
        "second add = {add}
        second mul = {mul}
        second min = {min}",
        add = add_result,
        mul = mul_result,
        min = min_result
    );

    
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");



}
