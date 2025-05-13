/*
8- Definir la función llamada sumar_arreglos que recibe 2 arreglos del mismo tamaño de
números flotantes y retorna un nuevo arreglo que contiene la suma de los elementos de los
arreglos pasados por parámetro, correspondiéndose el resultado con cada posición de los
arreglos pasados por parámetro.
*/

#[allow(dead_code)]
pub fn sumar_arreglos(arr1:[f32;5], arr2:[f32;5])->[f32;5]{
    let mut aux:[f32;5] = [0.0;5];
    
    for i in 0..aux.len(){
        aux[i] = arr1[i] + arr2[i];
    }

    return aux;
}