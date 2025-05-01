fn main(){
    let array: [String;5] = ["Hola".to_string(), "Mundo".to_string(), "Rust".to_string(), "Es".to_string(), "Genial".to_string()];
    let li: i32 = 5;
    let resultado = cant(array, li);
    println!("Cantidad de elementos con longitud mayor o igual a {}: {}", li, resultado);
}

fn cant(arr:[String;5], li:i32)->i32{
    let mut cont:i32 = 0;
    for i in 0..arr.len(){
        if arr[i].len() as i32 >= li {
            cont += 1;
        }
    }
    return cont;
}