/*

2- En base al ejercicio 8 del tp#3 implemente lo siguiente:
a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%
b- Una vez obtenido dicho coverage, las canciones de la playlist deben ser
guardadas en un archivo en formato JSON, por lo tanto las operaciones que agreguen,
quiten o modifiquen la playlist deben estar respaldadas sobre dicho archivo.
No debe modificar los tests hechos en el punto a. Si puede agregar más en caso de que
haga métodos nuevos. Recuerde también que se debe seguir manteniendo un coverage de
al menos 90%,
*/
#![allow(unused)]

use std::fmt::Display;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
struct PosInvalida(String);

impl Display for PosInvalida {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "La posicion {} en la que se movio la cancion es invalida", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
enum Genero{
    Rock,
    Pop,
    Jazz,
    Rap,
    Otros,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

pub struct Playlist {
    nombre: String,
    canciones: Vec<Cancion>,
}

impl Cancion {
    fn new(titulo: String, artista: String, genero: Genero) -> Cancion {
        Cancion { titulo, artista, genero }
    }
}

impl Playlist{
    pub fn new(nombre: String) -> Playlist {
        Playlist { nombre, canciones: Vec::new() }
    }

    fn escribir_json(&self) {
        let c_s = serde_json::to_string(&self.canciones).unwrap();
        let mut f = File::create("playlist.json").unwrap();
        f.write_all(&c_s.as_bytes());
    }

    pub fn agregar_cancion(&mut self, cancion: Cancion) {
        self.canciones.push(cancion);
        self.escribir_json();
    }

    fn eliminar_cancion(&mut self,cancion:Cancion){
        let genero_str = cancion.genero.to_string();
        for i in 0..self.canciones.len(){
            if self.canciones[i].artista == cancion.artista 
            && self.canciones[i].genero.to_string() == genero_str 
            && self.canciones[i].titulo == cancion.titulo{
                self.canciones.remove(i);
                self.escribir_json();
                break;
            }
        }
    }

    fn mover_cancion(&mut self,cancion:Cancion,pos:usize) -> Result<(), PosInvalida>{
        if pos-1 <= self.canciones.len()-1{ 
            let genero_str = cancion.genero.to_string();
            for i in 0..self.canciones.len(){
                if self.canciones[i].artista == cancion.artista 
                && self.canciones[i].genero.to_string() == genero_str 
                && self.canciones[i].titulo == cancion.titulo{
                    let cancion_to_move= self.canciones.remove(i);
                    self.canciones.insert(pos-1, cancion_to_move);
                    self.escribir_json();
                }
            }
            Ok(())
        }else{
            Err(PosInvalida(pos.to_string()))
        }
    }


    fn buscar_cancion_por_nombre(&self,nombre:String) -> Option<&Cancion>{
        for i in 0..self.canciones.len(){
            if self.canciones[i].titulo == nombre{
                return Some(&self.canciones[i]);
            }
        }
        return None;
    }

    fn filtrar_canciones_por_genero(&self,genero:Genero) -> Vec<&Cancion>{
        let genero_str = genero.to_string();
        let mut canciones_encontradas =  Vec::new();
        for i in 0..self.canciones.len(){
            if self.canciones[i].genero.to_string() == genero_str{
                canciones_encontradas.push(&self.canciones[i]);
            }
        }
        canciones_encontradas
    }

    fn filtrar_canciones_por_artista(&self,artista:String) -> Vec<&Cancion>{
        let mut canciones_encontradas =  Vec::new();
        for i in 0..self.canciones.len(){
            if self.canciones[i].artista == artista{
                canciones_encontradas.push(&self.canciones[i]);
            }
        }
        canciones_encontradas
    }

    fn modificar_titulo(&mut self,nuevo_titulo:String){
        self.nombre = nuevo_titulo;
    }
    
    fn eliminar_todas_canciones(&mut self){
        self.canciones.clear();
        self.escribir_json();
    }

}

impl Genero{
    fn to_string(&self) -> String{
        let str_genero = match self{
            Genero::Jazz => "Jazz".to_string(),
            Genero::Otros => "Otro".to_string(),
            Genero::Pop => "Pop".to_string(),
            Genero::Rap => "Rap".to_string(),
            Genero::Rock => "Rock".to_string()
        };
        str_genero
    }
}

#[test]
fn test_cancion_new() {
    let c = Cancion::new(String::from("Test1"), String::from("Test2"), Genero::Otros);

    assert_eq!(String::from("Test1"), c.titulo);
    assert_eq!(String::from("Test2"), c.artista);
    assert_eq!(Genero::Otros, c.genero);
}

#[test]
fn test_playlist_new() {
    let p = Playlist::new(String::from("Test1"));

    assert_eq!(String::from("Test1"), p.nombre);
    assert_eq!(true, p.canciones.is_empty());    
}

#[test]
fn test_playlist_agregar_cancion() {
    let mut p = Playlist::new(String::from("Test1"));
    let c = Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Otros);
    
    assert_eq!(true, p.canciones.is_empty());    
    p.agregar_cancion(c);
    
    assert_eq!(1, p.canciones.len());
    assert_eq!(String::from("Test2"), p.canciones[0].titulo);
}

#[test]
fn test_playlist_eliminar_cancion() {
    let mut p = Playlist::new(String::from("Test1"));
    let c = Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Otros);
    p.agregar_cancion(Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Otros));

    p.eliminar_cancion(c);

    assert_eq!(true, p.canciones.is_empty());
}

#[test]
fn test_playlist_mover_cancion() {
    let mut p = Playlist::new(String::from("Test1"));
    let c = Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Otros);
    p.agregar_cancion(Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Otros));
    p.agregar_cancion(Cancion::new(String::from("Test4"), String::from("Test5"), Genero::Pop));
    
    let err = p.mover_cancion(c.clone(), 10);
    assert_eq!(err.unwrap_err().to_string(), "La posicion 10 en la que se movio la cancion es invalida");
    p.mover_cancion(c.clone(), 2);
    assert_eq!(c, p.canciones[1]);
    assert_ne!(c, p.canciones[0]);
}

#[test]
fn test_playlist_buscar_nombre() {
    let mut p = Playlist::new(String::from("Test1"));
    p.agregar_cancion(Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Otros));
    p.agregar_cancion(Cancion::new(String::from("Test4"), String::from("Test5"), Genero::Rock));
    p.agregar_cancion(Cancion::new(String::from("Test6"), String::from("Test7"), Genero::Jazz));
    
    let c = p.buscar_cancion_por_nombre(String::from("Test6"));
    let c2 = p.buscar_cancion_por_nombre(String::from("No existe"));
    assert_eq!(&p.canciones[2], c.unwrap());
    assert_eq!(None, c2);
}

#[test]
fn test_playlist_obtener_genero() {
    let mut p = Playlist::new(String::from("Test1"));
    p.agregar_cancion(Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Jazz));
    p.agregar_cancion(Cancion::new(String::from("Test4"), String::from("Test5"), Genero::Rock));
    p.agregar_cancion(Cancion::new(String::from("Test6"), String::from("Test7"), Genero::Otros));
    p.agregar_cancion(Cancion::new(String::from("Test8"), String::from("Test9"), Genero::Rock));
    p.agregar_cancion(Cancion::new(String::from("Test10"), String::from("Test11"), Genero::Jazz));
    p.agregar_cancion(Cancion::new(String::from("Test12"), String::from("Test13"), Genero::Rock));
    
    let rock = p.filtrar_canciones_por_genero(Genero::Rock);
    let pop = p.filtrar_canciones_por_genero(Genero::Pop);
    assert_eq!(3, rock.len());
    assert_eq!(true, pop.is_empty());
}

#[test]
fn test_playlist_obtener_canciones_artista() {
    let mut p = Playlist::new(String::from("Test1"));
    p.agregar_cancion(Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Jazz));
    p.agregar_cancion(Cancion::new(String::from("Test4"), String::from("Test3"), Genero::Rock));
    p.agregar_cancion(Cancion::new(String::from("Test6"), String::from("Test5"), Genero::Otros));
    p.agregar_cancion(Cancion::new(String::from("Test8"), String::from("Test3"), Genero::Rock));
    p.agregar_cancion(Cancion::new(String::from("Test10"), String::from("Test11"), Genero::Jazz));
    p.agregar_cancion(Cancion::new(String::from("Test12"), String::from("Test13"), Genero::Rock));
    
    let test3 = p.filtrar_canciones_por_artista("Test3".to_string());
    let test5 = p.filtrar_canciones_por_artista("Test5".to_string());
    let test222 = p.filtrar_canciones_por_artista("Test222".to_string());

    assert_eq!(3, test3.len());
    assert_eq!(1, test5.len());
    assert!(test222.is_empty());
}


#[test]
fn test_playlist_modificar_titulo() {
    let mut p = Playlist::new(String::from("Test1"));

    p.modificar_titulo(String::from("Playlist musica"));

    assert_eq!(String::from("Playlist musica"), p.nombre);
}

#[test]
fn test_playlist_eliminar_canciones() {
    let mut p = Playlist::new(String::from("Test1"));
    p.agregar_cancion(Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Jazz));
    p.agregar_cancion(Cancion::new(String::from("Test4"), String::from("Test5"), Genero::Rock));
    p.agregar_cancion(Cancion::new(String::from("Test6"), String::from("Test7"), Genero::Otros));
    p.agregar_cancion(Cancion::new(String::from("Test8"), String::from("Test9"), Genero::Rock));

    p.eliminar_todas_canciones();
    assert_eq!(true, p.canciones.is_empty());
}