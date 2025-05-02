/*
9-Definir la función llamada cantidad_en_rango que recibe 3 parámetros: 1 arreglo de
enteros, un número entero llamado inferior y otro número entero llamado superior. Esta
función retorna la cantidad de números del arreglo que están entre el rango de los
parámetros inferior y superior inclusive.

*/

fn main(){
    let array: [i32; 5] = [2, 2, 4, 3, 5];
    let li: i32 = 3;
    let ls: i32 = 4;

    let resultado = cantidad_en_rango(array, li, ls);
    println!("Cantidad de elementos entre {} y {}: {}", li, ls, resultado);   
}

fn cantidad_en_rango(arr:[i32;5], li:i32, ls:i32)->i32{
    let mut cont:i32 = 0;

    for i in 0..arr.len(){
        if arr[i] >= li && arr[i] <= ls {
            cont += 1;
        }
    }

    return cont;
}