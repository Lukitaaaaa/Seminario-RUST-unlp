/*
6-Definir la función llamada longitud_de_cadenas que recibe un arreglo de String y retorna
un arreglo con la longitud de las cadenas del parámetro, correspondiéndose en posición del
arreglo.
*/

use std::vec;

#[allow(dead_code)]
pub fn longitud_de_cadenas(arr: Vec<String>) -> Vec<u32> {
    
    let mut vector:Vec<u32> = vec![];

    for i in 0..arr.len(){
        vector.push(arr[i].chars().count() as u32);
        
    }

    return vector;
}