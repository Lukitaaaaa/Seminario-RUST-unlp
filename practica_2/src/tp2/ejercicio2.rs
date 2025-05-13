/*
2- Definir la función llamada es_primo que recibe un número entero positivo mayor a 1 y
retorna true si es primo, false caso contrario.

*/

#[allow(dead_code)]
pub fn es_primo(n: u32) -> bool {
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