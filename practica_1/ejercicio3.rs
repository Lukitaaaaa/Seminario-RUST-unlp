/*
3- Escribir un programa que defina una variable de tipo booleano, 
y luego permita al usuario ingresar un valor booleano por teclado 
para actualizar su valor haciendo las operaciones and y or. Se deben 
imprimir ambos resultados.
*/

use std::io::stdin;
fn main(){
    let boolean: bool = false;

    let mut input = String::new();
    println!("Ingrese un valor booleano (true/false):");
    stdin().read_line(&mut input).expect("failed");

    match input.trim() {
        "true" => {
            println!("{boolean} & true = {}", boolean & true);
            println!("{boolean} | true = {}", boolean | true);
        },
        "false" => {
            println!("{boolean} & false = {}", boolean & false);
            println!("{boolean} | false = {}", boolean | false);
        },
        _ => println!("{} is not a boolean", input.trim()),
    }
}