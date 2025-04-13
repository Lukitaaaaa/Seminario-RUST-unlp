/*
12- Escribir un programa que defina una tupla que contenga una cadena y un arreglo de
enteros, y luego imprima la cadena y la suma de los valores en el arreglo.
*/

fn main(){
    let array: [i32; 3] = [1, 2, 3];
    let tupla: (String, [i32; 3]) = ("Cadena de la tupla".to_string(), array);  
    
    println!("Cadena: {}", tupla.0);
    println!("Suma de los valores del arreglo: {}", tupla.1.iter().sum::<i32>());
}