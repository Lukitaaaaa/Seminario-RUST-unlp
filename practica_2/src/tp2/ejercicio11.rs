/*
11-Definir la función llamada multiplicar_valores que recibe como parámetro un arreglo de
enteros y otro número entero llamado factor. Esta función multiplica los valores del arreglo
por el parámetro factor modificándolo.
*/

#[allow(dead_code)]
pub fn multiplicar_valores(arr: &mut Vec<i32>, factor: i32) {
    for i in 0..arr.len(){
        arr[i] = arr[i] * factor;
    }
}