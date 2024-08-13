weggli-native
=============

![Tests](https://github.com/trailofbits/weggli-native/actions/workflows/tests.yml/badge.svg)

weggli-native is a "native" C API for Google's
[weggli](https://github.com/googleprojectzero/weggli).

It exposes a subset of weggli's public Rust API to C and C++ consumers.

See the [demo](demo/) for a small example of use.

## Building

```console
$ git clone https://github.com/trailofbits/weggli-native
$ cd weggli-native
$ # add --release for a release build
$ cargo build
```

You can also use [`cbindgen`](https://github.com/mozilla/cbindgen) to generate
weggli-native's header file:

```console
$ cbindgen --config cbindgen.toml --crate weggli-native --output weggli.h
```
