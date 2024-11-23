use mockall::{mock, predicate::*};

// Define the ApiClient trait
trait ApiClient {
    fn fetch_data(&self) -> String;
}

mock! {
    // MockApiClient
    pub ApiClient {}

    impl ApiClient for ApiClient {
        // expect_fetch_data
        fn fetch_data(&self) -> String;
    }

}

// Real implementation of the ApiClient
struct RealApiClient;

impl ApiClient for RealApiClient {
    fn fetch_data(&self) -> String {
        // Simulating an API call
        // Processed: Real data from API
        "Real data from API".to_string()
    }
}

// Code that uses the ApiClient trait
struct DataService<T: ApiClient> {
    api_client: T,
}

impl<T: ApiClient> DataService<T> {
    fn new(api_client: T) -> Self {
        Self { api_client }
    }

    fn get_processed_data(&self) -> String {
        let data = self.api_client.fetch_data();
        format!("Processed: {}", data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_service_with_mockall() {
        // Create a new mock client
        let mut mock_client = MockApiClient::new();

        // Set up the mock to return specific data
        mock_client
            .expect_fetch_data()
            .return_const("Mocked data".to_string());

        // Use the mock client in the DataService
        let service = DataService::new(mock_client);

        // Call the method and check the result
        let result = service.get_processed_data();
        assert_eq!(result, "Processed: Mocked data");
    }
}
