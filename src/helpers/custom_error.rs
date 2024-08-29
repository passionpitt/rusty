#[derive(Debug)]
pub struct CustomError {
    pub message: String,
    pub cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl CustomError {
    pub fn with_cause<E>(message: &str, cause: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        CustomError {
            message: message.to_string(),
            cause: Some(cause.into()),
        }
    }
}

impl warp::reject::Reject for CustomError {}
