pub struct Rectangulo{
    pub longitud: f32,
    pub ancho: f32,
}

impl Rectangulo{
    pub fn new(longitud:f32, ancho:f32)->Rectangulo{
        Rectangulo{longitud, ancho}
    }

    pub fn calcular_area(&self)-> f32{
        let area =self.longitud * self.ancho;
        area
    }

    pub fn calcular_perimetro(&self) -> f32{
        let perimetro = (self.longitud + self.ancho) * 2.0;
        perimetro
    }

    pub fn es_cuadrado(&self) -> bool{
        let aux:bool = self.longitud == self.ancho;
        aux
    }
}