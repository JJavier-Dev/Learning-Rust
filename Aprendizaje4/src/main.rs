fn main() {
    // We create a mutable variable so it doesn't remain "frozen" or immutable
    let mut s = String::from("Hello");
    // Then we pass explicit permission to the 'modify' function so it can change it
    modify(&mut s);

    println!("{}", s);

    // We perform the same process, but this time using numbers.
    let mut x: i32 = 43;

    multiply(&mut x);

    println!("{}", x);

}

// The function signature specifies it receives a mutable reference to a String
fn modify (text: &mut String) {
    // We append the string knowing we have borrowed mutable permission
    text.push_str(" World")

}

fn multiply(num: &mut i32) {
    // Something different happens here compared to Strings. Rust is more 'user-friendly' with Strings, but with numbers, it is different. 
    // num represents the memory address where our number is located, not the number itself.
    // Therefore, we use the asterisk (*) to specify that we want to access the value stored inside that address.
    *num = *num * 3;
}