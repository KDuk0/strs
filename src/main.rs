//need this to turn string back into number
use std::str::FromStr;

fn main() {
    //standard string with default value
    let mut word = String::from("Hello world!");
    println!("{word}");
    let words = "New string".to_string();
    println!("{words}");
    
    //adding word to string
    word.push_str(" What?");
    println!("{}", word);
    
    //slicing string to the first 5 bytes
    let slice = &word [0..5];
    println!("{}", slice);

    //iterating with chars() method
    let word1 = String::from("Hello");
    for char in word1.chars() {
        println!("{}", char);
    }

    //reverse with rev() method
    for char in word1.chars().rev() {
        println!("{}", char);
    }

    //starting with empty string
    let mut word2 = String::new();
    word2.push_str("Not hello");
    println!("{}", word2);

    //str(string literal) is known at compile time and is stored on the stack
    let word3:&str = "Static";
    println!("{}", word3);

    //converting number to string
    let number:u8 = 5;
    number.to_string();
    println!("{}", number);
    //converting back to number
    let number1: Result<u8, _> = u8::from_str("5");
    println!("{:?}", number1);
    
    //replacing one string with another
    let word4 = String::from("Hello, bro, Hello");
    println!("{} {}", word4.replace("Hello", "Holla"), word4.len());
    println!("{:?}", word4.to_uppercase());

    //combine 2 strings with +
    let s1 = "Word1".to_string();
    let s2 = " Word2".to_string();
    let s3 = s1.clone() + &s2;
    println!("{}", s3);
}