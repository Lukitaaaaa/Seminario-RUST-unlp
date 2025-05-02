/*
1-Definir la función llamada es_par que recibe como parámetro un número entero y retorna
true si el número es par, false caso contrario.

*/


fn main() {
    let numero = 3;
    if es_par(numero) {
        println!("El número {} es par.", numero);
    } else {
        println!("El número {} es impar.", numero);
    }
}

fn es_par(n: i32) -> bool {
    n % 2 == 0
}

