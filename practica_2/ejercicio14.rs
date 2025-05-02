/*
14-Definir una función llamada incrementar que recibe como parámetro un número flotante
e incrementa en 1 su valor.
*/

fn main(){
    let mut flotante: f32 = 2.0;
    incrementar(&mut flotante);
    println!("Flotante después de incrementar: {}", flotante);
}

fn incrementar(flotante: &mut f32){
    *flotante += 1.0; 
}