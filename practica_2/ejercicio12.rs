fn main(){
    let mut array: [i32; 5] = [2, 2, 4, 3, 5];

    reemplazar_pares(&mut array);

    println!("RESULTADOS: {:?}", array);
}

fn reemplazar_pares(arr: &mut [i32;5]) {
    for i in 0..arr.len(){
        if arr[i] % 2 == 0{
            arr[i] = -1;
        }
    }
}