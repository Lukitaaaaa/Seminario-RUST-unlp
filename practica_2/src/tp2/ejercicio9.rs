/*
9-Definir la función llamada cantidad_en_rango que recibe 3 parámetros: 1 arreglo de
enteros, un número entero llamado inferior y otro número entero llamado superior. Esta
función retorna la cantidad de números del arreglo que están entre el rango de los
parámetros inferior y superior inclusive.
*/

#[allow(dead_code)]
pub fn cantidad_en_rango(arr:Vec<i32>, li:i32, ls:i32)->i32{
    let mut cont:i32 = 0;

    for i in 0..arr.len(){
        if arr[i] >= li && arr[i] <= ls {
            cont += 1;
        }
    }

    return cont;
}