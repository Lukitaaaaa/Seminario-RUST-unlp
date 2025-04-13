/*
8- Escribir un programa que defina una constante de tipo cadena, y luego imprima el
número de veces que un caracter específico ingresado por el usuario aparece en la cadena.
Se debe imprimir el resultado.

*/
use std::io::stdin;
fn main(){
    const CADENA:&str = "Hola guapisimos";

    let mut input = String::new();
    println!("Ingrese un caracter:");
    stdin().read_line(&mut input).expect("failed");
    let c: char = input.trim().parse().expect("not a char!");

    println!("La cantidad de veces que aparecio ese caracter en la cadena es: {}", CADENA.matches(c).count());


}