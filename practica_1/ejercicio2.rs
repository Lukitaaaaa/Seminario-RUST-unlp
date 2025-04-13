/*
2- Escribir un programa que defina una variable de tipo 
entero sin signo, y luego imprima su valor en hexadecimal.
*/

fn main() {

    let entero: u32 = 25;
    let hexadecimal = format!("{:X}", entero); // Convertir el número a hexadecimal
    println!("El número {} en hexadecimal es: {}", entero, hexadecimal); // Imprimir el resultado

}