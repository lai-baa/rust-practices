use rand::Rng; // importing the the range method from the random library - to specify the range of our secret number
use std::cmp::Ordering; // importing using the standard library importing comparing library then importing the Ordering enumerator/class
use std::io; // using the standard library and importing the input/output library

// function main is the main function that all code must run inside of in order to be evaluated/ utilized
fn main() {
    // the print to the console - typically the ! indicates for print strings and w/o bang is for printing functions
    println!("Guess the number!");

    // constant variable b/c we did not use mut using snake case
    // declaring the number is a u32 type number
    // getting random number from range and the numbers in the range are being generated with the gen range function
    let secret_number = rand::thread_rng().gen_range(1..=100); 

    // println!("The secret number is: {secret_number}");

    // creating a loop through so we can continue guessing if number is incorrect
    loop {
        println!("Please input your guess.");
    
        // declaring a variable guess that is mutable - the variable is a string - and we are creating a new string
        let mut guess = String::new();
    
        // read_line puts whatever the user enters into the string we pass to it and returns a result
        // Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states
            // Each possible state is called a variant
            // Result’s variants are Ok and Err
        // input/output from the std library and we are taking the input from the console and reading the guess and returns the result
        // the expect is required b/c you have to error handle the read line result
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // creating an immutable variable (guess) a shadow variable (same identifier but different type)
        // we are getting the mutable variable guess into a u32 inter, using trim to remove whitespace, parsing the string into an integer
        // we print the number if okay and stored in the constant guess
        // we use the underscore to essentially catchall the error no matter which kind and we continue, we do not print
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        // this print statement is a string interpolation - there are two ways to a string interpolation
        // the first is to do the same as directly below; however just place the variable within the curly braces and remove the comma. 
        // The one below is the other and this way is used because we cannot do anything with the variable inside the curly braces 
        // however we can do operations and manipulations to the variable outside of the curly braces
        println!("You guessed {}", guess);

        // match is checking if we have a match when we do a guess compare of the secret number
        // we return the guess compare function with checking if the guess is too big or too small and break/end loop if we are correct
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // the & sign means - that the argument is a reference - which gives you a way to let multiple parts of your code 
    // access one piece of data without needing to copy that data into memory multiple times
}
