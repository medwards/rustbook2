use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<'a, T, U, V>
    where T: Fn(&'a U) -> V,
          U: 'a + Eq + Hash
{
    calculation: T,
    values: HashMap<&'a U, V>
}

impl<'a, T, U, V> Cacher<'a, T, U, V>
    where T: Fn(&'a U) -> V,
          U: 'a + Eq + Hash
{
    fn new(calculation: T) -> Cacher<'a, T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: &U) -> &V {
        match self.values.get(arg) {
            Some(ref v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                &v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_cache_hit() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(&1);

        assert_eq!(v1, 1);
    }

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_non_ints() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value("asfd");

        assert_eq!(*v1, "asfd");
    }

    #[test]
    fn call_with_different_inputs_outputs() {
        let mut c = Cacher::new(|a: &String| -> usize { a.len() });

        let val = "asfd".to_string();
        let v1 = c.value(&val);

        assert_eq!(v1, 4 as uszie);
    }
}
