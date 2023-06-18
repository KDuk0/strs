
fn main() {
    //standard string
    let mut word = String::from("Hello world!");
    println!("Digest: {word}");
    
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

    
}