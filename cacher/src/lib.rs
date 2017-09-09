use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<T, U, V>
    where T: Fn(U) -> V,
          U: Eq + Hash + Copy,
          V: Copy
{
    calculation: T,
    values: HashMap<U, V>
}

impl<T, U, V> Cacher<T, U, V>
    where T: Fn(U) -> V,
          U: Eq + Hash + Copy,
          V: Copy
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        // Two-part match here is forced by borrow checker
        // self.values immutable borrow in the match, mutable borrow in the None arm
        match self.values.get(&arg) {
            Some(v) => return *v,
            None => (),
        }
        let v = (self.calculation)(arg);
        self.values.insert(arg, v);
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_cache_hit() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);

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

        assert_eq!(v1, "asfd");
    }

    #[test]
    fn call_with_different_inputs_outputs() {
        let mut c = Cacher::new(|a: &str| -> usize { a.len() });

        let v1 = c.value("asfd");

        assert_eq!(v1, 4 as usize);
    }

    #[test]
    fn call_with_string_input() {
        let a1 = String::from("asdf");
        let v1 = {
            let mut c = Cacher::new(|a: &str| -> usize { a.len() });
            c.value(&a1[..])
        };

        assert_eq!(v1, 4 as usize);

        // Switch to Copy to Clone and add clone basically every time arg and v are used
        //let mut c = Cacher::new(|a: String| -> usize { a.len() });
        //let v1 = c.value(String::from("asfd"));
        //assert_eq!(v1, 4 as usize);
    }
}
