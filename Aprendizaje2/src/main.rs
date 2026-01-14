fn main() {
    let s1 = String::from("Hola");

    let s2 = s1;

    // println!("{}", s1); -> This would be wrong because of Rust's "Ownership" model. 
    // When moving data, s1's value is moved to s2; therefore, s1 becomes invalid and no longer exists in scope.
    // This memory safety management is what sets Rust apart from other languages.
    
    println!("{}", s2);
}
