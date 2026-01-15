fn main() {
    // We create a String variable with the word "Rust"
    let s1 = String::from("Rust");
    // We call the calculate_length function and pass the data as a read-only reference
    let len = calculate_length(&s1);

    println!("La longitud de {} es {}", s1, len);

}

// We define the function and specify it receives a read-only reference, not for modification
fn calculate_length (s: &String) -> usize {
    // Return the length of the borrowed string
    s.len()

}