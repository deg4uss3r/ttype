#![feature(type_name_of_val)]

#[allow(unused_macros)]
// ptype == print type :) 
#[macro_export]
macro_rules! ptype {
    ($input:expr) => {
        println!("{}", std::any::type_name_of_val($input));
    }
}

#[allow(unused_macros)]
// rtype == return type :) 
#[macro_export]
macro_rules! rtype {
    ($input:expr) => {
        std::any::type_name_of_val($input)
    }
}

#[test]
fn test_type() {
    let x: u64 = 42;
    let t = rtype!(&x);
    ptype!(&x);
    assert_eq!(t, "u64");
}

#[allow(dead_code)]
struct Tester {
    x: u64,
    y: f64,
}

#[test]
fn test_custom_type() {
    let test_h = Tester{x: 42, y:18.0};
    let t = rtype!(&test_h);
    ptype!(&t);
    assert_eq!(t, "ttype::Tester");
}


#[test]
fn test_custom_type_sub() {
    let test_h = Tester{x: 42, y:18.0};
    let t = rtype!(&test_h.x);
    ptype!(&test_h.x);
    assert_eq!(t, "u64");
}
