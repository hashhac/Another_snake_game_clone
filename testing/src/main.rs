fn main(){
    let x: i32 = 1;
    let y: i32 = 2;
    let result: i32 = x*y;
    let z: Option<i32> = Some(result);
    println!("If i use the some operator in rust {:?}", z);
    process_option(Some(10)); // Output: We got the number: 10
    process_option(None);    // Output: There was no number.
}

fn process_option(opt: Option<i32>) {
    match opt {
        Some(number) => {
            // If it's Some, 'number' gets the i32 value inside
            println!("We got the number: {}", number);
        }
        None => {
            // If it's None
            println!("There was no number.");
        }
    }
}

