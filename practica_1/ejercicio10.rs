/*
10- Escribir un programa que defina dos arreglos de 5 números enteros cada uno, y luego
cree un tercer arreglo que contenga la suma de los elementos de los dos arreglos
originales.
*/

fn main(){
    let array1: [i32; 5] = [1, 2, 3, 4, 5];
    let array2: [i32; 5] = [6, 7, 8, 9, 10];

    let mut array3: [i32; 5] = [0; 5]; // Inicializar un nuevo arreglo para almacenar la suma

    for i in 0..5 {
        array3[i] = array1[i] + array2[i]; // Sumar los elementos de los dos arreglos
    }

    println!("La suma de los elementos de los arreglos es: {:?}", array3); // Imprimir el resultado
}