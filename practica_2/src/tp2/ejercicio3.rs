/*
3- Definir la función llamada suma_pares que recibe como parámetro un arreglo de
números enteros y retorna la suma de los números pares.
*/

#[allow(dead_code)]
pub fn suma_pares(a: [i32;5]) -> i32 {
    let mut suma:i32 = 0;
    for i in 0..a.len(){
        if a[i] % 2 == 0 {
            suma += a[i];
        }
    }

    return suma;
}