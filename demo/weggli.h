#pragma once

/* Generated with cbindgen:0.20.0 */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * An opaque container for individual query results.
 */
typedef struct QueryResult QueryResult;

/**
 * An opaque container for zero or more query results, as produced by
 * [`weggli_matches`](weggli_matches).
 */
typedef struct QueryResults QueryResults;

/**
 * An opaque container for Weggli's query tree.
 */
typedef struct QueryTree QueryTree;

typedef bool (*ResultCallback)(const struct QueryResult*, void*);

typedef bool (*CapturesCallback)(size_t, size_t, void*);

typedef bool (*VariablesCallback)(const char*, size_t, size_t, void*);

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Create a new Weggli query.
 *
 * # Safety
 *
 * * `q` must be a valid, NULL-terminated string.
 */
struct QueryTree *weggli_new_query(const char *q, bool cpp);

/**
 * Destroy a Weggli query produced with [`weggli_new_query`](weggli_new_query).
 *
 * # Safety
 *
 * * `qt` must have been created by `weggli_new_query`, and must not have been
 *    previously passed into this function. Passing in any other source of
 *    query trees will produce a double-free.
 */
void weggli_destroy_query(struct QueryTree *qt);

/**
 * Run a Weggli query against some source code.
 *
 * # Safety
 *
 * * `qt` must have been created by `weggli_new_query`. Passing in any other
 *    source of query trees will produce a double-free.
 *
 * * `source` must be a valid, NULL-terminated string.
 */
struct QueryResults *weggli_matches(struct QueryTree *qt, const char *source, bool cpp);

/**
 * Destroy the matches produced by [`weggli_matches`](weggli_matches).
 *
 * # Safety
 *
 * * `res` must have been created by `weggli_matches`, and must not have been
 *   previously passed into this function.
 */
void weggli_destroy_matches(struct QueryResults *res);

/**
 * Yield each match in `matches` to a callback. Callbacks have the following
 * signature:
 *
 * ```c
 * bool handle_result(const QueryResult *result, void *userdata);
 * ```
 *
 * Where `QueryResult` is an opaque pointer and `userdata` is optional,
 * user-supplied callback state.
 *
 * Callbacks can `true` to continue iteration, and `false` to exit.
 *
 * # Safety
 *
 * * `matches` must have been created by `weggli_matches`, and must not have
 *   been previously freed by a call to `weggli_destroy_matches`.
 *
 * * Callbacks must not hold onto match results for longer than `matches`
 *   is alive.
 */
void weggli_iter_matches(const struct QueryResults *matches, ResultCallback callback, void *user);

/**
 * Yield each capture in `result` to a callback. Callbacks have the following
 * signature:
 *
 * ```c
 * bool handle_capture(size_t start, size_t end, void *userdata);
 * ```
 *
 * Where the two `size_t` parameters represent the start and end range of
 * the capture, and `userdata` is optional, user-supplied callback state.
 *
 * Callbacks can `true` to continue iteration, and `false` to exit.
 *
 * # Safety
 *
 * * `matches` must have been created by `weggli_matches`, and must not have
 *   been previously freed by a call to `weggli_destroy_matches`.
 */
void weggli_iter_match_captures(const struct QueryResult *result,
                                CapturesCallback callback,
                                void *user);

/**
 * Yield each variable capture in `result` to a callback. Callbacks have the following
 * signature:
 *
 * ```c
 * bool handle_variable(const char *name, size_t start, size_t end, void *userdata);
 * ```
 *
 * Where `name` is the name of the variable, the two `size_t` parameters
 * represent the start and end range of the capture, and `userdata` is
 * optional, user-supplied callback state.
 *
 * Callbacks can `true` to continue iteration, and `false` to exit.
 *
 * Returns `false` if an error occurs during variable-to-capture mapping
 * (e.g., if a variable has an unrepresentable name or references a
 * nonexistent capture).
 *
 * # Safety
 *
 * * `matches` must have been created by `weggli_matches`, and must not have
 *   been previously freed by a call to `weggli_destroy_matches`.
 */
bool weggli_iter_match_variables(const struct QueryResult *result,
                                 VariablesCallback callback,
                                 void *user);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
