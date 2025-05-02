/*
3- Definir la función llamada suma_pares que recibe como parámetro un arreglo de
números enteros y retorna la suma de los números pares.

*/


fn main(){
    let array = [ 2, 4, 6, 8, 10 ];
    
    println!("La suma del array es {}.", suma_pares(array));
}

fn suma_pares(a: [i32;5]) -> i32 {
    let mut suma:i32 = 0;
    for i in 0..a.len(){
        suma += a[i];
    }

    return suma;
}