use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use thiserror::Error as ThisError;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    /// provider's parser was responsible for the error <br/>
    /// represented in the following pattern: `provider_name:parser_name`
    pub provider_parser: &'static str,

    /// error message indicating what went wrong
    pub message: &'static str,

    /// erroneous http status code
    pub status: u16,
}

impl ErrorDetails {
    fn new(
        provider_parser: &'static str,
        err_msg: Option<&'static str>,
        status: Option<StatusCode>,
    ) -> Self {
        const DEFAULT_ERROR_STATUS: u16 = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
        const DEFAULT_ERROR_MESSAGE: &str = "Something went wrong";

        let (status, message) = match (status, err_msg) {
            (Some(s), Some(msg)) => (s.as_u16(), msg),
            (Some(s), None) => (s.as_u16(), DEFAULT_ERROR_MESSAGE),
            (None, Some(msg)) => (DEFAULT_ERROR_STATUS, msg),
            (None, None) => (DEFAULT_ERROR_STATUS, DEFAULT_ERROR_MESSAGE),
        };

        let err = ErrorDetails {
            provider_parser,
            message,
            status,
        };

        err.log_error();
        return err;
    }

    fn get_json(&self) -> String {
        match to_string_pretty(self) {
            Ok(json) => json,
            Err(_) => {
                println!("error occured while serializing custom `EnmaError` into json");
                "{\"status\": 500}".to_string()
            }
        }
    }

    fn log_error(&self) {
        const ANSI_ESC_CODE_COLOR_RED: &str = "\x1b[31m";
        const ANSI_ESC_CODE_COLOR_RESET: &str = "\x1b[0m";

        println!(
            "{}{}{}",
            ANSI_ESC_CODE_COLOR_RED,
            self.get_json(),
            ANSI_ESC_CODE_COLOR_RESET
        );
    }
}

pub type EnmaResult<T> = std::result::Result<T, EnmaError>;

/// Custom error to handle different types of errors
#[derive(Debug, ThisError)]
pub enum EnmaError {
    // eg: <HiAnime:get_home_page>: 500 SrcFetchError
    #[error("<{}>: SrcFetchError {}", details.provider_parser, details.status)]
    SrcFetchError {
        details: ErrorDetails,
        #[source]
        source: reqwest::Error,
    },

    #[error("<{}>: ParseIntError {}", details.provider_parser, details.status)]
    ParseIntError {
        details: ErrorDetails,
        #[source]
        source: std::num::ParseIntError,
    },

    #[error("<{}>: UnknownError {}", details.provider_parser, details.status)]
    UnknownError { details: ErrorDetails },
}

impl EnmaError {
    pub fn src_fetch_error(
        source: reqwest::Error,
        provider_parser: &'static str,
        status: Option<StatusCode>,
    ) -> Self {
        return Self::SrcFetchError {
            details: ErrorDetails::new(
                provider_parser,
                Some("SrcFetchError: Failed to fetch source data"),
                status,
            ),
            source,
        };
    }

    pub fn parse_int_error(
        source: std::num::ParseIntError,
        provider_parser: &'static str,
        status: Option<StatusCode>,
    ) -> Self {
        return Self::ParseIntError {
            details: ErrorDetails::new(
                provider_parser,
                Some("ParseIntError: Failed to parse integer value"),
                status,
            ),
            source,
        };
    }

    pub fn unknown_error(
        provider_parser: &'static str,
        err_message: Option<&'static str>,
        status: Option<StatusCode>,
    ) -> Self {
        return Self::UnknownError {
            details: ErrorDetails::new(provider_parser, err_message, status),
        };
    }
}