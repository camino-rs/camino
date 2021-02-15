# camino - UTF-8 encoded paths

`camino` is an extension of the `std::path` module that adds new `Utf8PathBuf` and `Utf8Path`
types. These are like the standard library's `PathBuf` and `Path` types, except they are
guaranteed to only contain UTF-8 encoded data. Therefore, they expose the ability to get their
contents as strings, they implement `Display`, etc.

The `std::path` types are not guaranteed to be valid UTF-8. This is the right decision for the standard library,
since it must be as general as possible. However, on all platforms, non-Unicode paths are vanishingly uncommon for a
number of reasons:
* Unicode won. There are still some legacy codebases that store paths in encodings like Shift-JIS, but most
  have been converted to Unicode at this point.
* Unicode is the common subset of supported paths across Windows and Unix platforms. (On Windows, Rust stores paths
  as [an extension to UTF-8](https://simonsapin.github.io/wtf-8/), and converts them to UTF-16 at Win32
  API boundaries.
* There are already many systems, such as Cargo, that only support UTF-8 paths. If your own tool interacts with any such
  system, you can assume that paths are valid UTF-8 without creating any additional burdens on consumers.
* The ["makefile problem"](https://www.mercurial-scm.org/wiki/EncodingStrategy#The_.22makefile_problem.22)
  (which also applies to `Cargo.toml`, and any other metadata file that lists the names of other files) has *no general,
  cross-platform solution* in systems that support non-UTF-8 paths. However, restricting paths to UTF-8 eliminates
  this problem.

Therefore, many programs that want to manipulate paths *do* assume they contain UTF-8 data, and convert them to `str`s
as  necessary. However, because this invariant is not encoded in the `Path` type, conversions such as
`path.to_str().unwrap()` need to be repeated again and again, creating a frustrating experience.

Instead, `camino` allows you to check that your paths are UTF-8 *once*, and then manipulate them
as valid UTF-8 from there on, avoiding repeated lossy and confusing conversions.

## Rust version support

The minimum supported Rust version (MSRV) is **1.44**. This project is tested in CI against the latest stable version of
Rust and the MSRV.

## License

This project is available under the terms of either the [Apache 2.0 license](LICENSE-APACHE) or the [MIT
license](LICENSE-MIT).
