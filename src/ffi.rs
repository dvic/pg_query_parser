use libc::{c_char, c_int};

#[derive(Debug)]
#[repr(C)]
pub struct PgQueryError {
	pub message: *const c_char,  // exception message
	pub funcname: *const c_char, // source function of exception (e.g. SearchSysCache)
	pub filename: *const c_char, // source of exception (e.g. parse.l)
	pub lineno: c_int,           // source of exception (e.g. 104)
	pub cursorpos: c_int,        // char in query at which exception occurred
	pub context: *const c_char,  // additional context (optional, can be NULL)
}

#[derive(Debug)]
#[repr(C)]
pub struct PgQueryParseResult {
	pub parse_tree: *const c_char,
	pub stderr_buffer: *const c_char,
	pub error: *mut PgQueryError,
}

#[derive(Debug)]
#[repr(C)]
pub struct PgQueryFingerprintResult {
	pub hexdigest: *const c_char,
	pub stderr_buffer: *const c_char,
	pub error: *mut PgQueryError,
}

#[link(name = "pg_query")]
extern "C" {
	pub fn pg_query_parse(input: *const c_char) -> PgQueryParseResult;
	pub fn pg_query_free_parse_result(result: PgQueryParseResult);

	pub fn pg_query_fingerprint_with_opts(
		input: *const c_char,
		printTokens: bool,
	) -> PgQueryFingerprintResult;
	pub fn pg_query_free_fingerprint_result(result: PgQueryFingerprintResult);
}
