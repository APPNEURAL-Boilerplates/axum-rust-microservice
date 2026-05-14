use crate::common::error::AppError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
}

impl CreateItemRequest {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.name.trim().is_empty() {
            return Err(AppError::bad_request("name is required"));
        }
        if self.name.chars().count() > 120 {
            return Err(AppError::bad_request(
                "name must be 120 characters or fewer",
            ));
        }
        if !self.price.is_finite() || self.price < 0.0 {
            return Err(AppError::bad_request("price must be a non-negative number"));
        }
        Ok(())
    }
}
