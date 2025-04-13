/*
11- Escribir un programa que defina un arreglo de 5 cadenas, y luego permita al usuario
ingresar una cadena por teclado. El programa debe imprimir un mensaje si la cadena
ingresada por el usuario se encuentra en el arreglo.

*/

use std::io::stdin;
fn main(){
    let array:[String; 5] = ["Hola".to_string(), "Mundo".to_string(), "Rust".to_string(), "Es".to_string(), "Genial".to_string()];

    let mut input = String::new();
    println!("Ingrese una cadena:");
    stdin().read_line(&mut input).expect("failed");
    let cadena = input.trim().to_string();

    for i in 0..5 {
        if cadena == array[i] {
            println!("La cadena ingresada es igual a la cadena en la posición {} del array", i);
        } 
    }
}