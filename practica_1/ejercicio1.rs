
// 1- Escribir un programa que defina una variable de tipo flotante con algún valor, y luego
// permita al usuario ingresar un número decimal por teclado para multiplicar, dividir, sumar y
// restar su valor. Se deben imprimir los resultados.

use std::io::stdin;
fn main() {
    let mut input = String::new();
    let valor: f32 = 3.0; // Definimos una variable de tipo flotante con un valor inicial

    println!("Ingrese un número decimal:");
    stdin().read_line(&mut input).expect("failed"); 
    let num: u32 = input.trim().parse().expect("not an integer!");
    
    // let suma = input + valor; 
	// let resta = input - valor; 
	// let producto = input * valor; 
	// let division = input / valor; 

    println!("Suma: {}", num as f32 + valor);
    println!("Resta: {}", num as f32 - valor);
    println!("Producto: {}", num as f32 * valor);
    println!("División: {}", num as f32 / valor);
}