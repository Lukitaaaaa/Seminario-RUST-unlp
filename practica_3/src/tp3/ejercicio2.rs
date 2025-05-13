pub struct Rectangulo{
    pub logintud: f32,
    pub ancho: f32,
}

impl Rectangulo{
    fn new(longitud:f32, ancho:f32)->Rectangulo{
        Rectangulo{longitud, ancho}
    }

    fn calcular_area(self)-> f32{
        self.logintud * self.ancho;
    }

    fn calcular_perimetro(self) -> f32{
        (self.logintud + self.ancho) * 2;
    }

    fn es_cuadrado(self) -> bool{
        self.longitud == self.ancho;
    }
}