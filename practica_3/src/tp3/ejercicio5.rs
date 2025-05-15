/*
5- Escribir un programa que defina una estructura Producto que tenga campos para el 
nombre, el precio bruto y un número identificatorio. Para dicha estructura implemente los 
siguientes métodos: 
➢ new: que pasando los parámetros correspondientes, crea un Producto y lo retorna. 
➢ calcular_impuestos(porcentaje_de_impuestos): retorna el valor de impuestos sobre 
el precio bruto 
➢ aplicar_descuento(porcentaje_de_descuento): retorna el valor del porcentaje de 
descuento sobre el precio bruto 
➢ calcular_precio_total(porcentaje_de_impuestos, porcentaje_descuento): retorna el 
precio total a pagar aplicando impuesto y descuento. Tenga en cuenta que los 
parámetros son opcionales. 
*/

pub struct Producto{
    nombre:String,
    precio:f32,
    id:u32,
} 

impl Producto{
    pub fn new(nombre:String, precio:f32, id:u32) -> Producto{
        Producto{nombre, precio, id}
    }
    
    pub fn calcular_impuestos(&self, porcentaje:f32) -> f32{
        let impuestos:f32 = (self.precio * porcentaje) / 100.0;
        return impuestos;
    }

    pub fn calcular_descuento(&self, porcentaje:f32) -> f32{
        let descuento:f32 = (self.precio * porcentaje) / 100.0;
        return descuento;
    }
    
    pub fn calcular_precio_final(&self, porcentaje_impuesto:f32, porcentaje_descuento:f32) -> f32{
        let precio_final:f32 = self.precio + self.calcular_impuestos(porcentaje_impuesto) - self.calcular_descuento(porcentaje_descuento);
        return precio_final;
    }
}