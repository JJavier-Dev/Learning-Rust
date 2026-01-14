fn main() {
    // The compiler warned me to use "VELOCIDAD_LUZ" (SCREAMING_SNAKE_CASE) because it is a constant, 
    // indicating that the value is static and will never change.
    const VELOCIDAD_LUZ: i32 = 299792;
    // The compiler warned me again because I used "Distancia". Names starting with capital letters 
    // are reserved for Structs or Data Types (PascalCase).
    let distancia = 1000000;
    // "distancia" and "VELOCIDAD_LUZ" are cast to f64 (floating-point) because it allows up to 15 or 16 
    // decimal places. Otherwise, it would trigger a type mismatch error since the result is a decimal, not an integer.
    let tiempo = distancia as f64 / VELOCIDAD_LUZ as f64;
    println!("El tiempo es {}", tiempo)
}