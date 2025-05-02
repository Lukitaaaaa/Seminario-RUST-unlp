/*
4- Definir la función llamada cantidad_impares que recibe como parámetro un arreglo de
números enteros y retorna la cantidad de números impares.

*/


fn main(){
    let array: [i32;5] = [ 1, 4, 6, 8, 10 ];
    
    println!("La cantidad de elementos impares es {}.", cantidad_impares(array));
}

fn cantidad_impares(a: [i32;5])-> i32{
    let mut cant:i32 = 0;
    for i in 0..a.len(){
        if a[i] % 2 != 0 {
            cant += 1;
        }
    }

    return cant;
}