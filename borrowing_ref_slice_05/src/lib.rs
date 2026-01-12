#[cfg(test)]
mod tests {

    #[test]
    fn test_ref() {
        let v = [1, 2, 3];
        let immut_ref1: &i32 = &v[1];
        let immut_ref2: &i32 = &v[1];
        assert_eq!(immut_ref1, &v[1]);
        assert_eq!(immut_ref2, &v[1]);
        // error
        // let mut_ref: &mut i32 = &mut v[0];
        // where is can be only one mutable ref or many immutable ref
        let mut v_mut = [1, 2, 3];
        let mut_ref: &mut i32 = &mut v_mut[0];
        *mut_ref = 10;
        assert_eq!(v_mut[0], 10);
    }

    #[test]
    fn test_aliasing() {
        let mut a = 1;
        let mut b = 2;
        swap(&mut a, &mut b);
        assert!(a == 2 && b == 1);
    }

    #[allow(clippy::manual_swap)]
    fn swap(x: &mut i32, y: &mut i32) {
        *x ^= *y;
        *y ^= *x;
        *x ^= *y;
    }

    #[test]
    fn test_slices() {
        let v = vec![1, 2, 3, 4, 5];
        let all: &[i32] = &v[..];
        let first: &[i32] = &v[..1];
        let all_but_first: &[i32] = &v[1..];
        let middle: &[i32] = &v[2..3];

        assert!(all == v);
        assert!(first[0] == v[0]);
        assert!(all_but_first[0] == v[1]);
        assert!(middle == [3]);

        let string: &str = "Hello, Мир!";

        assert!(string.split_at(5).0 == "Hello");
        assert!(string.split_at(5).1 == ", Мир!");
        assert!(string.len() == 14);
        assert!(string.chars().count() == 11);
    }

    struct SomeStruct<'a> {
        some_ref: &'a i32,
    }

    struct LargeStruct<'a> {
        smaller: SomeStruct<'a>,
    }

    #[test]
    fn test_lifetime() {
        let x = 10; // 'a
        let large_struct = LargeStruct {
            smaller: SomeStruct { some_ref: &x },
        };
        let r = large_struct.smaller.some_ref;
        assert!(*r == x);
        assert!(longest("Hello", "world") == "Hello");
    }

    fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() >= s2.len() {
            s1
        } else {
            s2
        }
    }

    struct Storage<'a, T> {
        inner: Vec<&'a T>,
    }

    impl<'a, T> Storage<'a, T> {
        fn new() -> Self {
            Self {
                inner: Default::default(),
            }
        }

        fn get(&self, index: usize) -> Option<&'a T> {
            // 'a
            self.inner.get(index).cloned()
        }

        fn push(&mut self, value: &'a T) {
            // 'a
            self.inner.push(value);
        }
    }

    #[test]
    fn test_playground1() {
        let (x, y) = (10, 20);
        let ref_x = {
            let mut storage = Storage::<i32>::new();
            storage.push(&x);
            storage.push(&y);
            match storage.get(0) {
                Some(ref_x) => ref_x,
                _ => unreachable!(),
            }
        };
        assert!(*ref_x == 10); // вы должны увидеть "10" в консоли
    }

    #[test]
    fn test_who_is_owner() {
        let str: &str = &Box::new("Hello".to_string());
        println!("{str}");
        assert!(str == "Hello");
    }

    struct Connection<'a> {
        value: &'a Value,
    }

    struct Value {
        data: i32,
    }

    struct Db<'a> {
        inner: &'a Connection<'a>,
    }

    impl<'a> Db<'a> {
        fn new(inner: &'a Connection) -> Self {
            Self { inner }
        }
        fn get_value<'b>(&'b self) -> Option<&'b Value> {
            Some(self.inner.value)
        }
    }

    #[test]
    fn test_db_lifetime() {
        let db = Db::new(&Connection {
            value: &Value { data: 42 },
        });
        assert!(db.get_value().expect("Error").data == 42);
    }
}
