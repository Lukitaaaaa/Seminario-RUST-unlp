/*
6- Escribir un programa que defina una estructura Estudiante que tenga campos para el 
nombre, el número de identificación y las calificaciones de exámenes. De cada Examen se 
conoce el nombre de la materia y la nota. Para dichas estructuras implemente los siguientes 
métodos: 
❖ Examen: 
➢ new: que pasando los parámetros correspondientes, crea un Examen y lo 
retorna. 
❖ Estudiante: 
➢ new: que pasando los parámetros correspondientes, crea un Estudiante y lo 
retorna. 
➢ obtener_promedio: retorna el promedio de las notas. 
➢ obtener_calificacion_mas_alta: retorna la nota más alta. 
➢ obtener_calificacion_mas_baja: retorna la nota más baja.
*/


pub struct Estudiante {
    pub nombre: String,
    pub id: u32,
    pub examenes: Vec<Examen>,
}
pub struct Examen{
    pub materia: String,
    pub nota: f32,
}

impl Estudiante {
    pub fn new(nombre: String, id: u32) -> Estudiante {
        Estudiante {
            nombre,
            id,
            examenes: Vec::new(),
        }
    }

    pub fn obtener_promedio(&self) -> f32 {
        let suma: f32 = self.examenes.iter().map(|ex| ex.nota).sum();
        let cantidad = self.examenes.len() as f32;
        if cantidad > 0.0 {
            suma / cantidad
        } else {
            0.0
        }
    }

    pub fn obtener_calificacion_mas_alta(&self) -> f32 {
        let mut calificacion_mas_alta = 0.0;
        for ex in &self.examenes {
            if ex.nota > calificacion_mas_alta {
                calificacion_mas_alta = ex.nota;
            }
        }
        calificacion_mas_alta
    }

    pub fn obtener_calificacion_mas_baja(&self) -> f32 {
        let mut calificacion_mas_baja = f32::MAX;
        for ex in &self.examenes {
            if ex.nota < calificacion_mas_baja {
                calificacion_mas_baja = ex.nota;
            }
        }
        calificacion_mas_baja
    }
}

impl Examen {
    pub fn new(materia: String, nota: f32) -> Examen {
        Examen { materia, nota }
    }
}