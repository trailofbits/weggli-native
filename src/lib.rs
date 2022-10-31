use std::ffi::{CStr, CString};

use libc::{c_char, c_void, size_t};
use weggli::query::QueryTree as QueryTreeImpl;
use weggli::result::QueryResult as QueryResultImpl;

/// An opaque container for Weggli's query tree.
pub struct QueryTree(QueryTreeImpl);

/// An opaque container for individual query results.
pub struct QueryResult(QueryResultImpl);

/// An opaque container for zero or more query results, as produced by
/// [`weggli_matches`](weggli_matches).
pub struct QueryResults(Vec<QueryResult>);

/// Create a new Weggli query.
///
/// # Safety
///
/// * `q` must be a valid, NULL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn weggli_new_query(q: *const c_char, cpp: bool) -> *mut QueryTree {
    let q = match CStr::from_ptr(q).to_str() {
        Ok(q) => q,
        Err(_) => return std::ptr::null_mut(),
    };

    let tree = weggli::parse(q, cpp);
    let mut c = tree.walk();
    let qt = weggli::builder::build_query_tree(q, &mut c, cpp, None);
    match qt {
        Ok(qt) => Box::into_raw(Box::new(QueryTree(qt))),
        Err(_) => return std::ptr::null_mut(),
    }
}

/// Destroy a Weggli query produced with [`weggli_new_query`](weggli_new_query).
///
/// # Safety
///
/// * `qt` must have been created by `weggli_new_query`, and must not have been
///    previously passed into this function. Passing in any other source of
///    query trees will produce a double-free.
#[no_mangle]
pub unsafe extern "C" fn weggli_destroy_query(qt: *mut QueryTree) {
    Box::from_raw(qt);
}

/// Run a Weggli query against some source code.
///
/// # Safety
///
/// * `qt` must have been created by `weggli_new_query`. Passing in any other
///    source of query trees will produce a double-free.
///
/// * `source` must point to a valid region of memory containing a string
///   of at least `length` bytes, **not** including any null terminator.
#[no_mangle]
pub unsafe extern "C" fn weggli_matches(
    qt: *mut QueryTree,
    source: *const c_char,
    length: usize,
    cpp: bool,
) -> *mut QueryResults {
    let qt = &*qt;

    // NOTE(ww): We transmute from `*const c_char` (which is either `i8` or `u8`,
    // depending on the host) to `*const u8` here, since `str::from_utf8` only
    // knows how to convert from `&[u8]` and not `&[i8]`.
    let source = match std::str::from_utf8(std::slice::from_raw_parts(
        std::mem::transmute(source),
        length,
    )) {
        Ok(q) => q,
        Err(_) => return std::ptr::null_mut(),
    };

    let source_tree = weggli::parse(source, cpp);
    let results =
        qt.0.matches(source_tree.root_node(), source)
            .into_iter()
            .map(QueryResult)
            .collect();

    Box::into_raw(Box::new(QueryResults(results)))
}

/// Destroy the matches produced by [`weggli_matches`](weggli_matches).
///
/// # Safety
///
/// * `res` must have been created by `weggli_matches`, and must not have been
///   previously passed into this function.
#[no_mangle]
pub unsafe extern "C" fn weggli_destroy_matches(res: *mut QueryResults) {
    Box::from_raw(res);
}

type ResultCallback = unsafe extern "C" fn(*const QueryResult, *mut c_void) -> bool;

/// Yield each match in `matches` to a callback. Callbacks have the following
/// signature:
///
/// ```c
/// bool handle_result(const QueryResult *result, void *userdata);
/// ```
///
/// Where `QueryResult` is an opaque pointer and `userdata` is optional,
/// user-supplied callback state.
///
/// Callbacks can `true` to continue iteration, and `false` to exit.
///
/// # Safety
///
/// * `matches` must have been created by `weggli_matches`, and must not have
///   been previously freed by a call to `weggli_destroy_matches`.
///
/// * Callbacks must not hold onto match results for longer than `matches`
///   is alive.
#[no_mangle]
pub unsafe extern "C" fn weggli_iter_matches(
    matches: *const QueryResults,
    callback: ResultCallback,
    user: *mut c_void,
) {
    let matches = &*matches;

    for result in matches.0.iter() {
        if !callback(result as *const QueryResult, user) {
            break;
        }
    }
}

type CapturesCallback = unsafe extern "C" fn(size_t, size_t, *mut c_void) -> bool;

/// Yield each capture in `result` to a callback. Callbacks have the following
/// signature:
///
/// ```c
/// bool handle_capture(size_t start, size_t end, void *userdata);
/// ```
///
/// Where the two `size_t` parameters represent the start and end range of
/// the capture, and `userdata` is optional, user-supplied callback state.
///
/// Callbacks can `true` to continue iteration, and `false` to exit.
///
/// # Safety
///
/// * `result` must have been created by `weggli_iter_matches`.
#[no_mangle]
pub unsafe extern "C" fn weggli_iter_match_captures(
    result: *const QueryResult,
    callback: CapturesCallback,
    user: *mut c_void,
) {
    let result = &*result;

    for capture in result.0.captures.iter() {
        if !callback(capture.range.start, capture.range.end, user) {
            break;
        }
    }
}

type VariablesCallback = unsafe extern "C" fn(*const c_char, size_t, size_t, *mut c_void) -> bool;

/// Yield each variable capture in `result` to a callback. Callbacks have the following
/// signature:
///
/// ```c
/// bool handle_variable(const char *name, size_t start, size_t end, void *userdata);
/// ```
///
/// Where `name` is the name of the variable, the two `size_t` parameters
/// represent the start and end range of the capture, and `userdata` is
/// optional, user-supplied callback state.
///
/// Callbacks can `true` to continue iteration, and `false` to exit.
///
/// Returns `false` if an error occurs during variable-to-capture mapping
/// (e.g., if a variable has an unrepresentable name or references a
/// nonexistent capture).
///
/// # Safety
///
/// * `result` must have been created by `weggli_iter_matches`.
///
/// * Callbacks must not hold onto `name` for longer than their own lifetime.
#[no_mangle]
pub unsafe extern "C" fn weggli_iter_match_variables(
    result: *const QueryResult,
    callback: VariablesCallback,
    user: *mut c_void,
) -> bool {
    let result = &*result;

    for (var, idx) in result.0.vars.iter() {
        // This is unlikely to fail, but conceivably could if a variable
        // somehow ends up with a NULL byte in it.
        let var = match CString::new(var.clone()) {
            Ok(var) => var,
            Err(_) => return false,
        };

        // This shouldn't ever fail, assuming that `result` is
        // internally consistent.
        let capture = match result.0.captures.get(*idx) {
            Some(capture) => capture,
            None => return false,
        };

        if !callback(
            var.as_c_str().as_ptr(),
            capture.range.start,
            capture.range.end,
            user,
        ) {
            break;
        }
    }

    true
}
