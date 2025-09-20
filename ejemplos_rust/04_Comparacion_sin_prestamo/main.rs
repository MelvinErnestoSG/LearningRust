fn main() {
    //(transferencia de propiedad).
    let mensaje1 = String::from("Sin préstamo.");
    let mensaje2 = mensaje1; // mensaje1 ya no es válido después de esta línea.
    // println!("{}", mensaje1); // Esto causaría un error de compilación.
    println!("{}", mensaje2); // Solo mensaje2 es válido.
}
