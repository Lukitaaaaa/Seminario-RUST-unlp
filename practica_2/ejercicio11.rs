fn main() {

    let mut array: [i32; 5] = [2, 2, 4, 3, 5];
    let factor: i32 = 3;

    multiplicar_valores(&mut array, factor);

    println!("RESULTADOS: {:?}", array);

}

fn multiplicar_valores(arr: &mut [i32;5], factor: i32) {
    for i in 0..arr.len(){
        arr[i] = arr[i] * factor;
    }
}