use std::fs;
use std::io::Write;


fn main(){
    let content = fizzbuzzer(100);
    let filename = "target/fizzbuzz.txt";
    write_to_file(filename, &content);
}


fn fizzbuzzer(n: u32) -> Vec<String> {
    let mut fizzbuzz: Vec<String> = Vec::with_capacity(n as usize);

    for i in 1..=n {
        if (i % 3 == 0) && (i % 5 == 0) {
            fizzbuzz.push("fizzbuzz".to_string())
        } else if i % 3 == 0 {
            fizzbuzz.push("fizz".to_string())
        } else if i % 5 == 0 {
            fizzbuzz.push("buzz".to_string())
        } else {
            fizzbuzz.push(i.to_string())
        }
    }

    return fizzbuzz;
}


fn write_to_file(filename: &str, content: &Vec<String>) {
    let content_str = content.join("\n");
    let content_byte = content_str.as_bytes();

    let mut file = fs::File::create(filename).expect("Unable to get or create file");
    file.write(content_byte).expect("Unable to write to file");
}