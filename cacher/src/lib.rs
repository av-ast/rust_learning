use std::collections::HashMap;

pub struct Cacher<T>
where
    T: Fn(u32) -> u32
{
    calculation: T,
    values: HashMap<u32, u32>
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation, values: HashMap::new() }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                println!("Missed cache. Invoking calculation with arg: {}", arg);
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut cacher = Cacher::new(|arg| {
            println!("Calculation body");
            arg + 5
        });

        assert_eq!(6, cacher.value(1));
        assert_eq!(6, cacher.value(1));
    }
}
