#! [allow(unused)]

use std::collections::HashMap;

#[derive(Debug,PartialEq, Clone, Eq, Hash)]
enum Categoria{
    Electronica,
    Ropa,
    Alimentos,
    Hogar,
    Salud,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Producto{
    nombre: String,
    precio: u32,
    categoria: Categoria,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Cliente {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: u32,
    newsletter_email: Option<String>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Vendedor{
    nro_legajo: u32,
    antiguedad: u32,
    salario: u32,
}

#[derive(Debug,PartialEq, Clone, Eq, Hash)]
enum MedioPago{
    Efectivo,
    TarjetaCredito,
    TarjetaDebito,
    TransferenciaBancaria,
}


struct Venta {
    cliente: Cliente,
    vendedor: Vendedor,
    productos: HashMap<Producto, u32>, 
    medio_pago: MedioPago,
    fecha: String,
}

struct SistemaVentas {
    ventas: Vec<Venta>,
}

impl Venta{
    fn new(cliente: Cliente, vendedor: Vendedor, productos: HashMap<Producto, u32>, medio_pago: MedioPago, fecha: String) -> Self {
        Venta {cliente, vendedor, productos, medio_pago, fecha}
    }

    fn aplicar_descuento(producto:Producto)-> f64{
        //let categoria = producto.categoria;
        match producto.categoria {
            Categoria::Alimentos => producto.precio as f64 * 0.9, // 10% de descuento
            Categoria::Ropa => producto.precio as f64 * 0.8, // 20% de descuento
            Categoria::Electronica => producto.precio as f64 * 0.85, // 15% de descuento
            Categoria::Hogar => producto.precio as f64 * 0.95, // 5% de descuento
            Categoria::Salud => producto.precio as f64, // Sin descuento
        }
    }

    fn calcular_precio_final(&self) -> f64{
        let mut total = 0.0;
        for (producto, cantidad) in &self.productos {
            let precio_con_descuento = Venta::aplicar_descuento(producto.clone());
            total += precio_con_descuento * (*cantidad as f64);
        }

        // Aplica 5% de descuento adicional si el cliente está suscripto al newsletter
        if self.cliente.newsletter_email.is_some() {
            total *= 0.95;
        }
        total
    }

}

impl Producto {
    fn new(nombre: String, precio: u32, categoria: Categoria) -> Self {
        
        Producto {
            nombre,
            precio,
            categoria,
        }
    }
}

impl Vendedor {
    fn new(nro_legajo: u32, antiguedad: u32, salario: u32) -> Self {
        Vendedor {
            nro_legajo,
            antiguedad,
            salario,
        }
    }
}

impl Cliente {
    fn new(nombre: String, apellido: String, direccion: String, dni: u32, newsletter_email: Option<String>) -> Self {
        Cliente {
            nombre,
            apellido,
            direccion,
            dni,
            newsletter_email,
        }
    }
}

impl SistemaVentas {
    fn new() -> Self {
        SistemaVentas {
            ventas: Vec::new(),
        }
    }

    fn crear_venta(&mut self, nro: u32, fecha: String, cliente: Cliente, vendedor: Vendedor, pago: MedioPago, productos: HashMap<Producto, u32>) {
        self.ventas.push(Venta::new(cliente, vendedor, productos, pago, fecha));
    }

    fn ventas_por_categoria(&self, categoria: Categoria) -> u32 {
        let mut total = 0;
        for venta in &self.ventas {
            for producto in venta.productos.keys() {
                if categoria == producto.categoria {
                    total += 1;
                }
            }
        }
        total
    }

    fn ventas_por_vendedor(&self, nro_legajo: u32) -> u32 {
        let mut total = 0;
        for venta in &self.ventas {
            if venta.vendedor.nro_legajo == nro_legajo {
                total += 1;
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_aplicar_descuento() {
        let producto = Producto {
            nombre: String::from("Manzana"),
            precio: 100,
            categoria: Categoria::Alimentos,
        };
        assert_eq!(Venta::aplicar_descuento(producto), 90.0);
        
        let producto = Producto {
            nombre: String::from("Camisa"),
            precio: 200,
            categoria: Categoria::Ropa,
        };
        assert_eq!(Venta::aplicar_descuento(producto), 160.0);
        
        let producto = Producto {
            nombre: String::from("Televisor"),
            precio: 1000,
            categoria: Categoria::Electronica,
        };
        assert_eq!(Venta::aplicar_descuento(producto), 850.0);
    }

    #[test]
    fn test_sistema_calcular_precio_final() {
        let mut sistema = SistemaVentas::new();
        let fecha = "23-10-2023".to_string();
        let cliente = Cliente::new(
            String::from("Juan"),
            String::from("Perez"),
            String::from("Calle Falsa 123"),
            12345678,
            Some(String::from("test@test.com"))
        );
        let vendedor = Vendedor::new(1, 5, 30000);
        let producto = Producto { 
            nombre: String::from("Zapatos"),
            precio: 100,
            categoria: Categoria::Ropa,
        };
        let productos = HashMap::from([(producto.clone(), 1)]);

        //sistema.crear_venta(1, fecha, cliente, vendedor, MedioPago::Efectivo, productos);
        let venta = Venta::new(cliente, vendedor, productos, MedioPago::Efectivo, fecha);
        let precio_final = venta.calcular_precio_final();

        assert_eq!(precio_final, 76.0); 
    }

    #[test]
    fn test_sistema_ventas_por_categoria() {
        let mut sistema = SistemaVentas::new();
        let cliente = Cliente::new(
            String::from("Ana"),
            String::from("Gomez"),
            String::from("Avenida Siempre Viva 456"),
            87654321,
            None
        );
        let vendedor = Vendedor::new(2, 3, 25000);
        
        let producto1 = Producto::new(String::from("Leche"), 50, Categoria::Alimentos);
        let producto2 = Producto::new(String::from("Pantalon"), 150, Categoria::Ropa);
        let producto3 = Producto::new(String::from("Silla"), 200, Categoria::Hogar);
        
        let productos1 = HashMap::from([(producto1.clone(), 2)]);
        let productos2 = HashMap::from([(producto2.clone(), 1)]);
        let productos3 = HashMap::from([(producto3.clone(), 3)]);
        
        sistema.crear_venta(1, "01-01-2023".to_string(), cliente.clone(), vendedor.clone(), MedioPago::Efectivo, productos1);
        sistema.crear_venta(2, "02-01-2023".to_string(), cliente.clone(), vendedor.clone(), MedioPago::TarjetaCredito, productos2);
        sistema.crear_venta(3, "03-01-2023".to_string(), cliente, vendedor, MedioPago::TransferenciaBancaria, productos3);

        assert_eq!(sistema.ventas_por_categoria(Categoria::Alimentos), 1);
        assert_eq!(sistema.ventas_por_categoria(Categoria::Ropa), 1);
        assert_eq!(sistema.ventas_por_categoria(Categoria::Hogar), 1);
    }

    fn test_sistema_ventas_por_vendedor() {
        let mut sistema = SistemaVentas::new();
        let cliente = Cliente::new(
            String::from("Carlos"),
            String::from("Lopez"),
            String::from("Calle Falsa 789"),
            11223344,
            Some(String::from("test"))
        );
        let vendedor1 = Vendedor::new(1, 5, 30000);
        let vendedor2 = Vendedor::new(2, 3, 25000);
        let producto1 = Producto::new(String::from("Televisor"), 500, Categoria::Electronica);
        let producto2 = Producto::new(String::from("Sofa"), 800, Categoria::Hogar);
        let productos1 = HashMap::from([(producto1.clone(), 1)]);
        let productos2 = HashMap::from([(producto2.clone(), 2)]);
        sistema.crear_venta(1, "01-01-2023".to_string(), cliente.clone(), vendedor1.clone(), MedioPago::Efectivo, productos1.clone());
        sistema.crear_venta(2, "02-01-2023".to_string(), cliente.clone(), vendedor2.clone(), MedioPago::TarjetaCredito, productos2);
        sistema.crear_venta(3, "03-01-2023".to_string(), cliente, vendedor1, MedioPago::TransferenciaBancaria, productos1.clone());
        assert_eq!(sistema.ventas_por_vendedor(1), 2);
        assert_eq!(sistema.ventas_por_vendedor(2), 1);
    }
}
