/*
1- En base al ejercicio 7 del tp#3 implemente lo siguiente: 
a- Al agregar un auto si supera el límite de la concesionaria debe arrojar un error 
propio con un mensaje de contexto. 
b- Haga todos los tests correspondientes para probar en profundidad los métodos 
que agregan un auto y eliminan un auto de la concesionaria , obteniendo el mayor 
porcentaje de coverage sobre el código que realiza las operaciones. 
c- Una vez hecho el punto anterior debe hacer que los autos de la concesionaria se 
almacenen en un archivo en formato JSON. Agregue y modifique lo que considere 
necesario para que: - - 
Al agregar un nuevo auto se abre el archivo de autos guardados y lo agregue a 
dicho archivo. 
Eliminar un auto: al eliminar un auto se debe eliminar este del archivo. 
No debe modificar los tests hechos en el punto b. Si puede agregar más en caso de que 
haga nueva funcionalidad.. 
*/
#![allow(unused)]

use std::fmt::Display;
use serde::{Deserialize, Serialize};
use std::fs::File; 
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq)]
struct ErrorConcesionario(String);
impl Display for ErrorConcesionario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No se puede agregar el auto, capacidad maxima superada: {}", self.0)
    }
}
pub struct ConcesionarioAuto{
    nombre:String,
    direccion:String,
    cantidad_maxima:u32,
    autos: Vec<Auto>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
enum Color{
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Auto{
    marca:String,
    modelo:String,
    anio:u32,
    precio:f32,
    color:Color,
}

impl ConcesionarioAuto{
    //creacion
    pub fn new(nombre:String, direccion:String, cantidad_maxima:u32)-> ConcesionarioAuto{
        let autos: Vec<Auto> = Vec::new();
        ConcesionarioAuto{nombre, direccion, cantidad_maxima, autos}
    }

    fn agregar_auto(&mut self, auto:Auto)-> Result<(), ErrorConcesionario> {

        if self.autos.len() < self.cantidad_maxima as usize{
            self.autos.push(auto);
            self.escribir_autos();
            Ok(())
        } else {
            Err(ErrorConcesionario(self.cantidad_maxima.to_string()))
        }
    }

    fn eliminar_auto(&mut self, auto:Auto){
        self.autos.retain(|a| a != &auto);
        self.escribir_autos();
    }
    
    fn buscar_auto(&self, auto: &Auto) -> Option<&Auto> {
        for a in self.autos.iter() {
            if a == auto {
                return Some(a);
            } 
        }
        None
    }

    fn escribir_autos(&self) {
        match File::create("./autos.json") {
            Ok(mut file) => {
                let b_s = serde_json::to_string_pretty(&self.autos).unwrap();
                file.write_all(&b_s.as_bytes()).expect("Error al escribir el archivo autos.json");
            },
            Err(error) => {
                println!("Error al crear archivo: {error}");
            }
        };
    }
}

impl Auto{
    fn new(marca:String, modelo:String, anio:u32, precio:f32, color:Color)-> Auto{
        Auto{marca, modelo, anio, precio, color}
    }

    fn comparar(&self, otro_auto:Auto)->bool{
        self.color.to_string() == otro_auto.color.to_string() && self.anio == otro_auto.anio && self.marca == otro_auto.marca && self.modelo == otro_auto.modelo && self.precio == otro_auto.precio
    }

    pub fn calcular_precio(&self) -> f32 {
        let mut precio_total = self.precio;
    
        match self.color {
            Color::Rojo | Color::Amarillo | Color::Azul => {
                precio_total *= 1.25;
            },
            _ => {
                precio_total -= precio_total * 0.1;
            }
        }

        if self.marca == String::from("BMW") {
            precio_total *= 1.5;
        }
        if self.anio < 2000 {
            precio_total -= precio_total * 0.05; 
        }

        precio_total
    }
}

impl Color {
    fn to_string(&self) -> String {
        let color_str = match self {
            Color::Verde => "Verde".to_string(),
            Color::Rojo => "Rojo".to_string(),
            Color::Azul => "Azul".to_string(),
            Color::Amarillo => "Amarillo".to_string(),
            Color::Blanco => "Blanco".to_string(),
            Color::Negro => "Negro".to_string(),
        };
        return color_str
    }
}

#[test]
fn test_concesionario_new() {
    let mut c = ConcesionarioAuto::new(String::from("test1"), String::from("test calle"), 50);
 
    assert_eq!(String::from("test1"), c.nombre);
    assert_eq!(String::from("test calle"), c.direccion);
    assert_eq!(50, c.cantidad_maxima);
}

#[test]
fn test_agregar_auto() {
    let mut c = ConcesionarioAuto::new(String::from("test1"), String::from("test calle"), 50);
   
    c.agregar_auto(Auto::new(String::from("Test1"), String::from("Test2"), 2003, 4500.0, Color::Rojo));
    c.agregar_auto(Auto::new(String::from("Test3"), String::from("Test4"), 2007, 3500.0, Color::Amarillo));
    c.agregar_auto(Auto::new(String::from("Test5"), String::from("Test6"), 1998, 6500.0, Color::Blanco));

    assert_eq!(3, c.autos.len());

    let mut c2 = ConcesionarioAuto::new(String::from("test1"), String::from("test calle"), 2);
    c2.agregar_auto(Auto::new(String::from("Test1"), String::from("Test2"), 2000, 2500.0, Color::Verde));
    c2.agregar_auto(Auto::new(String::from("Test3"), String::from("Test4"), 2020, 5250.0, Color::Azul));
    
    let res = c2.agregar_auto(Auto::new(String::from("Test5"), String::from("Test6"), 2021, 10000.0, Color::Rojo));

    assert_eq!(ErrorConcesionario(2.to_string()), res.err().unwrap());
}

#[test]
fn test_eliminar_auto() {
    let mut c = ConcesionarioAuto::new(String::from("test1"), String::from("test calle"), 5);
    c.agregar_auto(Auto::new(String::from("Test1"), String::from("Test2"), 2000, 2500.0, Color::Verde));
    c.agregar_auto(Auto::new(String::from("Test3"), String::from("Test4"), 2020, 5250.0, Color::Azul));
    c.agregar_auto(Auto::new(String::from("Test5"), String::from("Test6"), 2021, 10000.0, Color::Rojo));

    let a = Auto::new(String::from("Test1"), String::from("Test2"), 2000, 2500.0, Color::Verde);

    c.eliminar_auto(a);
    assert_eq!(2, c.autos.len());
}

#[test]
fn test_buscar_auto() {
    let mut c = ConcesionarioAuto::new(String::from("test1"), String::from("test calle"), 5);
    c.agregar_auto(Auto::new(String::from("Test1"), String::from("Test2"), 2000, 2500.0, Color::Verde));
    c.agregar_auto(Auto::new(String::from("Test3"), String::from("Test4"), 2020, 5250.0, Color::Azul));
    c.agregar_auto(Auto::new(String::from("Test5"), String::from("Test6"), 2021, 10000.0, Color::Rojo));
    
    let a = Auto::new(String::from("Test1"), String::from("Test2"), 2000, 2500.0, Color::Verde);
    let n = Auto::new(String::from("Test23"), String::from("Test2"), 2000, 2500.0, Color::Verde);


    assert_eq!(Some(&a.clone()), c.buscar_auto(&a));
    assert_eq!(None, c.buscar_auto(&n));
}

#[test]
fn test_auto_new() {
    let a = Auto::new(String::from("Test1"), String::from("Test2"), 2000, 2500.0, Color::Verde);

    assert_eq!(String::from("Test1"), a.marca);
    assert_eq!(String::from("Test2"), a.modelo);
    assert_eq!(2000, a.anio);
    assert_eq!(2500.0, a.precio);
    assert_eq!(Color::Verde, a.color);
}

#[test]
fn test_auto_calcular_precio() {
    let a = Auto::new(String::from("BMW"), String::from("Test1"), 2005, 2500.0, Color::Verde);
    let a2 = Auto::new(String::from("Test1"), String::from("Test2"), 1998, 3400.0, Color::Azul);

    assert_eq!(3375.0, a.calcular_precio());
    assert_eq!(4037.5, a2.calcular_precio());
 }