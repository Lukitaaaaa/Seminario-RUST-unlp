/*
13-Definir una función llamada ordenar_nombres que recibe un arreglo de String y los
ordena en orden alfabético.
*/

fn main(){
    let mut array:[String;5] = ["Amanda".to_string(), "Bruno".to_string(), "Ana".to_string(), "Lucas".to_string(), "Carlos".to_string()];

    ordenar_nombres(&mut array);    
}

fn ordenar_nombres(arr: &mut [String;5]) {   
    arr.sort();
    println!("RESULTADOS: {:?}", arr);
}