/*
12-Definir una función llamada reemplazar_pares que recibe un arreglo de enteros y
reemplaza todos los números pares por -1.
*/

#[allow(dead_code)]
pub fn reemplazar_pares(arr: &mut Vec<i32>) {
    for i in 0..arr.len(){
        if arr[i] % 2 == 0{
            arr[i] = -1;
        }
    }
}