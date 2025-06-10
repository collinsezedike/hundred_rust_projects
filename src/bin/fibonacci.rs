fn main() {
    let fib_num = find_fib_num(40);
    println!("{:?}", fib_num); 
}


fn find_fib_num(index: u32) -> Result<u32, String> {
    let mut start = 0;
    let mut next  = 1;
    
    match index {
        0 => Err(String::from("Invalid index provided")),
        1 => Ok(start),
        2 => Ok(next),
        _ => {
            for _i in 2..index {
                let new_start = next;
                next = start + next;
                start = new_start;
            }
            Ok(next)
        }
    }
}
