use std::io;

/// Crate comment.
/// What does this crate do?

fn main() {
    // Doc comments below:
    //! # Main function
    //! Read user input from stdin and print it back to stdout
    //! 
    //! Declare a mutable variable named input
    let mut input = String::new();
    println!("Input");
    // Read the stdin and store it in input
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("Success");
            println!("You typed: {}", input);
        }
        Err(e) => {
            println!("Error: {}", e)
        }
    }
    println!("Hello, world!");
}


// Comment types:


// 1. Line comments
//   1.1. Single line comments
//         // This is a single line comment
//   1.2. Multi line comments
//         /* This is a multi line comment */


// 2. Block comments
//   2.1. Single line block comments
//         /// This is a single line block comment
//   2.2. Multi line block comments
//         /** This is a multi line block comment */


// 3. Doc comments
//  3.1. Single line doc comments
//        //! This is a single line doc comment
//  3.2. Multi line doc comments
//        /*! This is a multi line doc comment */
//  3.3. Heading doc comments
//        //! # Heading
