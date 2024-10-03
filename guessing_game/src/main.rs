// obtain user input and then print the result as output: io = input/output library
// io library comes from the standard library, known as std
// use std::cmp::Ordering;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
    
        // creating a new variable using let statement. 
            // variables are immutable by default in rust. to make the variable mutable, we add "mut" before the variable name
            // String::new is a function that returns a new instance of a String
            // The :: syntax in the ::new line indicates that new is an associated function of the String type
        let mut guess = String::new();
    
        // calling stdin function from the io module, which allows us to handle user input
        // read_line: calls the read_line method on the standard input handle to get input from the user
            // passing &mut guess as the argument to read_line to tell it what string to store the user input in
            // read_line takes whatever the user types into standard input and append that into a string (without overwriting its contents), 
            // so we therefore pass that string as an argument.
        // string argument needs to be mutable so the method can change the string’s content
            // & indicates that this argument is a reference
            // gives you a way to let multiple parts of your code access one piece of data 
            // without needing to copy that data into memory multiple times
        // .expect handles failures
        // read_line puts whatever the user enters into the string we pass to it and returns a result
        // Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states
            // Each possible state is called a variant
            // Result’s variants are Ok and Err
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
    
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
    
        // prints the string that now contains the user’s input.
            // {} set of curly brackets is a placeholder
            // When printing the value of a variable, the variable name can go inside the curly brackets
                // let x = 5;
                // let y = 10;
                // println!("x = {x} and y + 2 = {}", y + 2);
        println!("You guessed {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
