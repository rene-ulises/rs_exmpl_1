use std::io::stdin;

fn main() {
    println!("Enter an integer value: ");
    let mut input_text = String::new();
    stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => println!("your integer input: {}", i),
        Err(..) => println!("this was not an integer: {}", trimmed),
    };


    println!("Enter an integer value: ");
    let mut in_str1 = String::new();
    stdin().read_line(&mut in_str1).expect("failed to read input.");
    let par_in: i32 = in_str1.trim().parse().expect("invalid input");
    println!("User entered value is {}", par_in);


    let mut line = String::new();
    println!("Enter your name :");
    stdin().read_line(&mut line).unwrap();
    println!("Hello , {}", line);

}


