fn main() {
    // The compiler warned me to use "VELOCIDAD_LUZ" (SCREAMING_SNAKE_CASE) because it is a constant, 
    // indicating that the value is static and will never change.
    const SPEED_OF_LIGHT: i32 = 299792;
    // The compiler warned me again because I used "Distancia". Names starting with capital letters 
    // are reserved for Structs or Data Types (PascalCase).
    let distance = 1000000;
    // "distancia" and "VELOCIDAD_LUZ" are cast to f64 (floating-point) because it allows up to 15 or 16 
    // decimal places. Otherwise, it would trigger a type mismatch error since the result is a decimal, not an integer.
    let time = distance as f64 / SPEED_OF_LIGHT as f64;
    println!("Time is {}", time);
}