
fn es_primo(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main(){
    let numero = 7;

    es_primo(numero);
    if es_primo(numero) {
        println!("El número {} es primo.", numero);
    } else {
        println!("El número {} no es primo.", numero);
    }
}