/*
7- Defina una estructura llamada ConcesionarioAuto donde se conoce el nombre, la 
dirección y tiene una capacidad máxima para albergar X cantidad de autos. De los autos se 
conocen los campos de la marca, modelo, año, precio bruto y  color que pueden ser:rojo, 
verde, azul, amarillo, blanco o negro. 
Para dichas estructuras implemente los siguientes métodos: 
❖     ConcesionarioAuto: 
➢ new: que pasando los parámetros correspondientes, crea un 
ConcesionarioAuto y lo retorna. 
➢ agregar_auto(auto): agrega un auto a la lista de autos que tiene sin superar 
la máxima cantidad para albergarlos y retorna true, en caso de que lo supere 
no lo agrega y retorna false. 
➢ eliminar_auto(auto): elimina un auto de la lista de autos. 
➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna. 
❖     Auto: 
➢ new: que pasando los parámetros correspondientes, crea un Auto y lo 
retorna. 
➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios: 
■ si es de color primario le aplica un recargo del 25%, sino le aplica un 
descuento del 10%. 
■ si la marca es BMW le aplica un recargo del 15%- 
■ si el año es menor a 2000 le aplica un descuento del 5%.
*/

pub struct ConcesionarioAuto{
    nombre:String,
    direccion:String,
    cantidad_maxima:u32,
    autos: Vec<Auto>,
}

enum Color{
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

pub struct Auto{
    marca:String,
    modelo:String,
    anio:u32,
    precio:f32,
    color:Color,
}

impl ConcesionarioAuto{
    //creacion
    pub fn new(nombre:String, direccion:String, cantidad_maxima:u32, autos: Vec<Auto>)-> ConcesionarioAuto{
        ConcesionarioAuto{nombre, direccion, cantidad_maxima, autos}
    }

    pub fn agregar_auto(&mut self, auto:Auto)->bool{
        if self.autos.len() < self.cantidad_maxima{
            self.autos.push(auto);
            return true
        }
        false 
    }

    fn eliminar_auto(&mut self, auto:Auto){
        for i in 0..self.autos.len(){
            if auto.comparar(&self.autos[i]){
                self.autos.remove(i);
            }
        }
    }
    
    fn buscar_auto(&mut self, auto:&Auto) -> Option<Auto>{
        for i in 0..self.autos.len(){
            if auto.comparar(&self.autos[i])  {
                self.autos.get(i);
            }
        }
        None
    }
}

impl Auto{
    pub fn comparara(&self, otro_auto:Auto)->bool{
        self.color.to_string() == other.color.to_string() && self.año == other.año && self.marca == other.marca && self.modelo == other.modelo && self.precio_bruto == other.precio_bruto
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