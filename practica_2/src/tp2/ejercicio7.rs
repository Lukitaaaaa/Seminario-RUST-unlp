/*
7-Definir la función llamada cantidad_de_mayores que recibe como parámetro un arreglo
de números enteros y un número entero llamado límite. Esta función retorna la cantidad de
números mayores al límite que tiene el arreglo.
*/

#[allow(dead_code)]
pub fn cantidad_de_mayores(l:i32, a:Vec<i32>)-> i32{
    let mut aux:i32 = 0;
    for i in 0..a.len(){
        if a[i] < l{
            aux+= 1;
        }
    }
    return aux;
}