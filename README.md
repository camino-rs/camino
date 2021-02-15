# camino - UTF-8 encoded paths

camino is an extension of the `std::path` module, which adds new `Utf8PathBuf` and `Utf8Path` types.
These are like the standard library's PathBuf and Path types, except that they are guaranteed to
contain only utf-8 encoded data. Therefore, they expose the ability to get their contents as
strings, they implement Display, etc.

The standard library path types are not assumed to be utf8 data. On all platforms, non-utf8 paths
are vanishingly uncommon. Therefore, many programs that want to manipulate paths *do* assume they
contain utf8 data, and convert them to strs as necessary. However, becaues this conversion is not
encoded in the Path type, it needs to be repeated again and again, creating a frustrating
experience.

Instead, camino allows the user to assume that their paths are Utf8 *once*, and then manipulate them
as utf8 encoded paths from them on, avoiding repeated lossy and confusing conversions.

Most of the API is still TODO.

# Rust version support

The minimum supported Rust version (MSRV) is **1.44**. This project is tested in CI against the latest stable version of
Rust and the MSRV.

# License

This project is available under the terms of either the [Apache 2.0 license](LICENSE-APACHE) or the [MIT
license](LICENSE-MIT).
