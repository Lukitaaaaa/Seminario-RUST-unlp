trait Primo {
    fn es_primo(&self) -> bool;
}

impl Primo for i32 {
    fn es_primo(&self) -> bool {
        let n = *self;
        if n < 2 {
            return false;
        }
        for i in 2..=((n as f64).sqrt() as i32) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

fn cant_numeros_primos(v: &Vec<i32>) -> i32 {
    v.iter().filter(|&n| n.es_primo()).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_es_primo() {
        assert!(2.es_primo());
        assert!(3.es_primo());
        assert!(!4.es_primo());
        assert!(5.es_primo());
        assert!(!6.es_primo());
        assert!(7.es_primo());
        assert!(!8.es_primo());
    }

    #[test]
    fn test_cant_numeros_primos() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(cant_numeros_primos(&v), 4);
    }

    #[test]
    fn test_cant_numeros_primos_empty() {
        let v: Vec<i32> = vec![];
        assert_eq!(cant_numeros_primos(&v), 0);
    }
}