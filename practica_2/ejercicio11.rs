/*
11-Definir la función llamada multiplicar_valores que recibe como parámetro un arreglo de
enteros y otro número entero llamado factor. Esta función multiplica los valores del arreglo
por el parámetro factor modificándolo.
*/

fn main() {

    let mut array: [i32; 5] = [2, 2, 4, 3, 5];
    let factor: i32 = 3;

    multiplicar_valores(&mut array, factor);

    println!("RESULTADOS: {:?}", array);

}

fn multiplicar_valores(arr: &mut [i32;5], factor: i32) {
    for i in 0..arr.len(){
        arr[i] = arr[i] * factor;
    }
}