/*
3 -La plataforma de streaming "StreamingRust" ofrece distintos tipos de suscripciones
(Basic, Clasic, Super) a sus usuarios. Cada suscripción tiene un costo mensual y una
duración de meses y una fecha de inicio, además los usuarios pueden pagar por sus
suscripciones con distintos medios de pago que son Efectivo, MercadoPago, Tarjeta de
Crédito, Transferencia Bancaria, Cripto. Cada medio de pago tiene sus datos
correspondientes a excepción de Efectivo.

Los usuarios solo pueden tener una suscripción activa a la vez.
Implemente las estructuras, funciones asociadas y traits necesarios para resolver las

siguientes acciones:

➢ Crear un usuario con una determinada suscripción y medio de pago.
➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic
pasa a Clasic y si está en Clasic pasa a Super.
➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la
suscripción es del tipo Basic al hacerlo se cancelará la suscripción.
➢ Dado un usuario cancelar la suscripción.
➢ Saber el medio de pago que es más utilizado por los usuarios sobre las
suscripciones activas
➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones
activas.
➢ Saber cuál fue el medio de pago más utilizado.
➢ Saber cuál fue la suscripción más contratada.
*/

#![allow(unused)]

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum SubscriptionTipo {
    Basic,
    Classic,
    Super,
}

#[derive(Debug, Clone, PartialEq)]
struct Subscription {
    tipo: SubscriptionTipo,
    costo_mensual: f64,
    duracion_meses: u32,
    fecha_inicio: String, 
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MedioPagoTipo {
    Efectivo,
    MercadoPago,
    TarjetaCredito,
    TransferenciaBancaria,
    Cripto,
}

#[derive(Debug, Clone, PartialEq)]
struct MedioPago {
    nombre: Option<String>,
    tipo: MedioPagoTipo,
}

#[derive(Debug, Clone)]
struct Usuario {
    nombre: String,
    id: u32,
    suscripcion: Option<Subscription>,
    medio_pago: MedioPago,
}

#[derive(Debug, Clone)]
struct PlataformaStreaming {
    usuarios: Vec<Usuario>,
}

use chrono::{NaiveDate, Datelike, Duration, Utc};

impl Subscription {
    fn new(tipo: SubscriptionTipo, costo_mensual: f64, duracion_meses: u32, fecha_inicio: String) -> Self {
        Subscription {
            tipo,
            costo_mensual,
            duracion_meses,
            fecha_inicio,
        }
    }

    fn es_activa(&self) -> bool {
        let inicio = match NaiveDate::parse_from_str(&self.fecha_inicio, "%Y-%m-%d") {
            Ok(fecha) => fecha,
            Err(_) => return false,
        };
        let vencimiento = inicio
            .with_month(inicio.month() + self.duracion_meses)
            .unwrap_or_else(|| inicio + Duration::days(30 * self.duracion_meses as i64));
        let hoy = Utc::now().naive_utc().date();
        hoy <= vencimiento
    }
}

impl Usuario {
    fn new(nombre: String, id: u32, suscripcion: Subscription, medio_pago: MedioPago) -> Self {
        Usuario {
            nombre,
            id,
            suscripcion: Some(suscripcion),
            medio_pago,
        }
    }

    fn upgrade(&mut self) {
        if let Some(ref mut suscripcion) = self.suscripcion {
            suscripcion.tipo = match suscripcion.tipo {
                SubscriptionTipo::Basic => SubscriptionTipo::Classic,
                SubscriptionTipo::Classic => SubscriptionTipo::Super,
                SubscriptionTipo::Super => SubscriptionTipo::Super, // No puede subir más
            };
        }
    }

    fn downgrade(&mut self) {
        if let Some(ref mut suscripcion) = self.suscripcion {
            if suscripcion.tipo == SubscriptionTipo::Basic {
                self.suscripcion = None; // Cancelar la suscripción
            } else {
                suscripcion.tipo = match suscripcion.tipo {
                    SubscriptionTipo::Classic => SubscriptionTipo::Basic,
                    SubscriptionTipo::Super => SubscriptionTipo::Classic,
                    _ => suscripcion.tipo, // No puede bajar más
                };
            }
        }
    }

    fn cancelar_suscripcion(&mut self) {
        self.suscripcion = None;
    }
}

impl PlataformaStreaming {
    fn new() -> Self {
        PlataformaStreaming {
            usuarios: Vec::new(),
        }
    }

    fn agregar_usuario(&mut self, usuario: Usuario) {
        self.usuarios.push(usuario);
    }

    fn medio_pago_mas_utilizado(&self) -> Option<MedioPagoTipo> {
        let mut conteo = std::collections::HashMap::new();
        for usuario in &self.usuarios {
            if let Some(ref suscripcion) = usuario.suscripcion {
                *conteo.entry(usuario.medio_pago.tipo.clone()).or_insert(0) += 1;
            }
        }
        conteo.into_iter().max_by_key(|&(_, count)| count).map(|(tipo, _)| tipo)
    }

    fn suscripcion_mas_contratada(&self) -> Option<SubscriptionTipo> {
        let mut conteo = std::collections::HashMap::new();
        for usuario in &self.usuarios {
            if let Some(ref suscripcion) = usuario.suscripcion {
                *conteo.entry(suscripcion.tipo.clone()).or_insert(0) += 1;
            }
        }
        conteo.into_iter().max_by_key(|&(_, count)| count).map(|(tipo, _)| tipo)
    }

    fn medio_pago_mas_utilizado_subscripciones_activas(&self) -> Option<MedioPagoTipo> {
        let mut conteo = std::collections::HashMap::new();
        for usuario in &self.usuarios {
            if let Some(ref suscripcion) = usuario.suscripcion {
                if suscripcion.es_activa() {
                    *conteo.entry(usuario.medio_pago.tipo.clone()).or_insert(0) += 1;
                }
            }
        }
        conteo.into_iter().max_by_key(|&(_, count)| count).map(|(tipo, _)| tipo)
    }

    fn suscripcion_mas_contratada_activas(&self) -> Option<SubscriptionTipo> {
        let mut conteo = std::collections::HashMap::new();
        for usuario in &self.usuarios {
            if let Some(ref suscripcion) = usuario.suscripcion {
                if suscripcion.es_activa() {
                    *conteo.entry(suscripcion.tipo.clone()).or_insert(0) += 1;
                }
            }
        }
        conteo.into_iter().max_by_key(|&(_, count)| count).map(|(tipo, _)| tipo)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usuario_upgrade() {
        let mut usuario = Usuario::new(
            "Juan".to_string(),
            1,
            Subscription::new(SubscriptionTipo::Basic, 10.0, 1, "2023-01-01".to_string()),
            MedioPago { nombre: None, tipo: MedioPagoTipo::Efectivo },
        );
        usuario.upgrade();
        assert_eq!(usuario.suscripcion.as_ref().unwrap().tipo, SubscriptionTipo::Classic);
        usuario.upgrade();
        assert_eq!(usuario.suscripcion.unwrap().tipo, SubscriptionTipo::Super);
    }

    #[test]
    fn test_usuario_downgrade() {
        let mut usuario = Usuario::new(
            "Juan".to_string(),
            1,
            Subscription::new(SubscriptionTipo::Super, 10.0, 1, "2023-01-01".to_string()),
            MedioPago { nombre: None, tipo: MedioPagoTipo::Efectivo },
        );
        usuario.downgrade();
        assert_eq!(usuario.suscripcion.as_ref().unwrap().tipo, SubscriptionTipo::Classic);
        usuario.downgrade();
        assert_eq!(usuario.suscripcion.as_ref().unwrap().tipo, SubscriptionTipo::Basic);
        usuario.downgrade();
        assert!(usuario.suscripcion.is_none());
    }

    #[test]
    fn test_usuario_cancelar_suscripcion() {
        let mut usuario = Usuario::new(
            "Juan".to_string(),
            1,
            Subscription::new(SubscriptionTipo::Basic, 10.0, 1, "2023-01-01".to_string()),
            MedioPago { nombre: None, tipo: MedioPagoTipo::Efectivo },
        );
        usuario.cancelar_suscripcion();
        assert!(usuario.suscripcion.is_none());
    }

    #[test]
    fn test_es_activa(){
        let subscription = Subscription::new(SubscriptionTipo::Basic, 10.0, 1, "2025-06-01".to_string());
        assert!(subscription.es_activa());
        assert_eq!(subscription.es_activa(), true);
        
        let expired_subscription = Subscription::new(SubscriptionTipo::Basic, 10.0, 1, "2022-01-01".to_string());
        assert!(!expired_subscription.es_activa());
        assert_eq!(expired_subscription.es_activa(), false);
    }

    #[test]
    fn test_plataforma_medio_pago_mas_utilizado() {
        let mut plataforma = PlataformaStreaming::new();
        plataforma.agregar_usuario(Usuario::new(
            "Juan".to_string(),
            1,
            Subscription::new(SubscriptionTipo::Basic, 10.0, 1, "2023-01-01".to_string()),
            MedioPago { nombre: None, tipo: MedioPagoTipo::Efectivo },
        ));
        plataforma.agregar_usuario(Usuario::new(
            "Ana".to_string(),
            2,
            Subscription::new(SubscriptionTipo::Classic, 20.0, 1, "2023-01-01".to_string()),
            MedioPago { nombre: None, tipo: MedioPagoTipo::TarjetaCredito },
        ));
        plataforma.agregar_usuario(Usuario::new(
            "Luis".to_string(),
            3,
            Subscription::new(SubscriptionTipo::Super, 30.0, 1, "2023-01-01".to_string()),
            MedioPago { nombre: None, tipo: MedioPagoTipo::Efectivo },
        ));
        assert_eq!(plataforma.medio_pago_mas_utilizado(), Some(MedioPagoTipo::Efectivo));
    }

    #[test]
    fn test_plataforma_suscripcion_mas_contratada() {
        let mut plataforma = PlataformaStreaming::new();
        plataforma.agregar_usuario(Usuario::new(
            "Juan".to_string(),
            1,
            Subscription::new(SubscriptionTipo::Basic, 10.0, 1, "2023-01-01".to_string()),
            MedioPago { nombre: None, tipo: MedioPagoTipo::Efectivo },
        ));
        plataforma.agregar_usuario(Usuario::new(
            "Ana".to_string(),
            2,
            Subscription::new(SubscriptionTipo::Classic, 20.0, 1, "2023-01-01".to_string()),
            MedioPago { nombre: None, tipo: MedioPagoTipo::TarjetaCredito },
        ));
        plataforma.agregar_usuario(Usuario::new(
            "Luis".to_string(),
            3,
            Subscription::new(SubscriptionTipo::Super, 30.0, 1, "2023-01-01".to_string()),
            MedioPago { nombre: None, tipo: MedioPagoTipo::Efectivo },
        ));
        plataforma.agregar_usuario(Usuario::new(
            "Roberto".to_string(),
            4,
            Subscription::new(SubscriptionTipo::Basic, 10.0, 1, "2023-01-01".to_string()),
            MedioPago { nombre: None, tipo: MedioPagoTipo::Efectivo },
        ));
        assert_eq!(plataforma.suscripcion_mas_contratada(), Some(SubscriptionTipo::Basic));
    }

    #[test]
    fn test_plataforma_medio_pago_mas_utilizado_subs_activas() {
        let mut plataforma = PlataformaStreaming::new();
        plataforma.agregar_usuario(Usuario::new(
            "Juan".to_string(),
            1,
            Subscription::new(SubscriptionTipo::Basic, 10.0, 1, "2025-06-01".to_string()),
            MedioPago { nombre: None, tipo: MedioPagoTipo::Efectivo },
        ));
        plataforma.agregar_usuario(Usuario::new(
            "Ana".to_string(),
            2,
            Subscription::new(SubscriptionTipo::Classic, 20.0, 1, "2025-06-01".to_string()),
            MedioPago { nombre: None, tipo: MedioPagoTipo::TarjetaCredito },
        ));
        plataforma.agregar_usuario(Usuario::new(
            "Luis".to_string(),
            3,
            Subscription::new(SubscriptionTipo::Super, 30.0, 1, "2025-06-01".to_string()),
            MedioPago { nombre: None, tipo: MedioPagoTipo::Efectivo },
        ));
        assert_eq!(plataforma.medio_pago_mas_utilizado_subscripciones_activas(), Some(MedioPagoTipo::Efectivo));
    }

    #[test]
    fn test_plataforma_suscripcion_mas_contratada_activas() {
        let mut plataforma = PlataformaStreaming::new();
        plataforma.agregar_usuario(Usuario::new(
            "Juan".to_string(),
            1,
            Subscription::new(SubscriptionTipo::Basic, 10.0, 1, "2025-06-01".to_string()),
            MedioPago { nombre: None, tipo: MedioPagoTipo::Efectivo },
        ));
        plataforma.agregar_usuario(Usuario::new(
            "Ana".to_string(),
            2,
            Subscription::new(SubscriptionTipo::Classic, 20.0, 1, "2025-06-01".to_string()),
            MedioPago { nombre: None, tipo: MedioPagoTipo::TarjetaCredito },
        ));
        plataforma.agregar_usuario(Usuario::new(
            "Luis".to_string(),
            3,
            Subscription::new(SubscriptionTipo::Super, 30.0, 1, "2025-06-01".to_string()),
            MedioPago { nombre: None, tipo: MedioPagoTipo::Efectivo },
        ));
        plataforma.agregar_usuario(Usuario::new(
            "Roberto".to_string(),
            4,
            Subscription::new(SubscriptionTipo::Basic, 10.0, 1, "2025-06-01".to_string()),
            MedioPago { nombre: None, tipo: MedioPagoTipo::Efectivo },
        ));
        assert_eq!(plataforma.suscripcion_mas_contratada_activas(), Some(SubscriptionTipo::Basic));
    }
}