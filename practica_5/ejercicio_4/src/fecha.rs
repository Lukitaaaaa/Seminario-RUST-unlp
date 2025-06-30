#! [allow(unused)]

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
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
                    println!("AUMENTO DE AÑO\n");
                    self.anio += 1;
                    self.mes = 1;
                    dias_mes_actual = 31;
                }
                _ => ()
            }
            if self.dia > dias_mes_actual{ 
                println!("AUMENTO DE MES\n"); 
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