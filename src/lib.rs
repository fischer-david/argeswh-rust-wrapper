pub mod arge;

mod test {
    #[tokio::test]
    async fn get_poe_sample() {
        let abbreviation: String = "poe".to_string();
        let location: Option<Location> = Location::new(abbreviation).await;

        match location {
            None => {
                assert!(false, "Location not found");
            }
            Some(location) => {
                let items = location.get_items().await;
                match items {
                    None => {
                        assert!(false, "Error: Could not parse response");
                    }
                    Some(items) => {
                        assert!(true, "Items found");
                    }
                }
            }
        }
    }
}