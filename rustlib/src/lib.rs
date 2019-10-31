use std::thread;

extern crate ndarray;
use ndarray::prelude::*;

fn mul_test(v: i32, s: usize) -> i32 {
    let a = array![4, 5, 6];
    let vv = Array::from(vec![v; s]);
    a.dot(&vv)
}

#[no_mangle]
pub extern "C" fn hello(v: i32) -> i32 {
    let handler = thread::spawn(move || mul_test(v, v as usize));

    match handler.join() {
        Ok(v) => v,
        Err(_) => -1,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello() {
        assert_eq!(crate::hello(3), 45);
    }
    #[test]
    fn test_panic() {
        assert_eq!(crate::hello(2), -1);
    }
}
