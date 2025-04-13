/*
6- Escribir un programa que defina una variable de tipo entero sin signo, y luego permita al
usuario ingresar un número entero por teclado para sumarse con la variable definida. El
programa debe imprimir el valor del número elevado al cuadrado.
*/
use std::io;
fn main() {
    let mut input = String::new(); // Creamos una variable mutable para almacenar la entrada del usuario
    
    println!("Ingrese un numero");
    io::stdin().read_line(&mut input).expect("error"); // Leemos el número ingresado por el usuario
    let numero: u32 = input.trim().parse().expect("Error al convertir el número"); // Convertimos la entrada a un número entero sin signo
    
    println!("El resultado del valor elevado al cuadrado es: {}", numero * numero); // Imprimimos el resultado en hexadecimal
}