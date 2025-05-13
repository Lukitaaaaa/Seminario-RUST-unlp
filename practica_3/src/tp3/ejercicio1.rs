/*
1- Escribir un programa que defina una estructura Persona que tenga campos para el 
nombre, la edad y la dirección(que puede ser nulo al momento de la creación de una 
persona). Para dicha estructura implemente los siguientes métodos: 
➢  new: que pasando los parámetros correspondientes, crea una Persona y la retorna. 
➢  to_string: que retorna un string con  los datos de la persona concatenados sobre el 
mensaje ejecutado por ej: 
person.to_string() , donde person es una variable del tipo Persona. 
➢  obtener_edad: retorna la edad de la persona. 
➢  actualizar_direccion(nueva_direccion) 
*/

#[allow(dead_code)]
pub struct Persona{
    pub nombre:String,
    pub edad: u32,
    pub direccion:Option<String>
}

#[warn(dead_code)]
impl Persona{
    pub fn new(nombre: String, edad: u32, direccion: Option<String>)->Persona{
        Persona{nombre, edad, direccion}
    }

    pub fn to_string(&self) -> String{
        //let aux: String = format!("nombre: {}    edad: {}    ", self.nombre, self.edad);
        let aux: String = match &self.direccion{
            Some(dir) => format!("nombre: {}, edad: {}, direccion: {}", self.nombre, self.edad,  dir),
            None => format!("nombre: {}, edad: {}, no tiene direccion", self.nombre, self.edad),
        };  
        return aux;
    }

    pub fn obtener_edad(&self)-> u32{
        return self.edad;
    }

    pub fn actualizar_direccion(&mut self, direccion:Option<String>){
        self.direccion = direccion;
    }
}