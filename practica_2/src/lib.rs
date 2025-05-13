mod tp2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
   
    fn test_es_par() {
        assert_eq!(tp2::ejercicio1::es_par(2), true);
        assert_eq!(tp2::ejercicio1::es_par(3), false);
        assert_eq!(tp2::ejercicio1::es_par(0), true);
        assert_eq!(tp2::ejercicio1::es_par(-2), true);
        assert_eq!(tp2::ejercicio1::es_par(-3), false);
        assert_eq!(tp2::ejercicio1::es_par(i32::MAX), false);
    }

    #[test]
    fn test_es_primo() {
        assert_eq!(tp2::ejercicio2::es_primo(1), false);
        assert_eq!(tp2::ejercicio2::es_primo(2), true);
        assert_eq!(tp2::ejercicio2::es_primo(3), true);
        assert_eq!(tp2::ejercicio2::es_primo(4), false);
    }

    #[test]
    fn test_suma_de_pares(){
        let a = [1, 2, 3, 4, 5];
        assert_eq!(tp2::ejercicio3::suma_pares(a), 6);
        let b = [0, 2, 4, 6, 8];
        assert_eq!(tp2::ejercicio3::suma_pares(b), 20);
        let c = [-1, -2, -3, -4, -5];
        assert_eq!(tp2::ejercicio3::suma_pares(c), -6);
    }

    #[test]
    fn test_cantidad_impares(){
        let a = [1, 2, 3, 4, 5];
        assert_eq!(tp2::ejercicio4::cantidad_impares(a), 3);
        let b = [0, 2, 4, 6, 8];
        assert_eq!(tp2::ejercicio4::cantidad_impares(b), 0);
        let c = [-1, -2, -3, -4, -5];
        assert_eq!(tp2::ejercicio4::cantidad_impares(c), 3);
    }

    #[test]
    fn test_duplicar_valores(){
        let a = [1.0, 2.0, 3.0, 4.0, 5.0];
        let resultado = [2.0, 4.0, 6.0, 8.0, 10.0];
        assert_eq!(tp2::ejercicio5::duplicar_valores(a), resultado);
        let b = [0.0, 2.5, 3.5, 4.5, 5.5];
        let resultado_b = [0.0, 5.0, 7.0, 9.0, 11.0];
        assert_eq!(tp2::ejercicio5::duplicar_valores(b), resultado_b);
    }

    #[test]
    fn test_longitud_de_cadenas(){
        let a:Vec<String> = vec!["Hola".to_string(), "Mundo".to_string()];
        let resultado = [4, 5];
        assert_eq!(tp2::ejercicio6::longitud_de_cadenas(a), resultado);
        
    }

    #[test]
    fn test_cantidad_mayores(){
        let a: Vec<i32> = vec![1, 2, 3, 4, 5];
        let limite: i32 = 3;
        assert_eq!(tp2::ejercicio7::cantidad_de_mayores(limite, a), 2);
        let b:Vec<i32> = vec![0, -1, -2, -3, -4];
        let limite_b: i32 = -1;
        assert_eq!(tp2::ejercicio7::cantidad_de_mayores(limite_b, b), 3);
    }

    #[test]
    fn test_sumar_arreglos(){
        let arr1 = [1.0, 2.0, 3.0, 4.0, 5.0];
        let arr2 = [5.0, 4.0, 3.0, 2.0, 1.0];
        let resultado = [6.0, 6.0, 6.0, 6.0, 6.0];
        assert_eq!(tp2::ejercicio8::sumar_arreglos(arr1, arr2), resultado);
        
        let arr3 = [1.5, 2.5, 3.5, 4.5, 5.5];
        let arr4 = [5.5, 4.5, 3.5, 2.5, 1.5];
        let resultado_b = [7.0, 7.0, 7.0, 7.0, 7.0];
        assert_eq!(tp2::ejercicio8::sumar_arreglos(arr3, arr4), resultado_b);
    }

    #[test]
    fn test_cantidad_en_rango(){
        let arr:Vec<i32> = vec![1, 2, 3, 4, 5];
        let limite_inferior = 2;
        let limite_superior = 4;
        assert_eq!(tp2::ejercicio9::cantidad_en_rango(arr, limite_inferior, limite_superior), 3);
        
        let arr_b: Vec<i32> = vec![0, -1, -2, -3, -4];
        let limite_inferior_b = -3;
        let limite_superior_b = -1;
        assert_eq!(tp2::ejercicio9::cantidad_en_rango(arr_b, limite_inferior_b, limite_superior_b), 3);
    }

    #[test]
    fn test_cantidad_de_cadenas_mayor_a(){
        let arr: Vec<String> = vec!["Hola".to_string(), "Mundo".to_string(), "Rust".to_string(), "Programacion".to_string(), "Lenguaje".to_string()];
        let limite: i32 = 5;
        assert_eq!(tp2::ejercicio10::cantidad_de_cadenas_mayor_a(arr, limite), 3);
        
        let arr_b: Vec<String> = vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string()];
        let limite_b: i32 = 1;
        assert_eq!(tp2::ejercicio10::cantidad_de_cadenas_mayor_a(arr_b, limite_b), 5);
    }

    #[test]
    fn test_multiplicar_valores(){
        let mut lista:Vec<i32>= vec![1, 2, 3, 4, 5];
        let factor = 3;
        tp2::ejercicio11::multiplicar_valores(&mut lista, factor);
        assert_eq!(lista, [3, 6, 9, 12, 15], "Error en la multiplicación de valores");
    }

    #[test]
    fn test_reemplazar_pares(){
        let mut lista:Vec<i32> = vec![1, 2, 3, 4, 5];
        tp2::ejercicio12::reemplazar_pares(&mut lista);
        assert_eq!(lista, [1, -1, 3, -1, 5]);
    }

    #[test]
    fn test_ordenar_nombres(){
        let mut nombres:Vec<String> = vec!["Juan".to_string(), "Ana".to_string(), "Pedro".to_string()];
        tp2::ejercicio13::ordenar_nombres(&mut nombres);
        assert_eq!(nombres, ["Ana".to_string(), "Juan".to_string(), "Pedro".to_string()]);
    }

    #[test]

    fn test_incrementar(){
        let mut flotante:f32 = 1.0;
        tp2::ejercicio14::incrementar(&mut flotante);
        assert_eq!(flotante, 2.0);
    }
}
