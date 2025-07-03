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

TP3 - Ejercicio 6 - Funcionalidad adicional

Deberán agregar una funcionalidad al ejercicio que permita retornar un informe detallado del rendimiento académico de un estudiante.

Este informe debe incluir:
Nombre e identificación del estudiante.
Cantidad total de exámenes rendidos.
Promedio general de notas.
Nota más alta y la materia correspondiente.
Nota más baja y la materia correspondiente.

🔧 La funcionalidad deberá implementarse como un método asociado del estudiante llamado generar_informe.
En caso de que el estudiante no haya rendido ningún examen, no debe retornarse ningún informe.

📌 Requisitos:
La funcionalidad debe integrarse naturalmente con las estructuras ya definidas.
Se espera una solución robusta ante distintas situaciones, incluyendo estudiantes sin exámenes.
Se debe acompañar con al menos dos tests unitarios que validen su correcto funcionamiento.

*/
#![allow(unused)]

#[derive(Clone)]
pub struct Informe{
    pub nombre:String,
    pub id:u32,
    cantidad_examenes:u32,
    pub promedio:f32,
    pub calificacion_mas_alta:Examen,
    pub calificacion_mas_baja:Examen,

}
pub struct Estudiante {
    pub nombre: String,
    pub id: u32,
    pub examenes: Vec<Examen>,
}

#[derive(Clone)]
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

    fn generar_informe(&self) -> Option<Informe> {
        let promedio = self.obtener_promedio();
        let calificacion_mas_alta = self.obtener_calificacion_mas_alta();
        let calificacion_mas_baja = self.obtener_calificacion_mas_baja();
        
        let mut examen_mas_alta = Examen::new("Materia".to_string(), 0.0);
        let mut examen_mas_baja = Examen::new("Materia".to_string(), f32::MAX);
        for i in self.examenes.iter() {
            if i.nota == calificacion_mas_alta {
                examen_mas_alta.materia = i.materia.clone();
                examen_mas_alta.nota = i.nota;
            }
            if i.nota == calificacion_mas_baja {
                examen_mas_baja.materia = i.materia.clone();
                examen_mas_baja.nota = i.nota;
            }
        }

        if self.examenes.is_empty() {
            return None;
        }
        
        let informe= Informe {
            nombre: self.nombre.clone(),
            id: self.id,
            cantidad_examenes: self.examenes.len() as u32,
            promedio,
            calificacion_mas_alta: examen_mas_alta,
            calificacion_mas_baja: examen_mas_baja,
        };
        return Some(informe);
    }
}

impl Examen {
    pub fn new(materia: String, nota: f32) -> Examen {
        Examen { materia, nota }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estudiante(){
        let mut est1 = Estudiante::new("Lucas".to_string(), 1);
        let mut est2 = Estudiante::new("Fede".to_string(), 2);
        let ex1 = Examen::new("Matematica".to_string(), 8.0);
        let ex2 = Examen::new("Fisica".to_string(), 9.0);
        let ex3 = Examen::new("Quimica".to_string(), 7.0);

        est1.examenes.push(ex1);
        est1.examenes.push(ex2);
        est2.examenes.push(ex3);

        assert_eq!(est1.obtener_promedio(), 8.5);
        assert_eq!(est2.obtener_promedio(), 7.0);

        assert_eq!(est1.obtener_calificacion_mas_alta(), 9.0);
        assert_eq!(est2.obtener_calificacion_mas_alta(), 7.0);

        assert_eq!(est1.obtener_calificacion_mas_baja(), 8.0);
        assert_eq!(est2.obtener_calificacion_mas_baja(), 7.0);
    }

    #[test]
    fn test_informe(){
        let mut est1 = Estudiante::new("Lucas".to_string(), 1);
        let est2 = Estudiante::new("Jose".to_string(), 1);

        let ex1 = Examen::new("Matematica".to_string(), 8.0);
        let ex2 = Examen::new("Fisica".to_string(), 9.0);

        est1.examenes.push(ex1);
        est1.examenes.push(ex2);

        let informe1: Option<Informe> = est1.generar_informe();
        let informe2: Option<Informe> = est2.generar_informe();
        assert!(informe1.is_some());
        
        assert_eq!(informe2.is_none(), true);
        assert_eq!(informe1.is_none(), false);

    }

    //test para verificar que los exmaenes con la calificacion mas alta y la mas baja sea la correspondiete
    #[test]
    fn test_informe_calificaciones(){
        let mut est1 = Estudiante::new("Lucas".to_string(), 1);
        let ex1 = Examen::new("Matematica".to_string(), 8.0);
        let ex2 = Examen::new("Fisica".to_string(), 9.0);
        let ex3 = Examen::new("Quimica".to_string(), 7.0);

        est1.examenes.push(ex1);
        est1.examenes.push(ex2);
        est1.examenes.push(ex3);

        let informe: Option<Informe> = est1.generar_informe();
        let informe = informe.unwrap();
        assert_eq!(informe.calificacion_mas_alta.nota, 9.0);
        assert_eq!(informe.calificacion_mas_baja.nota, 7.0);
        assert_eq!(informe.calificacion_mas_alta.materia, "Fisica");
        assert_eq!(informe.calificacion_mas_baja.materia, "Quimica");
    }
}