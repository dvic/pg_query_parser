extern crate libc;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::str;

mod ffi;

#[derive(Debug)]
pub struct PgQueryError {
    pub message: String,
    pub funcname: String,
    pub filename: String,
    pub lineno: i32,
    pub cursorpos: i32,
    pub context: Option<String>,
}

#[derive(Debug)]
pub struct ParseErrors {
    pub query_error: Option<PgQueryError>,
    pub stderr_buffer: Option<String>,
}

type PgQueryParseResult = Result<String, ParseErrors>;

pub fn pg_query_parse(input: &str) -> PgQueryParseResult {
    let c_input = CString::new(input).unwrap();

    unsafe {
        let parse_result = ffi::pg_query_parse(c_input.as_ptr());

        let parse_tree = non_zero_string(parse_result.parse_tree);
        let query_error = convert_query_error(parse_result.error);
        let stderr_buffer = non_zero_string(parse_result.stderr_buffer);

        ffi::pg_query_free_parse_result(parse_result);

        if query_error.is_some() || stderr_buffer.is_some() {
            Err(ParseErrors {
                query_error,
                stderr_buffer,
            })
        } else {
            Ok(parse_tree.unwrap_or("".to_string()))
        }
    }
}

type PgQueryFingerprintResult = Result<String, ParseErrors>;

pub fn pg_fingerprint(input: &str) -> PgQueryFingerprintResult {
    let c_input = CString::new(input).unwrap();
    unsafe {
        let fingerprint_result = ffi::pg_query_fingerprint_with_opts(c_input.as_ptr(), false);

        let fingerprint = non_zero_string(fingerprint_result.hexdigest);
        let query_error = convert_query_error(fingerprint_result.error);
        let stderr_buffer = non_zero_string(fingerprint_result.stderr_buffer);

        ffi::pg_query_free_fingerprint_result(fingerprint_result);

        if query_error.is_some() || stderr_buffer.is_some() {
            Err(ParseErrors {
                query_error,
                stderr_buffer,
            })
        } else {
            Ok(fingerprint.unwrap_or("".to_string()))
        }
    }
}

unsafe fn non_zero_string(buf: *const c_char) -> Option<String> {
    if !buf.is_null() {
        let ret_bytes = CStr::from_ptr(buf).to_bytes();
        Some(str::from_utf8(ret_bytes).unwrap().to_string()).filter(|v| v != "")
    } else {
        None
    }
}

unsafe fn convert_query_error(error: *mut ffi::PgQueryError) -> Option<PgQueryError> {
    if !error.is_null() {
        let error = &*(error);
        let message = {
            let bytes = CStr::from_ptr(error.message).to_bytes();
            str::from_utf8(bytes).unwrap().to_string()
        };

        let funcname = {
            let bytes = CStr::from_ptr(error.funcname).to_bytes();
            str::from_utf8(bytes).unwrap().to_string()
        };

        let filename = {
            let bytes = CStr::from_ptr(error.filename).to_bytes();
            str::from_utf8(bytes).unwrap().to_string()
        };

        let context = if !error.context.is_null() {
            let bytes = CStr::from_ptr(error.context).to_bytes();
            Some(str::from_utf8(bytes).unwrap().to_string())
        } else {
            None
        };

        Some(PgQueryError {
            message: message,
            funcname: funcname,
            filename: filename,
            lineno: error.lineno,
            cursorpos: error.cursorpos,
            context: context,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_query_valid() {
        let result = pg_query_parse("SELECT 1");

        match result {
            Ok(parsed) => println!("{:?}", parsed),
            Err(ParseErrors {
                query_error,
                stderr_buffer,
            }) => panic!(
                "query_error: {:?}, stderr_buffer: {:?}",
                query_error, stderr_buffer
            ),
        }
    }

    #[test]
    fn parse_query_invalid() {
        let result = pg_query_parse("INSERT FROM DOES NOT WORK");

        match result {
            Ok(parsed) => panic!("got unexpected result {:?}", parsed),
            Err(ParseErrors {
                query_error,
                stderr_buffer,
            }) => println!(
                "query_error: {:?}, stderr_buffer: {:?}",
                query_error, stderr_buffer
            ),
        }
    }

    #[test]
    fn fingerprint_query_valid() {
        let result = pg_fingerprint("SELECT 1");

        match result {
            Ok(fingerprint) => {
                assert_eq!("02a281c251c3a43d2fe7457dff01f76c5cc523f8c8", fingerprint)
            }
            Err(ParseErrors {
                query_error,
                stderr_buffer,
            }) => panic!(
                "query_error: {:?}, stderr_buffer: {:?}",
                query_error, stderr_buffer
            ),
        }
    }

    #[test]
    fn fingerprint_query_invalid() {
        let result = pg_fingerprint("INSERT FROM DOES NOT WORK");

        match result {
            Ok(parsed) => panic!("got unexpected result {:?}", parsed),
            Err(ParseErrors {
                query_error,
                stderr_buffer,
            }) => println!(
                "query_error: {:?}, stderr_buffer: {:?}",
                query_error, stderr_buffer
            ),
        }
    }

    // TODO: more tests
}
