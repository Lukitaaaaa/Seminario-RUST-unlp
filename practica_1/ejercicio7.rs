/*
7- Escribir un programa que defina una variable de tipo arreglo que contenga seis números
enteros, y luego multiplique cada valor del arreglo por un valor constante definido,
modificando el contenido del arreglo.
*/

fn main(){
    let array = [1, 2, 3, 4, 5, 6];
    const PRODUCTO: i32 = 5; // Definimos una constante para el producto

    for i in 0..array.len() {
        let resultado = array[i] * PRODUCTO; // Multiplicamos cada elemento del array por el producto
        println!("El resultado de multiplicar {} por {} es: {}", array[i], PRODUCTO, resultado); // Imprimimos el resultado
    }
}