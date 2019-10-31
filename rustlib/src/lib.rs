use std::thread;

extern crate ndarray;
use ndarray::prelude::*;

#[repr(C)]
pub struct Point(pub i32, pub usize);

fn mul_test(v: i32, s: usize) -> i32 {
    let a = array![4, 5, 6];
    let vv = Array::from(vec![v; s]);
    a.dot(&vv)
}

#[no_mangle]
pub extern "C" fn hello(p: Point) -> i32 {
    let handler = thread::spawn(move || mul_test(p.0, p.1));

    match handler.join() {
        Ok(v) => v,
        Err(_) => -1,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello() {
        let p = crate::Point(3, 3);
        assert_eq!(crate::hello(p), 45);
    }
    #[test]
    fn test_panic() {
        let p = crate::Point(2, 2);
        assert_eq!(crate::hello(p), -1);
    }
}
