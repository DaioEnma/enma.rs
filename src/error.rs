use reqwest::StatusCode;
use serde::Serialize;
use thiserror::Error as ThisError;

const ANSI_ESC_CODE_COLOR_RED: &str = "\x1b[31m";
const ANSI_ESC_CODE_COLOR_RESET: &str = "\x1b[0m";

const DEFAULT_ERROR_MESSAGE: &str = "Something went wrong";

/// Internal generic error implementation
#[derive(Debug, Serialize, ThisError)]
#[error(
    "{ANSI_ESC_CODE_COLOR_RED}{{\n  \"provider_parser\": \"{}\",\n  \"message\": \"{}\",\n  \"status\": {}\n}}{ANSI_ESC_CODE_COLOR_RESET}",
    provider_parser,
    message,
    status
)]
pub struct ErrorDetails {
    /// provider's parser was responsible for the error <br/>
    /// represented in the following pattern: `provider_name:parser_name`
    pub provider_parser: &'static str,

    /// error message indicating what went wrong
    pub message: String,

    /// erroneous http status code
    pub status: u16,
}

impl ErrorDetails {
    fn new(
        provider_parser: &'static str,
        err_msg: Option<String>,
        status: Option<StatusCode>,
    ) -> Self {
        const DEFAULT_ERROR_STATUS: u16 = StatusCode::INTERNAL_SERVER_ERROR.as_u16();

        let (status, message) = match (status, err_msg) {
            (Some(s), Some(msg)) => (s.as_u16(), msg),
            (Some(s), None) => (s.as_u16(), DEFAULT_ERROR_MESSAGE.to_string()),
            (None, Some(msg)) => (DEFAULT_ERROR_STATUS, msg),
            (None, None) => (DEFAULT_ERROR_STATUS, DEFAULT_ERROR_MESSAGE.to_string()),
        };

        let err = ErrorDetails {
            provider_parser,
            message,
            status,
        };

        eprintln!("{err}");
        return err;
    }
}

/// Custom Result<`T`, [`EnmaError`]> for generic success result across providers
pub type EnmaResult<T> = std::result::Result<T, EnmaError>;

/// Custom error to generalize different types of errors
#[derive(Debug, ThisError)]
pub enum EnmaError {
    /// represents raw source data fetch error
    #[error("{ANSI_ESC_CODE_COLOR_RED}<{}>: {} {}{ANSI_ESC_CODE_COLOR_RESET}", details.provider_parser, details.status, details.message)]
    SrcFetchError { details: ErrorDetails },

    /// represents raw source data parse error
    #[error("{ANSI_ESC_CODE_COLOR_RED}<{}>: {} {}{ANSI_ESC_CODE_COLOR_RESET}", details.provider_parser, details.status, details.message)]
    SrcParseError { details: ErrorDetails },

    /// represents any form of parsing error
    #[error("{ANSI_ESC_CODE_COLOR_RED}<{}>: {} {}{ANSI_ESC_CODE_COLOR_RESET}", details.provider_parser, details.status, details.message)]
    ParsingError { details: ErrorDetails },

    /// represents any form of parsing error
    #[error("{ANSI_ESC_CODE_COLOR_RED}<{}>: {} {}{ANSI_ESC_CODE_COLOR_RESET}", details.provider_parser, details.status, details.message)]
    InvalidDataError { details: ErrorDetails },

    /// represents miscellaneous error
    #[error("{ANSI_ESC_CODE_COLOR_RED}<{}>: {} {}{ANSI_ESC_CODE_COLOR_RESET}", details.provider_parser, details.status, details.message)]
    MiscError { details: ErrorDetails },
}

impl EnmaError {
    pub fn details(&self) -> &ErrorDetails {
        match self {
            EnmaError::MiscError { details } => details,
            EnmaError::ParsingError { details } => details,
            EnmaError::SrcFetchError { details } => details,
            EnmaError::SrcParseError { details } => details,
            EnmaError::InvalidDataError { details } => details,
        }
    }

    pub fn src_fetch_error(
        provider_parser: &'static str,
        err_msg: Option<String>,
        status: Option<StatusCode>,
    ) -> Self {
        const ERROR_PREFIX: &'static str = "SrcFetchError: ";
        let err_msg =
            Self::get_formatted_err(err_msg, ERROR_PREFIX, "Failed to fetch raw source data");

        return Self::SrcFetchError {
            details: ErrorDetails::new(provider_parser, err_msg, status),
        };
    }

    pub fn src_parse_error(
        provider_parser: &'static str,
        err_msg: Option<String>,
        status: Option<StatusCode>,
    ) -> Self {
        const ERROR_PREFIX: &'static str = "SrcParseError: ";
        let err_msg = Self::get_formatted_err(
            err_msg,
            ERROR_PREFIX,
            "Failed to parse fetched raw source data",
        );

        return Self::SrcParseError {
            details: ErrorDetails::new(provider_parser, err_msg, status),
        };
    }

    pub fn parsing_error(
        provider_parser: &'static str,
        err_msg: Option<String>,
        status: Option<StatusCode>,
    ) -> Self {
        const ERROR_PREFIX: &'static str = "ParsingError: ";
        let err_msg = Self::get_formatted_err(err_msg, ERROR_PREFIX, DEFAULT_ERROR_MESSAGE);

        return Self::ParsingError {
            details: ErrorDetails::new(provider_parser, err_msg, status),
        };
    }

    pub fn invalid_data_error(
        provider_parser: &'static str,
        err_msg: Option<String>,
        status: Option<StatusCode>,
    ) -> Self {
        const ERROR_PREFIX: &'static str = "InvalidDataError: ";
        let err_msg = Self::get_formatted_err(err_msg, ERROR_PREFIX, DEFAULT_ERROR_MESSAGE);

        return Self::MiscError {
            details: ErrorDetails::new(provider_parser, err_msg, status),
        };
    }

    pub fn misc_error(
        provider_parser: &'static str,
        err_msg: Option<String>,
        status: Option<StatusCode>,
    ) -> Self {
        const ERROR_PREFIX: &'static str = "MiscError: ";
        let err_msg = Self::get_formatted_err(err_msg, ERROR_PREFIX, DEFAULT_ERROR_MESSAGE);

        return Self::MiscError {
            details: ErrorDetails::new(provider_parser, err_msg, status),
        };
    }

    fn get_formatted_err(
        err_message: Option<String>,
        err_prefix: &'static str,
        default_err_message: &'static str,
    ) -> Option<String> {
        let formatted_msg = match err_message {
            Some(msg) => {
                if msg.starts_with(err_prefix) {
                    msg
                } else {
                    format!("{err_prefix}{msg}")
                }
            }
            None => format!("{err_prefix}{default_err_message}"),
        };

        Some(formatted_msg)
    }
}
