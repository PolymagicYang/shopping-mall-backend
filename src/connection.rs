use mongodb::{options::ClientOptions, Client};

pub async fn get_connection() -> Result<Client, mongodb::error::Error>  {
    // Parse your connection string into an options struct
    let mut client_options =
        ClientOptions::parse("mongodb+srv://PolymagicYang:9jEEDC1dPFipzuxe@cluster0.oy6omls.mongodb.net/?retryWrites=true&w=majority")
            .await?;
    // Manually set an option
    client_options.app_name = Some("Rust Demo".to_string());
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    Ok(client)
}