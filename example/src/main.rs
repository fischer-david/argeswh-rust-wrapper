use arge_rust_wrapper::arge::Location;

#[tokio::main]
async fn main() {
    let abbreviation: String = "poe".to_string();
    let location: Option<Location> = Location::new(abbreviation).await;

    match location {
        None => {
            println!("Location not found")
        }
        Some(location) => {
            let items = location.get_items().await;
            match items {
                None => {
                    println!("Error: Could not parse response")
                }
                Some(items) => {
                    println!("Items found");
                    println!("{:?}", items);
                }
            }
        }
    }
}
