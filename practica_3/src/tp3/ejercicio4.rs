/*
4- Escribir un programa que defina la estructura Triángulo que tenga campos para las 
longitudes de sus tres lados. Para dicha estructura implemente los siguientes métodos: 
➢ new: que pasando los parámetros correspondientes, crea un Triángulo y lo retorna. 
➢ determinar_tipo: retorna el tipo del triángulo, los tipos pueden ser equilátero, 
isósceles o escaleno. 
➢ calcular_area: calcular el área y la retorna. 
➢ calcular_perimetro: calcula el perímetro y lo retorna.
*/

pub struct Triangulo {
    pub lado1: f64,
    pub lado2: f64,
    pub lado3: f64,    
}

impl Triangulo {
    pub fn new(lado1: f64, lado2: f64, lado3:f64) -> Triangulo {
        Triangulo { lado1, lado2, lado3 }
    }

    pub fn determinar_tipo(&self) -> String {
        if self.lado1 == self.lado2 && self.lado2 == self.lado3 {
            "Equilátero".to_string()
        } else if self.lado1 == self.lado2 || self.lado1 == self.lado3 || self.lado2 == self.lado3 {
            "Isósceles".to_string()
        } else {
            "Escaleno".to_string()
        }
    }

    pub fn calcular_area(&self) -> f64 {
        let s = (self.lado1 + self.lado2 + self.lado3) / 2.0;
        (s * (s - self.lado1) * (s - self.lado2) * (s - self.lado3)).sqrt()
    }

    pub fn calcular_perimetro(&self) -> f64 {
        self.lado1 + self.lado2 + self.lado3
    }
}