fn es_par(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numero = 3;
    if es_par(numero) {
        println!("El número {} es par.", numero);
    } else {
        println!("El número {} es impar.", numero);
    }
}


