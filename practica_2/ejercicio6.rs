fn longitud_de_cadenas(arr: [String; 2]) -> [usize; 2] {
    let mut aux:[usize;2] = [0, 0];

    for i in 0..arr.len(){
        aux[i] = arr[i].chars().count();
    }

    return aux;
}

fn main(){
    let array:[String; 2] = ["Hola".to_string(), "Mundo".to_string()];

    let array_cant:[usize;2] = longitud_de_cadenas(array);
    for i in 0..array_cant.len(){
        println!("longitud: {}", array_cant[i]);
    }
}

