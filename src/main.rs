use std::io;

fn main() {
    println!("Caesar Cipher");

    println!("Do you want to encrypt or decrypt? (1 for encrypt | 2 for decrypt)");
    let mut decision = String::new();

    io::stdin()
        .read_line(&mut decision)
        .expect("Failed to read line");

    let mut phrase = String::new();

    println!("{decision}");

    if decision.trim() == "1" {
        let mut encoded_phrase = String::new();

        println!("Enter string to encrypt");
        io::stdin()
            .read_line(&mut phrase)
            .expect("Failed to read line");
    
        let mut encoded_item: char;
        for item in phrase.trim().chars() { 
            //println!("item: {}", item);
            encoded_item = shift_char(item);
            encoded_phrase.push(encoded_item);
        };

        println!("{encoded_phrase}");

    } else {
        let mut decoded_phrase = String::new();

        println!("Enter string to decrypt");
        io::stdin()
            .read_line(&mut phrase)
            .expect("Failed to read line");

        let mut decoded_item: char;
        for item in phrase.trim().chars() { 
            //println!("item: {}", item);
            decoded_item = shift_reverse_char(item);
            decoded_phrase.push(decoded_item);
        };

        println!("{decoded_phrase}");

    }


}

fn shift_char(a: char) -> char {
    // Change u8 to char
    let mut b: u8 = a as u8;

    // Run cipher shift
    b = b + 5;
    
    // Return result
    return b as char;
}

fn shift_reverse_char(a: char) -> char {
    // Change u8 to char
    let mut b: u8 = a as u8;

    // Run cipher shift
    b = b - 5;
    
    // Return result
    return b as char;
}