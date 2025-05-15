mod tp3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persona(){
        let mut per1 = tp3::ejercicio1::Persona::new("Lucas".to_string(), 21, Some("18".to_string()));
        let mut per2 = tp3::ejercicio1::Persona::new("Guille".to_string(), 22, None);
        assert_eq!(per1.nombre, String::from("Lucas"));
        assert_eq!(per1.edad, 21);
        assert_eq!(per1.direccion, Some("18".to_string()));

        let string_per1:String = per1.to_string();
        assert_eq!(string_per1, "nombre: Lucas, edad: 21, direccion: 18");
        let string_per2:String = per2.to_string();
        assert_eq!(string_per2, "nombre: Guille, edad: 22, no tiene direccion");

        let edad_per1:u32 = per1.obtener_edad();
        assert_eq!(edad_per1, 21);
        let edad_per2:u32 = per2.obtener_edad();
        assert_eq!(edad_per2, 22);

        per1.actualizar_direccion(Some("17".to_string()));
        assert_eq!(per1.direccion, Some("17".to_string()));

        per2.actualizar_direccion(Some("Ya tengo direccrion".to_string()));
        assert_eq!(per2.direccion, Some("Ya tengo direccrion".to_string()));
    }

    #[test]
    fn test_rectangulo(){
        let rec1 = tp3::ejercicio2::Rectangulo::new(2.0, 4.0);
        let rec2 = tp3::ejercicio2::Rectangulo::new(3.0, 3.0);
        
        let peri1:f32 = rec1.calcular_perimetro();
        assert_eq!(peri1, 12.0);
        let peri2:f32 = rec2.calcular_perimetro();
        assert_ne!(peri2, 10.0);
        
        let area1:f32 = rec1.calcular_area();
        assert_eq!(area1, 8.0);
        let area2:f32 = rec2.calcular_area();
        assert_ne!(area2, 10.0);
        
        let cuadrado1:bool = rec1.es_cuadrado();
        assert_eq!(cuadrado1, false);
        let cuadrado2:bool = rec2.es_cuadrado();
        assert_eq!(cuadrado2, true);
    }

    #[test]
    fn test_fecha(){
        let mut fecha1 = tp3::ejercicio3::Fecha::new(2, 4, 2023);
        let mut fecha2 = tp3::ejercicio3::Fecha::new(2, 4, 2024);
        let mut fecha3 = tp3::ejercicio3::Fecha::new(2, 4, 2022);
        let mut fecha4 = tp3::ejercicio3::Fecha::new(29, 2, 2020);
        let mut fecha5 = tp3::ejercicio3::Fecha::new(29, 2, 1900);
        let mut fecha6 = tp3::ejercicio3::Fecha::new(31, 4, 2021);

        // assert_eq!(fecha1.to_string(), "2-4-2023");
        // assert_eq!(fecha2.to_string(), "2-4-2024");

        assert_eq!(fecha1.es_biciesto(), false);
        assert_eq!(fecha2.es_biciesto(), true);
        assert_eq!(fecha3.es_biciesto(), false);
        assert_eq!(fecha4.es_biciesto(), true);
        assert_eq!(fecha5.es_biciesto(), false);
        assert_eq!(fecha6.es_biciesto(), false);

        assert_eq!(fecha1.es_fecha_valida(), true);
        assert_eq!(fecha4.es_fecha_valida(), true);
        assert_eq!(fecha5.es_fecha_valida(), false);
        assert_eq!(fecha6.es_fecha_valida(), false);

        //fecha1.sumar_dias(365);
        //assert_eq!(fecha1.es_fecha_valida(), true);
        //fecha4.sumar_dias(2);
        //assert_eq!(fecha4.to_string(), "2-3-2020");

        //fecha1.restar_dias(365);
        //assert_eq!(fecha1.es_fecha_valida(), true);
        //fecha4.restar_dias(30);
        //assert_eq!(fecha4.to_string(), "30-1-2020");

        assert_eq!(fecha1.es_mayor(fecha2), true);
        assert_eq!(fecha6.es_mayor(fecha5), false);
    }

    #[test]
    fn test_triangulo(){
        let triangulo1 = tp3::ejercicio4::Triangulo::new(3.0, 4.0, 5.0);
        let triangulo2 = tp3::ejercicio4::Triangulo::new(2.0, 2.0, 2.0);
        let triangulo3 = tp3::ejercicio4::Triangulo::new(2.0, 2.0, 3.0);

        assert_eq!(triangulo1.determinar_tipo(), "Escaleno");
        assert_eq!(triangulo2.determinar_tipo(), "Equilátero");
        assert_eq!(triangulo3.determinar_tipo(), "Isósceles");

        assert_eq!(triangulo1.calcular_area(), 6.0);
        assert_eq!(triangulo2.calcular_area(), 1.7320508075688772); 
        assert_eq!(triangulo3.calcular_area(), 1.984313483298443);

        assert_eq!(triangulo1.calcular_perimetro(), 12.0);
        assert_eq!(triangulo2.calcular_perimetro(), 6.0);
        assert_eq!(triangulo3.calcular_perimetro(), 7.0);
    }

    #[test]
    fn test_producto(){
        let prod1 = tp3::ejercicio5::Producto::new("Producto1".to_string(), 100.0, 1);
        let prod2 = tp3::ejercicio5::Producto::new("Producto2".to_string(), 200.0, 2);
        
        assert_eq!(prod1.calcular_impuestos(21.0), 21.0);
        assert_eq!(prod2.calcular_impuestos(10.0), 20.0);

        assert_eq!(prod1.calcular_descuento(10.0), 10.0);
        assert_eq!(prod2.calcular_descuento(5.0), 10.0);

        assert_eq!(prod1.calcular_precio_final(21.0, 10.0), 111.0);
        assert_eq!(prod2.calcular_precio_final(20.0, 10.0), 220.0);
    }

    #[test]
    fn test_estudiante(){
        let mut est1 = tp3::ejercicio6::Estudiante::new("Lucas".to_string(), 1);
        let mut est2 = tp3::ejercicio6::Estudiante::new("Fede".to_string(), 2);
        let ex1 = tp3::ejercicio6::Examen::new("Matematica".to_string(), 8.0);
        let ex2 = tp3::ejercicio6::Examen::new("Fisica".to_string(), 9.0);
        let ex3 = tp3::ejercicio6::Examen::new("Quimica".to_string(), 7.0);

        est1.examenes.push(ex1);
        est1.examenes.push(ex2);
        est2.examenes.push(ex3);

        assert_eq!(est1.obtener_promedio(), 8.5);
        assert_eq!(est2.obtener_promedio(), 7.0);

        assert_eq!(est1.obtener_calificacion_mas_alta(), 9.0);
        assert_eq!(est2.obtener_calificacion_mas_alta(), 7.0);

        assert_eq!(est1.obtener_calificacion_mas_baja(), 8.0);
        assert_eq!(est2.obtener_calificacion_mas_baja(), 7.0);
    }
}
