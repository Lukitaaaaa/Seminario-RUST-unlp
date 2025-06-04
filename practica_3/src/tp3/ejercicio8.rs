enum Genero{
    Rock,
    Pop,
    Jazz,
    Rap,
}

pub struct Cancion {
    titulo: String,
    genero: Genero,
}

pub struct Playlist {
    nombre: String,
    canciones: Vec<Cancion>,
}

impl Playlist{
    pub fn new(nombre: String) -> Playlist {
        Playlist { nombre, canciones: Vec::new() }
    }

    pub fn agregar_cancion(&mut self, cancion: Cancion) {
        self.canciones.push(cancion);
    }

    fn eliminar_cancion(&mut self,cancion:Cancion){
        let genero_str = cancion.genero.to_string();
        for i in 0..self.canciones.len(){
            if self.canciones[i].artista == cancion.artista 
            && self.canciones[i].genero.to_string() == genero_str 
            && self.canciones[i].titulo == cancion.titulo{
                self.canciones.remove(i);
                break;
            }
        }
    }

    fn mover_cancion(&mut self,cancion:Cancion,new_position:usize){
        if new_position >= 0 && new_position <= self.canciones.len()-1{ 
            let genero_str = cancion.genero.to_string();
            for i in 0..self.canciones.len(){
                if self.canciones[i].artista == cancion.artista 
                && self.canciones[i].genero.to_string() == genero_str 
                && self.canciones[i].titulo == cancion.titulo{
                    let cancion_to_move= self.canciones.remove(i);
                    self.canciones.insert(new_position, cancion_to_move);
                }
            }
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
    }

}

impl Genero{
    fn to_string(&self) -> String{
        let str_genero = match self{
            Genero::Jazz => "Jazz".to_string(),
            Genero::Others => "Otro".to_string(),
            Genero::Pop => "Pop".to_string(),
            Genero::Rap => "Rap".to_string(),
            Genero::Rock => "Rock".to_string()
        };
        str_genero
    }
}
