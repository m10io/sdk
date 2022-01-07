pub struct ImageClient {
    address: String,
    image_client: reqwest::Client,
}

impl ImageClient {
    pub fn new(address: impl Into<String>) -> Self {
        let address = address.into();
        let image_client = reqwest::Client::new();
        Self {
            address,
            image_client,
        }
    }

    pub async fn put_image(&mut self, name: &str, image: Vec<u8>) -> reqwest::Result<String> {
        let url = format!("{}/images/{}", self.address, name);
        self.image_client
            .put(&url)
            .body(image)
            .send()
            .await?
            .error_for_status()?;
        Ok(url)
    }

    pub async fn get_image(&mut self, name: &str) -> reqwest::Result<Vec<u8>> {
        let url = format!("{}/images/{}", self.address, name);
        let image = self
            .image_client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?
            .to_vec();
        Ok(image)
    }
}
