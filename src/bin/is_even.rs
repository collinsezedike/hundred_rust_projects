fn main () {
    let num = 3;
    match is_even(num) {
        true => println!("{} is an even number", num),
        false => println!("{} is not an even number", num),
    };
}

fn is_even(num: u32) -> bool {
    num % 2 == 0
}