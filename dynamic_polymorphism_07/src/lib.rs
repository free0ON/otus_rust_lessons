#[cfg(test)]
mod tests {

    use std::string::ParseError;

    struct NetworkTask {
        id: i32,
    }

    struct DbTask {
        id: i32,
    }

    impl NetworkTask {
        fn new() -> Self {
            Self { id: 1 }
        }
    }

    impl DbTask {
        fn new() -> Self {
            Self { id: 2 }
        }
    }

    trait Task {
        fn execute(&self) -> Result<i32, ParseError>;
        fn get_id(&self) -> i32;
    }

    impl Task for NetworkTask {
        fn execute(&self) -> Result<i32, ParseError> {
            Ok(self.id)
        }

        fn get_id(&self) -> i32 {
            self.id
        }
    }

    impl Task for DbTask {
        fn execute(&self) -> Result<i32, ParseError> {
            Ok(self.id)
        }

        fn get_id(&self) -> i32 {
            self.id
        }
    }

    #[test]
    fn test_trait_objects() {
        let nw = NetworkTask::new();
        let db = DbTask::new();
        let tasks: Vec<&dyn Task> = vec![&nw, &db];
        for task in tasks {
            assert!(task.execute() == Ok(task.get_id()));
        }
    }

    #[test]
    fn test_dts() {
        let nums_array: &[i32; 3] = &[1, 2, 3];
        assert!(8 == size_of_val(&nums_array));
        let nums_slice: &[i32] = &[1, 2, 3];
        assert!(16 == size_of_val(&nums_slice));
    }

    trait Simple {
        fn by_ref(&self) {}
        fn _by_ref_mut(&mut self) {}
    }

    trait ByValue {
        fn _by_value(self)
        where
            Self: Sized,
        {
        }
        fn by_ref(&self) {}
        fn _by_ref_mut(&mut self) {}
    }

    // trait Max {
    //     fn max(&self, other: &Self) -> bool {
    //         true
    //     }
    // }

    trait MaxGeneric<Rhs> {
        fn max(&self, _other: &Rhs) -> bool {
            true
        }
    }

    // trait Foo: Clone {
    //     fn by_ref(&self) {}
    //     fn by_ref_mut(&mut self) {}
    // }

    #[derive(Clone)]
    struct S;
    impl Simple for S {}
    impl ByValue for S {}
    // impl Max for S {}
    impl MaxGeneric<Self> for S {}
    // impl Foo for S {}

    #[test]
    fn test_playground1() {
        //https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=bd5c7765fe3a7a52a2ba317e4a1a938a
        // 1. Simple
        let simple: &dyn Simple = &S;
        simple.by_ref();
        // 2. ByValue
        let by_value: &dyn ByValue = &S;
        by_value.by_ref();
        // by_value.by_value(); // th by_value method cannot be invoked on a trait
        // 3. Max
        // let max: &dyn Max = &S; // the traid is not dyn compatible
        // 3. MaxGeneric
        let max: &dyn MaxGeneric<S> = &S;
        max.max(&S);
        // 3. Foo
        // let max: &dyn Foo = &S; // Clone - Sized
    }

    struct SomeStruct;
    #[test]
    fn test_1() {
        let [x, y] = &mut [SomeStruct, SomeStruct];
        let eq = x as *mut SomeStruct == y as *mut SomeStruct;
        assert!(eq == true);
    }

    trait Trait {
        fn f(&self) -> i32;
    }

    impl Trait for u32 {
        fn f(&self) -> i32 {
            1
        }
    }

    impl<'a> Trait for &'a i32 {
        fn f(&self) -> i32 {
            2
        }
    }

    #[test]
    fn test_2() {
        let x = &1;
        assert!(x.f() == 1);
    }
}
