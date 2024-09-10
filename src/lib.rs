use libnbnet_sys as raw;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn server_run() {
    unsafe { raw::NBN_UDP_Register(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
