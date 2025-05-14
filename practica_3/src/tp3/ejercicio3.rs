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
                    //println!("AUMENTO DE AÑO\n");
                    self.anio += 1;
                    self.mes = 1;
                }
                _ => ()
            }
            if self.dia > dias_mes_actual{ 
                //println!("AUMENTO DE MES\n"); 
                self.mes += 1
            }

            self.dia = self.dia - dias_mes_actual;
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
                }
                _ => ()
            }

            aux = aux + dias_mes_anterior as i32;
        }
        self.dia = aux as u32;
    }

    pub fn es_mayor(&self, fecha: Fecha)->bool{
        if self.anio < fecha.anio{
            return true;
        }else if self.anio == fecha.anio && self.mes < fecha.mes{
            return true;
        }else if self.anio == fecha.anio && self.mes == fecha.mes && self.dia < fecha.dia{
            return true;
        }
        false
    }
}