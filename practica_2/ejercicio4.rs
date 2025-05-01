fn cantidad_impares(a: [i32;5])-> i32{
    let mut cant:i32 = 0;
    for i in 0..a.len(){
        if a[i] % 2 != 0 {
            cant += 1;
        }
    }

    return cant;
}

fn main(){
    let array: [i32;5] = [ 1, 4, 6, 8, 10 ];
    
    println!("La cantidad de elementos impares es {}.", cantidad_impares(array));
}