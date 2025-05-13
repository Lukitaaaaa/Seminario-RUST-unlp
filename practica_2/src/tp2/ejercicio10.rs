/*
10-Definir la función llamada cantidad_de_cadenas_mayor_a que recibe como parámetros
un arreglo de String y un entero llamado límite. Esta función retorna la cantidad de Strings
del arreglo que son de longitud mayor al parámetro límite.
*/

#[allow(dead_code)]
pub fn cantidad_de_cadenas_mayor_a(arr:Vec<String>, li:i32)->i32{
    let mut cont:i32 = 0;
    for i in 0..arr.len(){
        if arr[i].len() as i32 >= li {
            cont += 1;
        }
    }
    return cont;
}