fn nigga(mut a: [f32;5]) -> [f32;5]{
    //numero flotante
    for i in 0..a.len(){
        a[i] = a[i]*2.0;
    }

    return a;
}

fn main(){
    //array de flotantes
    let mut array:[f32;5] = [2.0, 2.3, 4.0, 3.4, 5.0];
    array = nigga(array);
    for i in 0..array.len(){
        println!("{}",array[i]);
    }
}