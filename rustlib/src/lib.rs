#[no_mangle]
pub extern "C" fn hello() {
    println!("world!!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
