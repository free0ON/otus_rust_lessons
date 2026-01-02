use std::any::Any;

#[cfg(test)]
#[test]
fn simple_types_tests() {
    use std::sync::atomic::AtomicU64;

    let value_i8: i8 = 1;
    let value_i16: i16 = 2;
    let value_i32: i32 = 4;
    let value_i64: i64 = 8;
    let value_i128: i128 = 16;
    let value_isize: isize = 8;
    let value_u8: u8 = 1;
    let value_u16: u16 = 2;
    let value_u32: u32 = 4;
    let value_u64: u64 = 8;
    let value_u128: u128 = 16;
    let value_usize: usize = 8;
    let value_f32: f32 = 4.0;
    let value_f64: f64 = 8.0;
    let value_char: char = '1';
    let value_bool: bool = true;
    let reference = &value_i32;
    assert!(size_of_val(&value_i8) == value_i8 as usize);
    assert!(size_of_val(&value_i16) == value_i16 as usize);
    assert!(size_of_val(&value_i32) == value_i32 as usize);
    assert!(size_of_val(&value_i64) == value_i64 as usize);
    assert!(size_of_val(&value_i128) == value_i128 as usize);
    assert!(size_of_val(&value_isize) == value_isize as usize);
    assert!(size_of_val(&value_u8) == value_u8 as usize);
    assert!(size_of_val(&value_u16) == value_u16 as usize);
    assert!(size_of_val(&value_u32) == value_u32 as usize);
    assert!(size_of_val(&value_u64) == value_u64 as usize);
    assert!(size_of_val(&value_u128) == value_u128 as usize);
    assert!(size_of_val(&value_usize) == value_usize);
    assert!(size_of_val(&value_f32) == value_f32 as usize);
    assert!(size_of_val(&value_f64) == value_f64 as usize);
    assert!(size_of_val(&value_char) == size_of::<char>());
    assert!(size_of_val(&value_bool) == value_bool as usize);
    assert!(size_of_val(&reference) == size_of::<&i32>());

    let lh_i32: i32 = 1;
    let rh_i32: i32 = 2;
    let lh_f32: f32 = 1.0;
    let rh_f32: f32 = 2.0;
    let lh_bool: bool = true;
    let rh_bool: bool = false;
    let lh_u8: u8 = 1;
    let rh_u8: u8 = 2;
    const BIRTH_DATE: &str = "22.01.1982";
    static MY_BIRTH_BAY: &str = "22.01.1982";

    static GLOBAL_COUNTER: AtomicU64 = AtomicU64::new(0);

    assert!(lh_i32 + rh_i32 == 3);
    assert!(lh_f32 + rh_f32 == 3.0);
    assert!((lh_i32 as f32) + rh_f32 == 3.0);
    assert!(!lh_bool == rh_bool);
    assert!(lh_i32 << 1 == 2);
    assert!(lh_f32 + rh_f32 == 3.0);
    assert!(lh_u8 % rh_u8 == 1);

    // wrapping
    assert!(lh_u8.wrapping_add(255) == 0);

    // checket return Option<T>
    match lh_u8.checked_add(255) {
        Some(result) => assert!(result == 0),
        None => eprint!("Overflow"),
    }
    // overfloving retunt tuple (T, bool)
    assert!(lh_u8.overflowing_add(255).1 == true);
    assert!(lh_u8.overflowing_add(255).0 == 0);
    // saturating
    assert!(lh_u8.saturating_add(255) == 255);
    // downcast
    let big_u16: u16 = 2 * 255;
    let small_u8: u8 = match big_u16.try_into() {
        Ok(result) => result,
        Err(e) => {
            dbg!(format!("{e}"));
            (big_u16 - u8::MAX as u16) as u8
        }
    };

    assert!(small_u8 == 255);

    // tuple
    let lh_tupple: (i32, f64, bool) = (1, 2.0, true);
    let rh_tupple: (i32, f64, bool) = (1, 2.0, true);

    assert!(lh_tupple == rh_tupple);
    assert!(lh_tupple.0 == rh_tupple.0);

    //array
    let lh_array: [i32; 5] = [1, 2, 3, 4, 5];
    let mut rh_array: [i32; 5] = [1, 2, 3, 4, 5];

    assert!(lh_array == rh_array);
    assert!(lh_array.len() == 5);
    assert!(lh_array[0] == rh_array[0]);
    assert!(lh_array.contains(&rh_array[0]) == true);
    assert!(lh_array.is_sorted() == true);
    assert!(lh_array.map(|x| x == 5)[4] == true);
    rh_array.swap(0, 1);
    assert!(rh_array == [2, 1, 3, 4, 5]);

    assert!(lh_u8 == 1_u8);

    assert!(BIRTH_DATE == MY_BIRTH_BAY);
    GLOBAL_COUNTER.fetch_add(1_u64, std::sync::atomic::Ordering::Acquire);
    assert!(GLOBAL_COUNTER.load(std::sync::atomic::Ordering::Acquire) == 1_u64);

    let sum = |array: &[i32; 5]| -> i32 { array.iter().sum() };
    assert!(sum(&lh_array) == 1 + 2 + 3 + 4 + 5);
}

#[test]
fn ownersheep_test() {
    // ownersheep
    // 1. Any data has a variable-owner
    // 2. At the time dame has just one owner
    // 3. Then owner goes out from scope - the data is dropes

    let a_string = String::from("Hello, world");
    let b_string = a_string.clone();
    let a_i32 = 666;
    let b_i32 = a_i32; // simple types has trait Copy and automaticly copy then assigh 
    assert!(b_string == a_string);
    assert!(b_i32 == a_i32);
    println!("{a_string} {a_i32}");
    println!("{b_string} {b_i32}")
}

#[test]
fn loop_while_for_test() {
    let mut array: [i32; 5] = [1; 5];
    let mut for_cirle = || {
        for i in 0..array.len() {
            array[i] = (i + 1) as i32;
        }
        array
    };

    assert!(for_cirle() == array);

    let while_circle = |array: &[i32; 5]| {
        let mut i = 0;
        while array[i] < 3 {
            i += 1;
        }
        array[i]
    };
    assert!(while_circle(&mut array) == 3);

    let loop_circle = || {
        let mut i = 0;
        'outer: loop {
            if array[i] == 3 {
                break 'outer;
            }
            i += 1;
        }
        array[i]
    };

    assert!(loop_circle() == 3);
}

#[test]
fn if_else_test() {
    let num_i32 = 6;
    let ret = if num_i32 % 4 == 0 {
        4
    } else if num_i32 % 3 == 0 {
        3
    } else if num_i32 % 2 == 0 {
        2
    } else {
        1
    };
    assert!(ret == 3);
}

struct Doc {
    name: String,
    text: String,
    view: u32,
}

fn init_doc(name: String, text: String) -> Doc {
    Doc {
        name: name,
        text: text,
        view: 0,
    }
}

impl PartialEq for Doc {
    fn eq(&self, rh: &Self) -> bool {
        self.name == rh.name && self.text == rh.text
    }
}

#[test]
fn strust_test() {
    let doc1 = Doc {
        name: String::from("Hello"),
        text: String::from("world"),
        view: 0,
    };

    let doc2 = Doc {
        name: String::from("Hello"),
        ..doc1
    };

    assert!(init_doc(String::from("Hello"), String::from("world")) == doc2);

    assert!(
        Doc {
            name: String::from("Hello"),
            text: String::from("world"),
            view: 0,
        } == doc2
    );
}
#[derive(Debug)]
enum LoadError {
    NotFound,
    BufferTooSmall { actual: usize, expected: usize },
    Internal(String),
}

impl LoadError {
    fn get_buff_size(&self) -> (usize, usize) {
        match self {
            LoadError::BufferTooSmall { actual, expected } => (*actual, *expected),
            _ => (0, 0),
        }
    }
}

impl PartialEq for LoadError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LoadError::NotFound, LoadError::NotFound) => true,
            (LoadError::BufferTooSmall { actual, expected }, LoadError::BufferTooSmall { .. }) => {
                (*actual, *expected) == other.get_buff_size()
            }
            (LoadError::Internal(s1), LoadError::Internal(s2)) => s1 == s2,
            _ => false,
        }
    }
}

#[test]
fn enum_test() {
    let err1 = LoadError::BufferTooSmall {
        actual: 1,
        expected: 2000,
    };
    let err2 = LoadError::BufferTooSmall {
        actual: 1,
        expected: 2000,
    };
    let err3 = LoadError::Internal("Some error".to_string());
    let err = err3;
    match err {
        LoadError::NotFound => {
            println!("Not found");
        }
        LoadError::BufferTooSmall { actual, expected } => {
            println!("{actual} {expected}");
        }
        LoadError::Internal(s) => {
            println!("{s}");
        }
    }

    assert!(err1 == err2);
}

#[test]
fn if_let_while_let_test() {
    let msg = String::from("connection failed");
    let err1 = LoadError::Internal(msg.clone());
    let mut err2 = LoadError::Internal("Error".to_string());

    let try_load_again = |num_err: i32| -> LoadError {
        if num_err > 0 {
            return LoadError::Internal("Error".to_string());
        }
        LoadError::NotFound
    };

    if let LoadError::Internal(s) = err1 {
        assert!(s == msg)
    }

    let mut num_err = 10;
    while let LoadError::Internal(_) = err2 {
        err2 = try_load_again(num_err);
        println!("{:?}", err2);
        // assert!(err2 == LoadError::Internal("Error".to_string()));
        num_err -= 1;
    }
}

#[test]
fn fizz_buzz_test() {
    let x = 15;
    let s = match (x % 3, x % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => x.to_string(),
    };

    assert!(s == "FizzBuzz".to_string());
}
