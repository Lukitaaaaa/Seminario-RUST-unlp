/*
5-Defina la función llamada duplicar_valores que recibe un arreglo de números flotantes y
retorna un arreglo nuevo con los valores duplicados del parámetro.
*/


fn main(){
    //array de flotantes
    let mut array:[f32;5] = [2.0, 2.3, 4.0, 3.4, 5.0];
    array = duplicar_valores(array);
    for i in 0..array.len(){
        println!("{}",array[i]);
    }
}

fn duplicar_valores(mut a: [f32;5]) -> [f32;5]{
    //numero flotante
    for i in 0..a.len(){
        a[i] = a[i]*2.0;
    }

    return a;
}