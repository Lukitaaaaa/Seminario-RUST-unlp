/*
14-Definir una función llamada incrementar que recibe como parámetro un número flotante
e incrementa en 1 su valor.
*/

#[allow(dead_code)]
pub fn incrementar(flotante: &mut f32){
    *flotante += 1.0; 
}