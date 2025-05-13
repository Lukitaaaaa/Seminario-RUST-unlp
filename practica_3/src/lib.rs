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
        let rec1 = tp3::ejercicio2::Rectangulo::new(2, 4);
        let rec2 = tp3::ejercicio2::Rectangulo::new(3, 3);
        
    }
}
