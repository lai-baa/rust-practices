// obtain user input and then print the result as output: io = input/output library
// io library comes from the standard library, known as std
// use std::cmp::Ordering;
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
    
        // creating a new variable using let statement. 
            // variables are immutable by default in rust. to make the variable mutable, we add "mut" before the variable name
            // String::new is a function that returns a new instance of a String
            // The :: syntax in the ::new line indicates that new is an associated function of the String type
        // declaring a variable guess that is mutable - the variable is a string - and we are creating a new string
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
    
        // prints the string that now contains the user’s input.
            // {} set of curly brackets is a placeholder
            // When printing the value of a variable, the variable name can go inside the curly brackets
                // let x = 5;
                // let y = 10;
                // println!("x = {x} and y + 2 = {}", y + 2);
        
        // this print statement is a s string interpolation - there are two ways to a a string interpolation
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
