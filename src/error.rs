#[derive(Debug)]
pub enum Error {
    // 400 Bad Request
    // The server could not process the request due to invalid request parameters or invalid format
    // of the parameters.
    InvalidRequestError,

    // 402 Payment Required
    // The request could not be processed because of the user has an insufficient plan. If you want
    // to be able to process this request, get a higher plan.
    InsufficientPlan,

    // 403 Forbidden
    // The request could not be processed due to invalid API key.
    InvalidApiKey,

    // 404 Not Found
    // The server could not process the request due to invalid URL or invalid path parameter.
    InvalidParameter,

    // 429 Too Many Requests
    // The rate limit has been exceeded. Reduce the frequency of requests to avoid this error.
    RateLimitError,

    // 500 Internal Server Error
    // An unexpected server error has occured.
    InternalServerError,

    // Failed to connect with API.
    ApiConnectionError,

    // Error from http client.
    Reqwest(reqwest::Error),

    // Error from JSON creation/processing.
    Json(serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Json(e)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Reqwest(ref e) => Some(e),
            Error::Json(ref e) => Some(e),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidRequestError => {
                write!(f, "The server could not process the request due to invalid request parameters or invalid format of the parameters.")
            }
            Error::InsufficientPlan => {
                write!(f, "The request could not be processed because of the user has an insufficient plan. If you want to be able to process this request, get a higher plan.")
            }
            Error::InvalidApiKey => {
                write!(
                    f,
                    "The request could not be processed due to invalid API key."
                )
            }
            Error::InvalidParameter => {
                write!(f, "The server could not process the request due to invalid URL or invalid path parameter.")
            }
            Error::RateLimitError => {
                write!(f, "The rate limit has been exceeded. Reduce the frequency of requests to avoid this error.")
            }
            Error::InternalServerError => {
                write!(f, "An unexpected server error has occured.")
            }
            Error::ApiConnectionError => {
                write!(f, "Fail to connect to API.")
            }
            Error::Reqwest(err) => {
                write!(f, "{}", err)
            }
            Error::Json(err) => {
                write!(f, "{}", err)
            }
        }
    }
}
