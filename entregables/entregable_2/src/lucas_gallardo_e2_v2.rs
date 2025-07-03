/*
 Implementar una funcionalidad que permita obtener un informe de compras realizadas por un cliente específico, filtrando solo aquellas en las que el monto total final de la venta sea mayor a un valor dado.

Este informe debe incluir, ordenado cronológicamente, lo siguiente para cada compra:

-Fecha de la venta

-Productos adquiridos y sus cantidades

-Monto total final de la venta

-Medio de pago utilizado

La consulta se debe realizar a partir de un identificador único del cliente (por ejemplo, su DNI o su correo electrónico, según cómo lo hayan modelado), y un monto mínimo a partir del cual incluir la venta en el informe.

En caso de que el cliente no tenga compras que cumplan esa condición, el sistema debe reflejar esa situación de forma adecuada.

🔧 Esta funcionalidad debe implementarse como un método dentro del struct principal del sistema.

🧪 Además, deben incluir los tests necesarios para verificar el correcto funcionamiento de esta funcionalidad.

📌 Firma esperada del método:

get_historial_compras(id: id_cliente, monto_minimo: f64) -> ???
*/

#! [allow(unused)]

use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub struct Fecha{
    pub dia: u32,
    pub mes: u32,
    pub anio: u32,
}

impl Fecha{
    pub fn new(dia:u32, mes:u32, anio:u32)->Fecha{
        Fecha{dia, mes, anio}
    }
    
    pub fn to_string(&self) -> String{
        let aux: String = format!("{}-{}-{}", self.dia, self.mes, self.anio);
        return aux;
    }

    pub fn es_biciesto(&self) -> bool{
        let aux:bool = (self.anio % 4 == 0) && (self.anio % 100 != 0 || self.anio % 400 == 0);
        aux
    }

    pub fn es_fecha_valida(&self) -> bool{
        let mut aux:bool = (self.mes <13 && self.mes > 0) &&(self.dia < 32 && self.dia > 0);
        
        if aux {
            match self.mes{
                2 => {
                    if self.es_biciesto() {
                        aux = self.dia < 30;
                    }else{
                        aux = self.dia < 29;
                    }
                },
                4 | 6 | 9 | 11 => {
                    aux = self.dia < 31;
                },
                _ => ()
            }
        }
        aux
    }

    pub fn sumar_dias(&mut self, dias:u32) {
        
        self.dia = self.dia + dias;
        let mut dias_mes_actual: u32 = 0;
        
        while !self.es_fecha_valida(){
            match self.mes{
                1 | 3 | 5 | 7 | 8 | 10 | 12 => { dias_mes_actual = 31},
                2 => {
                    if self.es_biciesto() {
                        dias_mes_actual = 29;
                    }else{
                        dias_mes_actual = 28;
                    }
                },
                4 | 6 | 9 | 11 => {dias_mes_actual = 30},
                13 => {
                    self.anio += 1;
                    self.mes = 1;
                    dias_mes_actual = 31;
                }
                _ => ()
            }
            if self.dia > dias_mes_actual{ 
                self.mes += 1;
                self.dia = self.dia - dias_mes_actual;
            }
        }
    }

    pub fn restar_dias(&mut self, dias:u32){
        
        let mut aux:i32 = self.dia as i32 - dias as i32;
        let mut dias_mes_anterior: u32 = 0;


        while aux <= 0 {
            self.mes -= 1;
            match self.mes{
                1 | 3 | 5 | 7 | 8 | 10 | 12 => { dias_mes_anterior = 31},
                2 => {
                    if self.es_biciesto() {
                        dias_mes_anterior = 29;
                    }else{
                        dias_mes_anterior = 28;
                    }
                },
                4 | 6 | 9 | 11 => {dias_mes_anterior = 30},
                0 => {
                    self.anio -= 1;
                    self.mes = 12;
                    dias_mes_anterior = 31;
                }
                _ => ()
            }

            aux = aux + dias_mes_anterior as i32;
        }
        self.dia = aux as u32;
    }

    pub fn es_mayor(&self, fecha: Fecha)->bool{
        if self.anio > fecha.anio{
            return true;
        }else if self.anio == fecha.anio && self.mes > fecha.mes{
            return true;
        }else if self.anio == fecha.anio && self.mes == fecha.mes && self.dia > fecha.dia{
            return true;
        }
        false
    }
}

#[test]
fn test_new_fecha() {
    let fecha = Fecha::new(1, 1, 2020);
    assert_eq!(fecha.dia, 1);
    assert_eq!(fecha.mes, 1);
    assert_eq!(fecha.anio, 2020);
}

#[test]
fn test_to_string() {
    let fecha = Fecha::new(1, 1, 2020);
    assert_eq!(fecha.to_string(), "1-1-2020");
}

#[test]
fn test_es_biciesto() {
    let fecha = Fecha::new(29, 2, 2020);
    assert!(fecha.es_biciesto());
    let fecha_no_biciesto = Fecha::new(29, 2, 2021);
    assert!(!fecha_no_biciesto.es_biciesto());
}

#[test]
fn test_es_fecha_valida() {
    let fecha_valida = Fecha::new(29, 2, 2020);
    assert!(fecha_valida.es_fecha_valida());
    let fecha_no_valida = Fecha::new(30, 2, 2021);
    assert!(!fecha_no_valida.es_fecha_valida());
}

#[test]
fn test_sumar_dias() {
    let mut fecha = Fecha::new(28, 2, 2020);
    fecha.sumar_dias(1);
    assert_eq!(fecha.to_string(), "29-2-2020");
    fecha.sumar_dias(2);
    assert_eq!(fecha.to_string(), "2-3-2020");
    
    let mut fecha2 = Fecha::new(31, 12, 2020);
    fecha2.sumar_dias(1);
    assert_eq!(fecha2.to_string(), "1-1-2021");

    let mut fecha3 = Fecha::new(30, 1, 2020);
    fecha3.sumar_dias(1);
    assert_eq!(fecha3.to_string(), "31-1-2020");
}

#[test]
fn test_restar_dias() {
    let mut fecha = Fecha::new(1, 3, 2020);
    fecha.restar_dias(1);
    assert_eq!(fecha.to_string(), "29-2-2020");
    fecha.restar_dias(28);
    assert_eq!(fecha.to_string(), "1-2-2020");
    
    let mut fecha2 = Fecha::new(1, 1, 2021);
    fecha2.restar_dias(1);
    assert_eq!(fecha2.to_string(), "31-12-2020");

    let mut fecha3 = Fecha::new(1, 2, 2020);
    fecha3.restar_dias(1);
    assert_eq!(fecha3.to_string(), "31-1-2020");

    let mut fecha4 = Fecha::new(1, 3, 2022);
    fecha4.restar_dias(1);
    assert_eq!(fecha4.to_string(), "28-2-2022");

    let mut fecha5 = Fecha::new(1, 5, 2020);
    fecha5.restar_dias(1);
    assert_eq!(fecha5.to_string(), "30-4-2020");
}

#[test]
fn test_es_mayor() {
    let fecha1 = Fecha::new(3, 2, 2020);
    let fecha2 = Fecha::new(2, 2, 2020);
    assert!(fecha1.es_mayor(fecha2));
    
    let fecha3 = Fecha::new(3, 1, 2019);
    assert!(fecha1.es_mayor(fecha3));

    let fecha4 = Fecha::new(3, 1, 2020);
    assert!(fecha1.es_mayor(fecha4));

    let fecha5 = Fecha::new(4, 2, 2020);
    assert!(!fecha1.es_mayor(fecha5));
    
    let fecha6 = Fecha::new(1, 1, 2021);
    assert!(!fecha1.es_mayor(fecha6));
}


use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq, Clone)]
struct ErrorHistorialCompras(String, String);

impl Display for ErrorHistorialCompras {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No hay compras del cliente {} que superen el monto mínimo de {}", self.0, self.1)
    }
}

#[derive(Debug,PartialEq, Clone, Eq, Hash)]
enum Categoria{
    Electronica,
    Ropa,
    Alimentos,
    Hogar,
    Salud,
}

#[derive(Clone, Debug, PartialEq)]
struct Compra {
    fecha_inf: Fecha,
    productos_vendidos: HashMap<Producto, u32>, 
    monto_final: f64,
    medio_pago: MedioPago,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct DatosPersonales{
    nombre: String,
    apellido: String,
    direccion: String,
    dni: u32,
}

#[derive(Clone, Debug, PartialEq)]
struct Producto{
    nombre: String,
    precio: f64,
    categoria: Categoria,
}

impl Eq for Producto {}

impl Hash for Producto {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.nombre.hash(state);
        self.precio.to_bits().hash(state);
        self.categoria.hash(state);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Cliente {
    datos: DatosPersonales,
    newsletter_email: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Vendedor{
    datos: DatosPersonales,
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

#[derive(Debug, PartialEq, Clone)]
struct Venta {
    cliente: Cliente,
    vendedor: Vendedor,
    productos: HashMap<Producto, u32>, 
    medio_pago: MedioPago,
    fecha: Fecha,
}

struct SistemaVentas {
    ventas: Vec<Venta>,
}

impl Compra{
    fn new(fecha_inf: Fecha, productos_vendidos: HashMap<Producto, u32>, monto_final: f64, medio_pago: MedioPago) -> Self {
        Compra { fecha_inf, productos_vendidos, monto_final, medio_pago }
    } 
}

impl Venta{
    fn new(cliente: Cliente, vendedor: Vendedor, productos: HashMap<Producto, u32>, medio_pago: MedioPago, fecha: Fecha) -> Self {
        Venta {cliente, vendedor, productos, medio_pago, fecha}
    }

    fn aplicar_descuento(producto:Producto)-> f64{
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

        if self.cliente.newsletter_email.is_some() {
            total *= 0.95;
        }
        total
    }

}

impl Producto {
    fn new(nombre: String, precio: f64, categoria: Categoria) -> Self {
        
        Producto {
            nombre,
            precio,
            categoria,
        }
    }
}

impl Vendedor {
    fn new(datos:DatosPersonales, nro_legajo: u32, antiguedad: u32, salario: u32) -> Self {
        Vendedor {
            datos,
            nro_legajo,
            antiguedad,
            salario,
        }
    }
}

impl Cliente {
    fn new(datos: DatosPersonales, newsletter_email: Option<String>) -> Self {
        Cliente {
            datos,
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

    fn get_historial_compras(&self, dni: u32, monto_minimo: f64) -> Result<Vec<Compra>, ErrorHistorialCompras> {
        let ventas_del_cliente: Vec<&Venta>  = self.ventas.iter().filter(|venta| venta.cliente.datos.dni == dni.clone() && venta.calcular_precio_final() >= monto_minimo).collect();
        let mut compras: Vec<Compra> = vec![];

        if ventas_del_cliente.is_empty() {
            return Err(ErrorHistorialCompras(dni.to_string(),monto_minimo.to_string()));
        }

        let mut compras: Vec<Compra> = ventas_del_cliente.iter().map(|venta| {
            Compra::new(
                venta.fecha.clone(),
                venta.productos.clone(),
                venta.calcular_precio_final(),
                venta.medio_pago.clone(),
            )
        }).collect();

        compras.sort_by(|a,b| a.fecha_inf.cmp(&b.fecha_inf));
        
        Ok(compras)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_aplicar_descuento() {
        let producto: Producto = Producto {
            nombre: String::from("Manzana"),
            precio: 100.0,
            categoria: Categoria::Alimentos,
        };
        assert_eq!(Venta::aplicar_descuento(producto), 90.0);
        
        let producto = Producto {
            nombre: String::from("Camisa"),
            precio: 200.0,
            categoria: Categoria::Ropa,
        };
        assert_eq!(Venta::aplicar_descuento(producto), 160.0);
        
        let producto = Producto {
            nombre: String::from("Televisor"),
            precio: 1000.0,
            categoria: Categoria::Electronica,
        };
        assert_eq!(Venta::aplicar_descuento(producto), 850.0);
    }

    #[test]
    fn test_sistema_calcular_precio_final() {
        let mut sistema = SistemaVentas::new();
        let fecha = Fecha::new(23, 10, 2023);
        
        let datos_c: DatosPersonales = Default::default();
        let cliente = Cliente::new(datos_c.clone(), Some(String::from("test@test.com")));
        
        let datos_v: DatosPersonales = Default::default();
        let vendedor = Vendedor::new(datos_v.clone(),1, 5, 30000);
        
        let producto = Producto::new(String::from("Zapatos"), 100.0, Categoria::Ropa);
    
        let productos = HashMap::from([(producto.clone(), 1)]);

        //sistema.crear_venta(1, fecha, cliente, vendedor, MedioPago::Efectivo, productos);
        let venta = Venta::new(cliente, vendedor, productos, MedioPago::Efectivo, fecha);
        let precio_final = venta.calcular_precio_final();

        assert_eq!(precio_final, 76.0); 
    }

    #[test]
    fn test_sistema_ventas_por_categoria() {
        let mut sistema = SistemaVentas::new();
        let datos_c: DatosPersonales = Default::default();
        let cliente = Cliente::new(datos_c.clone(),None);

        let datos_v: DatosPersonales = Default::default();
        let vendedor = Vendedor::new(datos_v.clone(),2, 3, 25000);
        
        let producto1 = Producto::new(String::from("Leche"), 50.0, Categoria::Alimentos);
        let producto2 = Producto::new(String::from("Pantalon"), 150.0, Categoria::Ropa);
        let producto3 = Producto::new(String::from("Silla"), 200.0, Categoria::Hogar);
        
        let mut productos = HashMap::new();
        productos.insert(producto1, 2 );
        productos.insert(producto2, 1);
        productos.insert(producto3, 3);

        let fecha = Fecha::new(30, 6, 2023);

        let venta1 = Venta::new(cliente.clone(), vendedor.clone(), productos, MedioPago::Efectivo, fecha.clone());
        
        sistema.ventas.push(venta1);

        assert_eq!(sistema.ventas_por_categoria(Categoria::Alimentos), 1);
        assert_eq!(sistema.ventas_por_categoria(Categoria::Ropa), 1);
        assert_eq!(sistema.ventas_por_categoria(Categoria::Hogar), 1);
    }

    #[test]
    fn test_sistema_ventas_por_vendedor() {
        let mut sistema = SistemaVentas::new();

        let datos_c: DatosPersonales = Default::default();
        let cliente = Cliente::new( datos_c.clone(),Some(String::from("test")));

        let datos_v1: DatosPersonales = Default::default();
        let vendedor1 = Vendedor::new(datos_v1.clone(),1, 5, 30000);
        
        let datos_v2: DatosPersonales = Default::default();
        let vendedor2 = Vendedor::new(datos_v2.clone(),2, 3, 25000);
        
        let producto1 = Producto::new(String::from("Televisor"), 500.0, Categoria::Electronica);
        let producto2 = Producto::new(String::from("Sofa"), 800.0, Categoria::Hogar);
        
        let productos1 = HashMap::from([(producto1.clone(), 1)]);
        let productos2 = HashMap::from([(producto2.clone(), 2)]);
        
        let ventas1 = Venta::new(cliente.clone(), vendedor1.clone(), productos1, MedioPago::Efectivo, Fecha::new(1, 1, 2023));
        let ventas2 = Venta::new(cliente.clone(), vendedor2.clone(), productos2, MedioPago::TarjetaCredito, Fecha::new(2, 1, 2023));

        sistema.ventas.push(ventas1.clone());
        sistema.ventas.push(ventas1.clone()); // Agregamos otra venta del mismo vendedor para verificar el conteo
        sistema.ventas.push(ventas2);

        sistema.ventas_por_vendedor(1);
        sistema.ventas_por_vendedor(2);

        assert_eq!(sistema.ventas_por_vendedor(1), 2);
        assert_eq!(sistema.ventas_por_vendedor(2), 1);
    }

    #[test]
    fn test_get_historial_compras() {
        let mut sistema = SistemaVentas::new();
        
        let datos_c1: DatosPersonales = DatosPersonales { nombre: "lucas".to_string(), apellido: "gallardo".to_string(), direccion: "calle 1".to_string(), dni: 22222222 };
        let cliente1 = Cliente::new(datos_c1.clone(),Some(String::from("lucas@gamil.com")));

        let datos_c2: DatosPersonales = Default::default();
        let cliente2 = Cliente::new(datos_c2.clone(),Some(String::from("lean@gmail.com")));

        let datos_v: DatosPersonales = Default::default();
        let vendedor = Vendedor::new(datos_v.clone(),1, 5, 30000);
        
        let producto1 = Producto::new(String::from("Laptop"), 1000.0, Categoria::Electronica);
        let producto2 = Producto::new(String::from("Camisa"), 200.0, Categoria::Ropa);
        
        let productos1 = HashMap::from([(producto1.clone(), 1)]);
        let productos2 = HashMap::from([(producto2.clone(), 3)]);
        let productos3 = HashMap::from([(producto1.clone(), 2), (producto2.clone(), 1)]);
        let productos4 = HashMap::from([(producto2.clone(), 1)]);

        let venta1 = Venta::new(cliente1.clone(), vendedor.clone(), productos1, MedioPago::Efectivo, Fecha::new(1, 1, 2024));
        let venta2 = Venta::new(cliente1.clone(), vendedor.clone(), productos2, MedioPago::TarjetaCredito, Fecha::new(2, 1, 2023));
        let venta3 = Venta::new(cliente2.clone(), vendedor.clone(), productos3, MedioPago::Efectivo, Fecha::new(3, 1, 2023));   
        let venta4 = Venta::new(cliente2.clone(), vendedor.clone(), productos4, MedioPago::TransferenciaBancaria, Fecha::new(4, 1, 2023));

        sistema.ventas.push(venta1);
        sistema.ventas.push(venta2);    

        let resultado = sistema.get_historial_compras(cliente1.datos.dni, 100.0);        
        assert!(resultado.is_ok());
        
        let mut sistema2 = SistemaVentas::new();
        sistema2.ventas.push(venta3);
        sistema2.ventas.push(venta4); 
        let resultado2 = sistema2.get_historial_compras(cliente2.datos.dni, 2000.0);
        assert_eq!(ErrorHistorialCompras(0.to_string(),2000.0.to_string()), resultado2.err().unwrap());
    }
}
