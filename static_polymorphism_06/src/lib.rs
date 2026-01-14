#[cfg(test)]
mod tests {
    use std::{f32::consts::PI, fmt};

    pub trait Say {
        fn say(&self) -> &str {
            "Hello!"
        }
    }

    struct Dog;
    impl Say for Dog {
        fn say(&self) -> &str {
            "Woof!"
        }
    }

    struct Human;
    impl Say for Human {}

    pub trait HasId {
        const ID: usize;
        fn get_id(&self) -> usize;
    }

    impl HasId for Human {
        const ID: usize = 10;
        fn get_id(&self) -> usize {
            Self::ID
        }
    }

    pub trait FromStr {
        // type Err;
        // fn from_str_result(s: &str) -> Result<Self, Self::Err>;
        fn from_str(s: &str) -> Self;
    }

    impl FromStr for String {
        // type Err = Infallible;
        fn from_str(s: &str) -> Self {
            s.to_string()
        }
        // fn from_str_result(s: &str) -> Result<Self, Self::Err> {
        //     Ok(s.to_string())
        // }
    }
    // #[derive(PartialEq)]
    // enum MyEnum {
    //     SomeVariant,
    //     DefautlVariant,
    // }

    // trait Default {
    //     fn default() -> Self;
    // }

    // impl Default for MyEnum {
    //     fn default() -> Self {
    //         MyEnum::DefautlVariant
    //     }
    // }

    // struct S;
    // impl Default for S {
    //     fn default() -> Self {
    //         S {}
    //     }
    // }
    // impl Into<T> for S {
    //     fn into() {}
    // }

    #[test]
    fn test_trait() {
        let dog = Dog;
        assert!(dog.say() == "Woof!");

        let man = Human;
        assert!(man.say() == "Hello!");

        assert!(man.get_id() == 10);
        // #[allow(clippy::assertions_on_constants)]
        assert!(Human::ID == 10);
        assert!(String::from_str("Hello") == "Hello".to_string());

        // let def_enum = Default::default();
        // match def_enum {
        //     MyEnum::SomeVariant => assert!(def_enum == MyEnum::SomeVariant),
        //     _ => assert!(def_enum == MyEnum::DefautlVariant),
        // }
        // assert!(def_enum == Default::default());
        // let def = Default::default();
        // let s = S::default();
        // assert!(s == S::default());
    }

    trait Area {
        fn area(&self) -> f32;
    }

    struct Rectangle {
        width: f32,
        height: f32,
    }

    impl Area for Rectangle {
        fn area(&self) -> f32 {
            self.width * self.height
        }
    }

    struct Circle {
        radius: f32,
    }

    impl Area for Circle {
        fn area(&self) -> f32 {
            PI * self.radius.powi(2)
        }
    }

    struct RightTriangle {
        base: f32,
        height: f32,
    }

    impl Area for RightTriangle {
        fn area(&self) -> f32 {
            0.5 * self.base * self.height
        }
    }

    #[test]
    fn test_playground1() {
        // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=36c246e46dc8261365ce162c849288a1
        let rect = Rectangle {
            width: 100.0,
            height: 38.8,
        };
        assert!((rect.area() - 100.0 * 38.8).abs() < f32::EPSILON);

        let circle = Circle { radius: 15.0 };
        assert!((circle.area() - PI * 15.0 * 15.0).abs() < f32::EPSILON);

        let triangle = RightTriangle {
            base: 14.0,
            height: 36.0,
        };
        assert!((triangle.area() - 0.5 * 14.0 * 36.0).abs() < f32::EPSILON);
    }

    // trait Copy: Clone {}
    // Marker trait
    // Auti-trait
    #[derive(PartialEq)]
    struct Clonable {
        id: i32,
        info: String,
    }

    #[derive(Copy, PartialEq)]
    struct Copyable {
        id: i32,
        // info: String,
    }

    impl Clone for Clonable {
        fn clone(&self) -> Self {
            println!("copy for {self}");
            Self {
                id: self.id,
                info: self.info.clone(),
            }
        }
    }

    impl Clone for Copyable {
        fn clone(&self) -> Self {
            println!("clone for {self}");
            Self { id: self.id }
        }
    }

    impl fmt::Display for Clonable {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.id, self.info)
        }
    }

    impl fmt::Display for Copyable {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({})", self.id)
        }
    }

    #[test]
    fn test_copy_clone() {
        let clonable1 = Clonable {
            id: 1,
            info: "1".to_string(),
        };
        let clonable2 = clonable1.clone();
        assert!(clonable1 == clonable2);
        println!("{clonable1} == {clonable2}");
        let copiable1 = Copyable { id: 1 };
        let copiable2 = copiable1;
        assert!(copiable1 == copiable2);
        println!("{copiable1} == {copiable2}");
        let clonable3 = copiable2.clone();
        assert!(copiable2 == clonable3);
        println!("{copiable2} == {clonable3}");
    }

    // playground 2
    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c4900f5b30ac9a419041d4a6ecb52dd2
    //
    use std::ops::{Add, Neg};

    // Трейты, которые мы используем:
    // - Add (сложение) https://doc.rust-lang.org/stable/std/ops/trait.Add.html
    // - Neg (отрицание) https://doc.rust-lang.org/stable/std/ops/trait.Neg.html

    // PartialEq автоматически реализует операцию сравнения (==)
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Vec2<T> {
        x: T,
        y: T,
    }

    // Реализуйте Add для всех Vec2<T>, где T: Add<Output=T>
    // Шаблон реализации для Vec2<f32> дан для примера, измените его.
    impl<T> Add for Vec2<T>
    where
        T: Add<Output = T>,
    {
        type Output = Self;
        fn add(self, other: Self) -> Self::Output {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    // Реализуйте Neg для всех Vec2<T> аналогичным образом.
    impl<T> Neg for Vec2<T>
    where
        T: Neg<Output = T>,
    {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Self {
                x: -self.x,
                y: -self.y,
            }
        }
    }

    // Реализация `.length()` более сложна, так как у нас нет трейта Sqrt.
    // Для выполнения задания достаточно реализовать length для Vec2<f32>
    impl Vec2<f32> {
        fn length(self) -> f32 {
            f32::sqrt(self.x.powi(2) + self.y.powi(2))
        }
    }

    fn vec2f(x: f32, y: f32) -> Vec2<f32> {
        Vec2 { x, y }
    }

    fn vec2i(x: i32, y: i32) -> Vec2<i32> {
        Vec2 { x, y }
    }

    #[test]
    fn test_add() {
        let res = vec2f(1.0, 4.0) + vec2f(-9.0, 6.0);
        assert_eq!(res, vec2f(-8.0, 10.0));

        let res = vec2i(1, 4) + vec2i(-9, 6);
        assert_eq!(res, vec2i(-8, 10));
    }

    #[test]
    fn test_neg() {
        let res = -vec2f(1.0, -4.0);
        assert_eq!(res, vec2f(-1.0, 4.0));

        let res = -vec2i(1, -4);
        assert_eq!(res, vec2i(-1, 4));
    }

    #[test]
    fn test_length() {
        let res = vec2f(4.0, 3.0).length();
        assert_eq!(res, 5.0);
    }
}
