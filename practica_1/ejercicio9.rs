/*
9- Escribir un programa que defina un arreglo de 5 números enteros, y luego imprima la
suma de los valores del arreglo.
*/

fn main(){
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let mut suma:i32 = 0;
    for i in 0..5{
        suma += array[i]; // Sumar cada elemento del array
    }

    println!("La suma de los elementos del arreglo es: {}", array.iter().sum::<i32>());
    println!("La suma de los elementos del arreglo con un for es: {}", suma);
}