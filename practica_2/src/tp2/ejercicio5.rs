/*
5-Defina la función llamada duplicar_valores que recibe un arreglo de números flotantes y
retorna un arreglo nuevo con los valores duplicados del parámetro.
*/

#[allow(dead_code)]
pub fn duplicar_valores(mut a: [f32;5]) -> [f32;5]{
    
    for i in 0..a.len(){
        a[i] = a[i]*2.0;
    }

    return a;
}