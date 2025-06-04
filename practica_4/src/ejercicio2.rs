#[derive(Debug, Clone)]
struct Persona<'a>{
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8,
}

fn punto_a(v:Vec<Persona>, salario:f64) -> Vec<Persona> {
    v.into_iter()
        .filter(|p| p.salario > salario)
        .collect()
} 

fn punto_b<'a>(v: &'a Vec<Persona>, edad: u8, ciudad: &'a str) -> Vec<&'a Persona<'a>> {
    v.iter().filter(|&p| p.edad > edad && p.ciudad == ciudad).collect()
}

fn punto_c(v: Vec<Persona>, ciudad: &str) -> bool {
    v.iter().all(|p| p.ciudad == ciudad)
}

fn punto_d(v: Vec<Persona>, ciudad: &str) -> bool {
    v.iter().any(|p| p.ciudad == ciudad)
}

fn punto_e(v: Vec<Persona>, persona: &Persona) -> bool {
    v.iter().any(|p| {
        p.nombre == persona.nombre &&
        p.apellido == persona.apellido &&
        p.direccion == persona.direccion &&
        p.ciudad == persona.ciudad &&
        p.salario == persona.salario &&
        p.edad == persona.edad
    })
}

fn punto_f(v: Vec<Persona>) -> Vec<u8> {
    v.iter()
        .map(|p| p.edad)
        .collect()
}

fn punto_g<'a>(v: &'a [Persona<'a>]) -> Option<(&'a Persona<'a>, &'a Persona<'a>)> {
    if v.is_empty() {
        return None;
    }

    let menor = v.iter().min_by(|a, b| {
        a.salario
            .partial_cmp(&b.salario)
            .unwrap()
            .then_with(|| b.edad.cmp(&a.edad)) // Mayor edad desempata
    })?;

    let mayor = v.iter().max_by(|a, b| {
        a.salario
            .partial_cmp(&b.salario)
            .unwrap()
            .then_with(|| b.edad.cmp(&a.edad)) // Mayor edad desempata
    })?;
    Some((menor, mayor))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_punto_a() {
        let personas = vec![
            Persona { nombre: "Alice", apellido: "Smith", direccion: "123 Main St", ciudad: "Springfield", salario: 50000.0, edad: 30 },
            Persona { nombre: "Bob", apellido: "Johnson", direccion: "456 Elm St", ciudad: "Springfield", salario: 60000.0, edad: 40 },
            Persona { nombre: "Charlie", apellido: "Brown", direccion: "789 Oak St", ciudad: "Shelbyville", salario: 45000.0, edad: 35 },
            Persona { nombre: "Diana", apellido: "Prince", direccion: "321 Maple St", ciudad: "Springfield", salario: 70000.0, edad: 28 },
        ];
        let resultado = punto_a(personas, 49999.0);
        assert_eq!(resultado.len(), 3);
        assert_eq!(resultado[0].nombre, "Alice");
    }

    #[test]
    fn test_punto_b() {
        let personas = vec![
            Persona { nombre: "Alice", apellido: "Smith", direccion: "123 Main St", ciudad: "Springfield", salario: 50000.0, edad: 30 },
            Persona { nombre: "Bob", apellido: "Johnson", direccion: "456 Elm St", ciudad: "Shelbyville", salario: 60000.0, edad: 40 },
        ];
        let resultado = punto_b(&personas, 25, "Springfield");
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0].nombre, "Alice");
    }

    #[test]
    fn test_punto_c() {
        let personas = vec![
            Persona { nombre: "Alice", apellido: "Smith", direccion: "123 Main St", ciudad: "Springfield", salario: 50000.0, edad: 30 },
            Persona { nombre: "Bob", apellido: "Johnson", direccion: "456 Elm St", ciudad: "Springfield", salario: 60000.0, edad: 40 },
        ];
        
        let personas2 = vec![
            Persona { nombre: "Charlie", apellido: "Brown", direccion: "789 Oak St", ciudad: "Shelbyville", salario: 45000.0, edad: 35 },
            Persona { nombre: "Diana", apellido: "Prince", direccion: "321 Maple St", ciudad: "Springfield", salario: 70000.0, edad: 28 },
        ];
        assert_eq!(punto_c(personas.clone(), "Springfield"), true);
        assert_eq!(punto_c(personas2, "Springfield"), false);
    }

    #[test]
    fn test_punto_d() {
        let personas = vec![
            Persona { nombre: "Alice", apellido: "Smith", direccion: "123 Main St", ciudad: "Springfield", salario: 50000.0, edad: 30 },
            Persona { nombre: "Bob", apellido: "Johnson", direccion: "456 Elm St", ciudad: "Shelbyville", salario: 60000.0, edad: 40 },
        ];
        
        let personas2 = vec![
            Persona { nombre: "Charlie", apellido: "Brown", direccion: "789 Oak St", ciudad: "Shelbyville", salario: 45000.0, edad: 35 },
            Persona { nombre: "Diana", apellido: "Prince", direccion: "321 Maple St", ciudad: "Springfield", salario: 70000.0, edad: 28 },
        ];
        assert_eq!(punto_d(personas.clone(), "Springfield"), true);
        assert_eq!(punto_d(personas2, "Springfield"), true);
    }

    #[test]
    fn test_punto_e() {
        let persona = Persona { nombre: "Alice", apellido: "Smith", direccion: "123 Main St", ciudad: "Springfield", salario: 50000.0, edad: 30 };
        let personas = vec![
            persona.clone(),
            Persona { nombre: "Bob", apellido: "Johnson", direccion: "456 Elm St", ciudad: "Shelbyville", salario: 60000.0, edad: 40 },
        ];
        
        let personas2 = vec![
            Persona { nombre: "Charlie", apellido: "Brown", direccion: "789 Oak St", ciudad: "Shelbyville", salario: 45000.0, edad: 35 },
            Persona { nombre: "Diana", apellido: "Prince", direccion: "321 Maple St", ciudad: "Springfield", salario: 70000.0, edad: 28 },
        ];

        assert_eq!(punto_e(personas.clone(), &persona), true);
        assert_eq!(punto_e(personas2, &persona), false);
    }

    #[test]
    fn test_punto_f() {
        let personas = vec![
            Persona { nombre: "Alice", apellido: "Smith", direccion: "123 Main St", ciudad: "Springfield", salario: 50000.0, edad: 30 },
            Persona { nombre: "Bob", apellido: "Johnson", direccion: "456 Elm St", ciudad: "Shelbyville", salario: 60000.0, edad: 40 },
        ];
        
        let edades = punto_f(personas);
        assert_eq!(edades.len(), 2);
        assert_eq!(edades[0], 30);
        assert_eq!(edades[1], 40);
    }

    #[test]
    fn test_punto_g() {
        let personas = vec![
            Persona { nombre: "Alice", apellido: "Smith", direccion: "123 Main St", ciudad: "Springfield", salario: 50000.0, edad: 30 },
            Persona { nombre: "Bob", apellido: "Johnson", direccion: "456 Elm St", ciudad: "Shelbyville", salario: 60000.0, edad: 40 },
            Persona { nombre: "Charlie", apellido: "Brown", direccion: "789 Oak St", ciudad: "Springfield", salario: 45000.0, edad: 35 },
        ];
        
        let resultado = punto_g(&personas);
        assert!(resultado.is_some());
        let (menor, mayor) = resultado.unwrap();
        assert_eq!(menor.nombre, "Charlie");
        assert_eq!(mayor.nombre, "Bob");
    }
    // Otros tests para los puntos b, c, d, e, f y g...
}