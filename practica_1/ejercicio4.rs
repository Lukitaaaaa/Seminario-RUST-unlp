/*
4- Escribir un programa que defina una tupla que contenga una cadena, un número entero
con signo y un valor booleano, y luego imprima cada valor de la tupla
*/

fn main(){
    let tupla: (String, i32, bool) = ("Hola".to_string(), -5, true);
    println!("Cadena: {}", tupla.0);
    println!("Número entero: {}", tupla.1);
    println!("Valor booleano: {}", tupla.2);
}