fn sumar_arreglos(arr1:[f32;5], arr2:[f32;5])->[f32;5]{
    let mut aux:[f32;5] = [0.0;5];
    
    for i in 0..aux.len(){
        aux[i] = arr1[i] + arr2[i];
    }

    return aux;
}

fn main(){
    let array1:[f32;5] = [2.0, 4.0, 6.0, 8.0, 10.0];
    let array2:[f32;5] = [2.0, 4.0, 6.0, 8.0, 10.0];
    let resultado = sumar_arreglos(array1,array2);
    println!("RESULTADOS: {:?}", resultado);
}