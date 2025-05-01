fn main(){
    let mut flotante: f32 = 2.0;
    incrementar(&mut flotante);
    println!("Flotante después de incrementar: {}", flotante);
}

fn incrementar(flotante: &mut f32){
    *flotante += 1.0; 
}