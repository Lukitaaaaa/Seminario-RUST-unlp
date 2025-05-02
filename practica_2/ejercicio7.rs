/*
7-Definir la función llamada cantidad_de_mayores que recibe como parámetro un arreglo
de números enteros y un número entero llamado límite. Esta función retorna la cantidad de
números mayores al límite que tiene el arreglo.

*/


fn main(){
    let limite = 2;
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    
    let cantidad = cantidad_de_mayores(limite, array);

    println!("Los numeros mayores al limite son :{}", cantidad);
}

fn cantidad_de_mayores(l:i32, a:[i32;5])-> i32{
    let mut aux = 0;
    for i in 0..a.len(){
        if a[i] > l{
            aux+= 1;
        }

    }

    return aux;
}