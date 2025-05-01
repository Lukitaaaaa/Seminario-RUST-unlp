fn main(){
    let array: [i32; 5] = [2, 2, 4, 3, 5];
    let li: i32 = 3;
    let ls: i32 = 4;

    let resultado = cant(array, li, ls);
    println!("Cantidad de elementos entre {} y {}: {}", li, ls, resultado);   
}

fn cant(arr:[i32;5], li:i32, ls:i32)->i32{
    let mut cont:i32 = 0;

    for i in 0..arr.len(){
        if arr[i] >= li && arr[i] <= ls {
            cont += 1;
        }
    }

    return cont;
}