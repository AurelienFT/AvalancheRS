pub mod apis;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{:#?}", crate::utils::constants::HRP_TO_NETWORK_ID.get(&"custom"));
        assert_eq!(2 + 2, 4);
    }
}
