/*
5- Escribir un programa que defina una variable de tipo cadena, y luego permita al usuario
ingresar una cadena por teclado para concatenar su valor. El programa debe imprimir la
cadena en mayúsculas.
*/

use std::io;
fn main() {
    let mut cadena = String::new();
    println!("Ingrese una cadena:"); 
    io::stdin().read_line(&mut cadena).expect("Error al leer la línea");
    let cadena_concatenada = format!("{}{}", cadena.trim(), " concatenada");
    println!("Cadena en mayúsculas: {}", cadena_concatenada.to_uppercase());
}