#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
#[async_trait::async_trait]
pub trait OCPPWS: Send + Sync {
    async fn request(&self) -> Result<(), Box<dyn std::error::Error>>;
}
