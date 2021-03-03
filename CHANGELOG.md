# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.2] - 2021-03-02

### Added

- `From` impls for converting a `&Utf8Path` or a `Utf8PathBuf` into `Box<Path>`, `Rc<Path>`, `Arc<Path>` and `Cow<'a, Path>`.
- `PartialEq` and `PartialOrd` implementations comparing `Utf8Path` and `Utf8PathBuf` with `Path`, `PathBuf` and its
  variants, and comparing `OsStr`, `OsString` and its variants.

## [1.0.1] - 2021-02-25

### Added

- More `PartialEq` and `PartialOrd` implementations.
- MSRV lowered to 1.34.

## [1.0.0] - 2021-02-23

Initial release.

[1.0.2]: https://github.com/withoutboats/camino/releases/tag/camino-1.0.2
[1.0.1]: https://github.com/withoutboats/camino/releases/tag/camino-1.0.1
[1.0.0]: https://github.com/withoutboats/camino/releases/tag/camino-1.0.0
