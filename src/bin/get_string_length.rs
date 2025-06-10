fn main() {
    let s = String::from("Hello, world!");
    let str_lenght = get_string_length(&s);
    println!("The length of the string is: {:?}", str_lenght);
}

fn get_string_length(s: &str) -> usize {
    s.len()
}