use std::{
    borrow::{Borrow, Cow},
    cmp::Ordering,
    ffi::{OsStr, OsString},
    fmt, fs, io,
    iter::FusedIterator,
    ops::Deref,
    path::*,
    rc::Rc,
    sync::Arc,
};

/// An owned, mutable UTF-8 path (akin to [`String`]).
///
/// This type provides methods like [`push`] and [`set_extension`] that mutate
/// the path in place. It also implements [`Deref`] to [`Utf8Path`], meaning that
/// all methods on [`Utf8Path`] slices are available on `Utf8PathBuf` values as well.
///
/// [`push`]: Utf8PathBuf::push
/// [`set_extension`]: Utf8PathBuf::set_extension
///
/// # Examples
///
/// You can use [`push`] to build up a `Utf8PathBuf` from
/// components:
///
/// ```
/// use camino::Utf8PathBuf;
///
/// let mut path = Utf8PathBuf::new();
///
/// path.push(r"C:\");
/// path.push("windows");
/// path.push("system32");
///
/// path.set_extension("dll");
/// ```
///
/// However, [`push`] is best used for dynamic situations. This is a better way
/// to do this when you know all of the components ahead of time:
///
/// ```
/// use camino::Utf8PathBuf;
///
/// let path: Utf8PathBuf = [r"C:\", "windows", "system32.dll"].iter().collect();
/// ```
///
/// We can still do better than this! Since these are all strings, we can use
/// `From::from`:
///
/// ```
/// use camino::Utf8PathBuf;
///
/// let path = Utf8PathBuf::from(r"C:\windows\system32.dll");
/// ```
///
/// Which method works best depends on what kind of situation you're in.
// NB: Internal PathBuf must only contain utf8 data
#[derive(Clone, Default, Hash)]
#[repr(transparent)]
pub struct Utf8PathBuf(PathBuf);

impl Utf8PathBuf {
    /// Allocates an empty `Utf8PathBuf`.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let path = Utf8PathBuf::new();
    /// ```
    pub fn new() -> Utf8PathBuf {
        Utf8PathBuf(PathBuf::new())
    }

    /// Creates a new `Utf8PathBuf` from a `PathBuf` containing valid UTF-8 characters.
    ///
    /// Errors with the original `PathBuf` if it is not valid UTF-8.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    /// use std::ffi::OsStr;
    /// # #[cfg(unix)]
    /// use std::os::unix::ffi::OsStrExt;
    /// use std::path::PathBuf;
    ///
    /// let unicode_path = PathBuf::from("/valid/unicode");
    /// Utf8PathBuf::from_path_buf(unicode_path).expect("valid Unicode path succeeded");
    ///
    /// // Paths on Unix can be non-UTF-8.
    /// # #[cfg(unix)]
    /// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
    /// # #[cfg(unix)]
    /// let non_unicode_path = PathBuf::from(non_unicode_str);
    /// # #[cfg(unix)]
    /// Utf8PathBuf::from_path_buf(non_unicode_path).expect_err("non-Unicode path failed");
    /// ```
    pub fn from_path_buf(path: PathBuf) -> Result<Utf8PathBuf, PathBuf> {
        match path.into_os_string().into_string() {
            Ok(string) => Ok(Utf8PathBuf::from(string)),
            Err(os_string) => Err(PathBuf::from(os_string)),
        }
    }

    /// Creates a new `Utf8PathBuf` with a given capacity used to create the internal [`PathBuf`].
    /// See [`with_capacity`] defined on [`PathBuf`].
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let mut path = Utf8PathBuf::with_capacity(10);
    /// let capacity = path.capacity();
    ///
    /// // This push is done without reallocating
    /// path.push(r"C:\");
    ///
    /// assert_eq!(capacity, path.capacity());
    /// ```
    ///
    /// [`with_capacity`]: PathBuf::with_capacity
    pub fn with_capacity(capacity: usize) -> Utf8PathBuf {
        Utf8PathBuf(PathBuf::with_capacity(capacity))
    }

    /// Coerces to a [`Utf8Path`] slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let p = Utf8PathBuf::from("/test");
    /// assert_eq!(Utf8Path::new("/test"), p.as_path());
    /// ```
    pub fn as_path(&self) -> &Utf8Path {
        unsafe { Utf8Path::assert_utf8(&*self.0) }
    }

    /// Extends `self` with `path`.
    ///
    /// If `path` is absolute, it replaces the current path.
    ///
    /// On Windows:
    ///
    /// * if `path` has a root but no prefix (e.g., `\windows`), it
    ///   replaces everything except for the prefix (if any) of `self`.
    /// * if `path` has a prefix but no root, it replaces `self`.
    ///
    /// # Examples
    ///
    /// Pushing a relative path extends the existing path:
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let mut path = Utf8PathBuf::from("/tmp");
    /// path.push("file.bk");
    /// assert_eq!(path, Utf8PathBuf::from("/tmp/file.bk"));
    /// ```
    ///
    /// Pushing an absolute path replaces the existing path:
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let mut path = Utf8PathBuf::from("/tmp");
    /// path.push("/etc");
    /// assert_eq!(path, Utf8PathBuf::from("/etc"));
    /// ```
    pub fn push(&mut self, path: impl AsRef<Utf8Path>) {
        self.0.push(&path.as_ref().0)
    }

    /// Truncates `self` to [`self.parent`].
    ///
    /// Returns `false` and does nothing if [`self.parent`] is [`None`].
    /// Otherwise, returns `true`.
    ///
    /// [`self.parent`]: Utf8Path::parent
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let mut p = Utf8PathBuf::from("/spirited/away.rs");
    ///
    /// p.pop();
    /// assert_eq!(Utf8Path::new("/spirited"), p);
    /// p.pop();
    /// assert_eq!(Utf8Path::new("/"), p);
    /// ```
    pub fn pop(&mut self) -> bool {
        self.0.pop()
    }

    /// Updates [`self.file_name`] to `file_name`.
    ///
    /// If [`self.file_name`] was [`None`], this is equivalent to pushing
    /// `file_name`.
    ///
    /// Otherwise it is equivalent to calling [`pop`] and then pushing
    /// `file_name`. The new path will be a sibling of the original path.
    /// (That is, it will have the same parent.)
    ///
    /// [`self.file_name`]: Utf8Path::file_name
    /// [`pop`]: Utf8PathBuf::pop
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let mut buf = Utf8PathBuf::from("/");
    /// assert_eq!(buf.file_name(), None);
    /// buf.set_file_name("bar");
    /// assert_eq!(buf, Utf8PathBuf::from("/bar"));
    /// assert!(buf.file_name().is_some());
    /// buf.set_file_name("baz.txt");
    /// assert_eq!(buf, Utf8PathBuf::from("/baz.txt"));
    /// ```
    pub fn set_file_name(&mut self, file_name: impl AsRef<str>) {
        self.0.set_file_name(file_name.as_ref())
    }

    /// Updates [`self.extension`] to `extension`.
    ///
    /// Returns `false` and does nothing if [`self.file_name`] is [`None`],
    /// returns `true` and updates the extension otherwise.
    ///
    /// If [`self.extension`] is [`None`], the extension is added; otherwise
    /// it is replaced.
    ///
    /// [`self.file_name`]: Utf8Path::file_name
    /// [`self.extension`]: Utf8Path::extension
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let mut p = Utf8PathBuf::from("/feel/the");
    ///
    /// p.set_extension("force");
    /// assert_eq!(Utf8Path::new("/feel/the.force"), p.as_path());
    ///
    /// p.set_extension("dark_side");
    /// assert_eq!(Utf8Path::new("/feel/the.dark_side"), p.as_path());
    /// ```
    pub fn set_extension(&mut self, extension: impl AsRef<str>) -> bool {
        self.0.set_extension(extension.as_ref())
    }

    /// Consumes the `Utf8PathBuf`, yielding its internal [`String`] storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let p = Utf8PathBuf::from("/the/head");
    /// let s = p.into_string();
    /// assert_eq!(s, "/the/head");
    /// ```
    pub fn into_string(self) -> String {
        self.into_os_string().into_string().unwrap()
    }

    /// Consumes the `Utf8PathBuf`, yielding its internal [`OsString`] storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    /// use std::ffi::OsStr;
    ///
    /// let p = Utf8PathBuf::from("/the/head");
    /// let s = p.into_os_string();
    /// assert_eq!(s, OsStr::new("/the/head"));
    /// ```
    pub fn into_os_string(self) -> OsString {
        self.0.into_os_string()
    }

    /// Converts this `Utf8PathBuf` into a [boxed](Box) [`Utf8Path`].
    pub fn into_boxed_path(self) -> Box<Utf8Path> {
        unsafe { Box::from_raw(Box::into_raw(self.0.into_boxed_path()) as *mut Utf8Path) }
    }

    /// Invokes [`capacity`] on the underlying instance of [`PathBuf`].
    ///
    /// [`capacity`]: PathBuf::capacity
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Invokes [`clear`] on the underlying instance of [`PathBuf`].
    ///
    /// [`clear`]: PathBuf::clear
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Invokes [`reserve`] on the underlying instance of [`PathBuf`].
    ///
    /// [`reserve`]: PathBuf::reserve
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    /// Invokes [`reserve_exact`] on the underlying instance of [`PathBuf`].
    ///
    /// [`reserve_exact`]: PathBuf::reserve_exact
    pub fn reserve_exact(&mut self, additional: usize) {
        self.0.reserve_exact(additional)
    }

    /// Invokes [`shrink_to_fit`] on the underlying instance of [`PathBuf`].
    ///
    /// [`shrink_to_fit`]: PathBuf::shrink_to_fit
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit()
    }
}

impl Deref for Utf8PathBuf {
    type Target = Utf8Path;

    fn deref(&self) -> &Utf8Path {
        self.as_path()
    }
}

impl fmt::Debug for Utf8PathBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl fmt::Display for Utf8PathBuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl<P: AsRef<Utf8Path>> Extend<P> for Utf8PathBuf {
    fn extend<I: IntoIterator<Item = P>>(&mut self, iter: I) {
        for path in iter {
            self.push(path);
        }
    }
}

/// A slice of a UTF-8 path (akin to [`str`]).
///
/// This type supports a number of operations for inspecting a path, including
/// breaking the path into its components (separated by `/` on Unix and by either
/// `/` or `\` on Windows), extracting the file name, determining whether the path
/// is absolute, and so on.
///
/// This is an *unsized* type, meaning that it must always be used behind a
/// pointer like `&` or [`Box`]. For an owned version of this type,
/// see [`Utf8PathBuf`].
///
/// # Examples
///
/// ```
/// use camino::Utf8Path;
///
/// // Note: this example does work on Windows
/// let path = Utf8Path::new("./foo/bar.txt");
///
/// let parent = path.parent();
/// assert_eq!(parent, Some(Utf8Path::new("./foo")));
///
/// let file_stem = path.file_stem();
/// assert_eq!(file_stem, Some("bar"));
///
/// let extension = path.extension();
/// assert_eq!(extension, Some("txt"));
/// ```
// NB: Internal Path must only contain utf8 data
#[repr(transparent)]
#[derive(Hash)]
pub struct Utf8Path(Path);

impl Utf8Path {
    /// Directly wraps a string slice as a `Utf8Path` slice.
    ///
    /// This is a cost-free conversion.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// Utf8Path::new("foo.txt");
    /// ```
    ///
    /// You can create `Utf8Path`s from `String`s, or even other `Utf8Path`s:
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let string = String::from("foo.txt");
    /// let from_string = Utf8Path::new(&string);
    /// let from_path = Utf8Path::new(&from_string);
    /// assert_eq!(from_string, from_path);
    /// ```
    pub fn new(s: &(impl AsRef<str> + ?Sized)) -> &Utf8Path {
        unsafe { Utf8Path::assert_utf8(Path::new(s.as_ref())) }
    }

    /// Converts a [`Path`] to a `Utf8Path`.
    ///
    /// Returns `None` if the path is not valid UTF-8.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    /// use std::ffi::OsStr;
    /// # #[cfg(unix)]
    /// use std::os::unix::ffi::OsStrExt;
    /// use std::path::Path;
    ///
    /// let unicode_path = Path::new("/valid/unicode");
    /// Utf8Path::from_path(unicode_path).expect("valid Unicode path succeeded");
    ///
    /// // Paths on Unix can be non-UTF-8.
    /// # #[cfg(unix)]
    /// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
    /// # #[cfg(unix)]
    /// let non_unicode_path = Path::new(non_unicode_str);
    /// # #[cfg(unix)]
    /// assert!(Utf8Path::from_path(non_unicode_path).is_none(), "non-Unicode path failed");
    /// ```
    pub fn from_path(path: &Path) -> Option<&Utf8Path> {
        path.as_os_str().to_str().map(|s| Utf8Path::new(s))
    }

    /// Yields the underlying [`str`] slice.
    ///
    /// Unlike [`Path::to_str`], this always returns a slice because the contents of a `Utf8Path`
    /// are guaranteed to be valid UTF-8.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let s = Utf8Path::new("foo.txt").as_str();
    /// assert_eq!(s, "foo.txt");
    /// ```
    ///
    /// [`str`]: str
    pub fn as_str(&self) -> &str {
        unsafe { assert_utf8(self.as_os_str()) }
    }

    /// Yields the underlying [`OsStr`] slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let os_str = Utf8Path::new("foo.txt").as_os_str();
    /// assert_eq!(os_str, std::ffi::OsStr::new("foo.txt"));
    /// ```
    pub fn as_os_str(&self) -> &OsStr {
        self.0.as_os_str()
    }

    /// Converts a `Utf8Path` to an owned [`Utf8PathBuf`].
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let path_buf = Utf8Path::new("foo.txt").to_path_buf();
    /// assert_eq!(path_buf, Utf8PathBuf::from("foo.txt"));
    /// ```
    pub fn to_path_buf(&self) -> Utf8PathBuf {
        Utf8PathBuf(self.0.to_path_buf())
    }

    /// Returns `true` if the `Utf8Path` is absolute, i.e., if it is independent of
    /// the current directory.
    ///
    /// * On Unix, a path is absolute if it starts with the root, so
    /// `is_absolute` and [`has_root`] are equivalent.
    ///
    /// * On Windows, a path is absolute if it has a prefix and starts with the
    /// root: `c:\windows` is absolute, while `c:temp` and `\temp` are not.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert!(!Utf8Path::new("foo.txt").is_absolute());
    /// ```
    ///
    /// [`has_root`]: Utf8Path::has_root
    pub fn is_absolute(&self) -> bool {
        self.0.is_absolute()
    }

    /// Returns `true` if the `Utf8Path` is relative, i.e., not absolute.
    ///
    /// See [`is_absolute`]'s documentation for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert!(Utf8Path::new("foo.txt").is_relative());
    /// ```
    ///
    /// [`is_absolute`]: Utf8Path::is_absolute
    pub fn is_relative(&self) -> bool {
        self.0.is_relative()
    }

    /// Returns `true` if the `Utf8Path` has a root.
    ///
    /// * On Unix, a path has a root if it begins with `/`.
    ///
    /// * On Windows, a path has a root if it:
    ///     * has no prefix and begins with a separator, e.g., `\windows`
    ///     * has a prefix followed by a separator, e.g., `c:\windows` but not `c:windows`
    ///     * has any non-disk prefix, e.g., `\\server\share`
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert!(Utf8Path::new("/etc/passwd").has_root());
    /// ```
    pub fn has_root(&self) -> bool {
        self.0.has_root()
    }

    /// Returns the `Path` without its final component, if there is one.
    ///
    /// Returns [`None`] if the path terminates in a root or prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/foo/bar");
    /// let parent = path.parent().unwrap();
    /// assert_eq!(parent, Utf8Path::new("/foo"));
    ///
    /// let grand_parent = parent.parent().unwrap();
    /// assert_eq!(grand_parent, Utf8Path::new("/"));
    /// assert_eq!(grand_parent.parent(), None);
    /// ```
    pub fn parent(&self) -> Option<&Utf8Path> {
        self.0
            .parent()
            .map(|path| unsafe { Utf8Path::assert_utf8(path) })
    }

    /// Produces an iterator over `Utf8Path` and its ancestors.
    ///
    /// The iterator will yield the `Utf8Path` that is returned if the [`parent`] method is used zero
    /// or more times. That means, the iterator will yield `&self`, `&self.parent().unwrap()`,
    /// `&self.parent().unwrap().parent().unwrap()` and so on. If the [`parent`] method returns
    /// [`None`], the iterator will do likewise. The iterator will always yield at least one value,
    /// namely `&self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let mut ancestors = Utf8Path::new("/foo/bar").ancestors();
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("/foo/bar")));
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("/foo")));
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("/")));
    /// assert_eq!(ancestors.next(), None);
    ///
    /// let mut ancestors = Utf8Path::new("../foo/bar").ancestors();
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("../foo/bar")));
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("../foo")));
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("..")));
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("")));
    /// assert_eq!(ancestors.next(), None);
    /// ```
    ///
    /// [`parent`]: Utf8Path::parent
    pub fn ancestors(&self) -> Utf8Ancestors<'_> {
        Utf8Ancestors(self.0.ancestors())
    }

    /// Returns the final component of the `Utf8Path`, if there is one.
    ///
    /// If the path is a normal file, this is the file name. If it's the path of a directory, this
    /// is the directory name.
    ///
    /// Returns [`None`] if the path terminates in `..`.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert_eq!(Some("bin"), Utf8Path::new("/usr/bin/").file_name());
    /// assert_eq!(Some("foo.txt"), Utf8Path::new("tmp/foo.txt").file_name());
    /// assert_eq!(Some("foo.txt"), Utf8Path::new("foo.txt/.").file_name());
    /// assert_eq!(Some("foo.txt"), Utf8Path::new("foo.txt/.//").file_name());
    /// assert_eq!(None, Utf8Path::new("foo.txt/..").file_name());
    /// assert_eq!(None, Utf8Path::new("/").file_name());
    /// ```
    pub fn file_name(&self) -> Option<&str> {
        self.0.file_name().map(|s| unsafe { assert_utf8(s) })
    }

    /// Returns a path that, when joined onto `base`, yields `self`.
    ///
    /// # Errors
    ///
    /// If `base` is not a prefix of `self` (i.e., [`starts_with`]
    /// returns `false`), returns [`Err`].
    ///
    /// [`starts_with`]: Utf8Path::starts_with
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let path = Utf8Path::new("/test/haha/foo.txt");
    ///
    /// assert_eq!(path.strip_prefix("/"), Ok(Utf8Path::new("test/haha/foo.txt")));
    /// assert_eq!(path.strip_prefix("/test"), Ok(Utf8Path::new("haha/foo.txt")));
    /// assert_eq!(path.strip_prefix("/test/"), Ok(Utf8Path::new("haha/foo.txt")));
    /// assert_eq!(path.strip_prefix("/test/haha/foo.txt"), Ok(Utf8Path::new("")));
    /// assert_eq!(path.strip_prefix("/test/haha/foo.txt/"), Ok(Utf8Path::new("")));
    ///
    /// assert!(path.strip_prefix("test").is_err());
    /// assert!(path.strip_prefix("/haha").is_err());
    ///
    /// let prefix = Utf8PathBuf::from("/test/");
    /// assert_eq!(path.strip_prefix(prefix), Ok(Utf8Path::new("haha/foo.txt")));
    /// ```
    pub fn strip_prefix(&self, base: impl AsRef<Path>) -> Result<&Utf8Path, StripPrefixError> {
        self.0
            .strip_prefix(base)
            .map(|path| unsafe { Utf8Path::assert_utf8(path) })
    }

    /// Determines whether `base` is a prefix of `self`.
    ///
    /// Only considers whole path components to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/etc/passwd");
    ///
    /// assert!(path.starts_with("/etc"));
    /// assert!(path.starts_with("/etc/"));
    /// assert!(path.starts_with("/etc/passwd"));
    /// assert!(path.starts_with("/etc/passwd/")); // extra slash is okay
    /// assert!(path.starts_with("/etc/passwd///")); // multiple extra slashes are okay
    ///
    /// assert!(!path.starts_with("/e"));
    /// assert!(!path.starts_with("/etc/passwd.txt"));
    ///
    /// assert!(!Utf8Path::new("/etc/foo.rs").starts_with("/etc/foo"));
    /// ```
    pub fn starts_with(&self, base: impl AsRef<Path>) -> bool {
        self.0.starts_with(base)
    }

    /// Determines whether `child` is a suffix of `self`.
    ///
    /// Only considers whole path components to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/etc/resolv.conf");
    ///
    /// assert!(path.ends_with("resolv.conf"));
    /// assert!(path.ends_with("etc/resolv.conf"));
    /// assert!(path.ends_with("/etc/resolv.conf"));
    ///
    /// assert!(!path.ends_with("/resolv.conf"));
    /// assert!(!path.ends_with("conf")); // use .extension() instead
    /// ```
    pub fn ends_with(&self, base: impl AsRef<Path>) -> bool {
        self.0.ends_with(base)
    }

    /// Extracts the stem (non-extension) portion of [`self.file_name`].
    ///
    /// [`self.file_name`]: Utf8Path::file_name
    ///
    /// The stem is:
    ///
    /// * [`None`], if there is no file name;
    /// * The entire file name if there is no embedded `.`;
    /// * The entire file name if the file name begins with `.` and has no other `.`s within;
    /// * Otherwise, the portion of the file name before the final `.`
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert_eq!("foo", Utf8Path::new("foo.rs").file_stem().unwrap());
    /// assert_eq!("foo.tar", Utf8Path::new("foo.tar.gz").file_stem().unwrap());
    /// ```
    pub fn file_stem(&self) -> Option<&str> {
        self.0.file_stem().map(|s| unsafe { assert_utf8(s) })
    }

    /// Extracts the extension of [`self.file_name`], if possible.
    ///
    /// The extension is:
    ///
    /// * [`None`], if there is no file name;
    /// * [`None`], if there is no embedded `.`;
    /// * [`None`], if the file name begins with `.` and has no other `.`s within;
    /// * Otherwise, the portion of the file name after the final `.`
    ///
    /// [`self.file_name`]: Utf8Path::file_name
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert_eq!("rs", Utf8Path::new("foo.rs").extension().unwrap());
    /// assert_eq!("gz", Utf8Path::new("foo.tar.gz").extension().unwrap());
    /// ```
    pub fn extension(&self) -> Option<&str> {
        self.0.extension().map(|s| unsafe { assert_utf8(s) })
    }

    /// Creates an owned [`Utf8PathBuf`] with `path` adjoined to `self`.
    ///
    /// See [`Utf8PathBuf::push`] for more details on what it means to adjoin a path.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// assert_eq!(Utf8Path::new("/etc").join("passwd"), Utf8PathBuf::from("/etc/passwd"));
    /// ```
    pub fn join(&self, path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
        Utf8PathBuf(self.0.join(&path.as_ref().0))
    }

    /// Creates an owned [`PathBuf`] with `path` adjoined to `self`.
    ///
    /// See [`PathBuf::push`] for more details on what it means to adjoin a path.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    /// use std::path::PathBuf;
    ///
    /// assert_eq!(Utf8Path::new("/etc").join_os("passwd"), PathBuf::from("/etc/passwd"));
    /// ```
    pub fn join_os(&self, path: impl AsRef<Path>) -> PathBuf {
        self.0.join(path)
    }

    /// Creates an owned [`Utf8PathBuf`] like `self` but with the given file name.
    ///
    /// See [`Utf8PathBuf::set_file_name`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let path = Utf8Path::new("/tmp/foo.txt");
    /// assert_eq!(path.with_file_name("bar.txt"), Utf8PathBuf::from("/tmp/bar.txt"));
    ///
    /// let path = Utf8Path::new("/tmp");
    /// assert_eq!(path.with_file_name("var"), Utf8PathBuf::from("/var"));
    /// ```
    pub fn with_file_name(&self, file_name: impl AsRef<str>) -> Utf8PathBuf {
        Utf8PathBuf(self.0.with_file_name(file_name.as_ref()))
    }

    /// Creates an owned [`Utf8PathBuf`] like `self` but with the given extension.
    ///
    /// See [`Utf8PathBuf::set_extension`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let path = Utf8Path::new("foo.rs");
    /// assert_eq!(path.with_extension("txt"), Utf8PathBuf::from("foo.txt"));
    ///
    /// let path = Utf8Path::new("foo.tar.gz");
    /// assert_eq!(path.with_extension(""), Utf8PathBuf::from("foo.tar"));
    /// assert_eq!(path.with_extension("xz"), Utf8PathBuf::from("foo.tar.xz"));
    /// assert_eq!(path.with_extension("").with_extension("txt"), Utf8PathBuf::from("foo.txt"));
    /// ```
    pub fn with_extension(&self, extension: impl AsRef<str>) -> Utf8PathBuf {
        Utf8PathBuf(self.0.with_extension(extension.as_ref()))
    }

    /// Produces an iterator over the [`Utf8Component`]s of the path.
    ///
    /// When parsing the path, there is a small amount of normalization:
    ///
    /// * Repeated separators are ignored, so `a/b` and `a//b` both have
    ///   `a` and `b` as components.
    ///
    /// * Occurrences of `.` are normalized away, except if they are at the
    ///   beginning of the path. For example, `a/./b`, `a/b/`, `a/b/.` and
    ///   `a/b` all have `a` and `b` as components, but `./a/b` starts with
    ///   an additional [`CurDir`] component.
    ///
    /// * A trailing slash is normalized away, `/a/b` and `/a/b/` are equivalent.
    ///
    /// Note that no other normalization takes place; in particular, `a/c`
    /// and `a/b/../c` are distinct, to account for the possibility that `b`
    /// is a symbolic link (so its parent isn't `a`).
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Component, Utf8Path};
    ///
    /// let mut components = Utf8Path::new("/tmp/foo.txt").components();
    ///
    /// assert_eq!(components.next(), Some(Utf8Component::RootDir));
    /// assert_eq!(components.next(), Some(Utf8Component::Normal("tmp")));
    /// assert_eq!(components.next(), Some(Utf8Component::Normal("foo.txt")));
    /// assert_eq!(components.next(), None)
    /// ```
    ///
    /// [`CurDir`]: Utf8Component::CurDir
    pub fn components(&self) -> Utf8Components {
        Utf8Components(self.0.components())
    }

    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        self.0.metadata()
    }

    pub fn symlink_metadata(&self) -> io::Result<fs::Metadata> {
        self.0.symlink_metadata()
    }

    pub fn canonicalize(&self) -> io::Result<PathBuf> {
        self.0.canonicalize()
    }

    pub fn read_link(&self) -> io::Result<PathBuf> {
        self.0.read_link()
    }

    pub fn read_dir(&self) -> io::Result<fs::ReadDir> {
        self.0.read_dir()
    }

    pub fn exists(&self) -> bool {
        self.0.exists()
    }

    pub fn is_file(&self) -> bool {
        self.0.is_file()
    }

    pub fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    pub fn into_path_buf(self: Box<Utf8Path>) -> Utf8PathBuf {
        unsafe { Utf8PathBuf(Box::from_raw(Box::into_raw(self) as *mut Path).into_path_buf()) }
    }

    // invariant: Path must be guaranteed to be utf-8 data
    unsafe fn assert_utf8(path: &Path) -> &Utf8Path {
        &*(path as *const Path as *const Utf8Path)
    }
}

impl fmt::Display for Utf8Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl fmt::Debug for Utf8Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Utf8Ancestors<'a>(Ancestors<'a>);

impl<'a> fmt::Debug for Utf8Ancestors<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<'a> Iterator for Utf8Ancestors<'a> {
    type Item = &'a Utf8Path;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|path| unsafe { Utf8Path::assert_utf8(path) })
    }
}

impl<'a> FusedIterator for Utf8Ancestors<'a> {}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Utf8Components<'a>(Components<'a>);

impl<'a> Utf8Components<'a> {
    pub fn as_path(&self) -> &'a Utf8Path {
        unsafe { Utf8Path::assert_utf8(self.0.as_path()) }
    }
}

impl<'a> Iterator for Utf8Components<'a> {
    type Item = Utf8Component<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|component| unsafe { Utf8Component::new(component) })
    }
}

impl<'a> FusedIterator for Utf8Components<'a> {}

impl<'a> DoubleEndedIterator for Utf8Components<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0
            .next_back()
            .map(|component| unsafe { Utf8Component::new(component) })
    }
}

impl<'a> fmt::Debug for Utf8Components<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Utf8Component<'a> {
    Prefix(Utf8PrefixComponent<'a>),
    RootDir,
    CurDir,
    ParentDir,
    Normal(&'a str),
}

impl<'a> Utf8Component<'a> {
    unsafe fn new(component: Component<'a>) -> Utf8Component<'a> {
        match component {
            Component::Prefix(prefix) => Utf8Component::Prefix(Utf8PrefixComponent(prefix)),
            Component::RootDir => Utf8Component::RootDir,
            Component::CurDir => Utf8Component::CurDir,
            Component::ParentDir => Utf8Component::ParentDir,
            Component::Normal(s) => Utf8Component::Normal(assert_utf8(s)),
        }
    }

    pub fn as_str(&self) -> &'a str {
        unsafe { assert_utf8(self.as_os_str()) }
    }

    pub fn as_os_str(&self) -> &'a OsStr {
        match *self {
            Utf8Component::Prefix(prefix) => prefix.as_os_str(),
            Utf8Component::RootDir => Component::RootDir.as_os_str(),
            Utf8Component::CurDir => Component::CurDir.as_os_str(),
            Utf8Component::ParentDir => Component::ParentDir.as_os_str(),
            Utf8Component::Normal(s) => OsStr::new(s),
        }
    }
}

impl<'a> fmt::Debug for Utf8Component<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_os_str(), f)
    }
}

impl<'a> fmt::Display for Utf8Component<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Utf8PrefixComponent<'a>(PrefixComponent<'a>);

impl<'a> Utf8PrefixComponent<'a> {
    // TODO kind

    pub fn as_str(&self) -> &'a str {
        unsafe { assert_utf8(self.as_os_str()) }
    }

    pub fn as_os_str(&self) -> &'a OsStr {
        self.0.as_os_str()
    }
}

impl<'a> fmt::Debug for Utf8PrefixComponent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<'a> fmt::Display for Utf8PrefixComponent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl From<String> for Utf8PathBuf {
    fn from(string: String) -> Utf8PathBuf {
        Utf8PathBuf(string.into())
    }
}

impl From<&str> for Utf8PathBuf {
    fn from(s: &str) -> Utf8PathBuf {
        Utf8PathBuf(s.into())
    }
}

impl From<Box<Utf8Path>> for Utf8PathBuf {
    fn from(path: Box<Utf8Path>) -> Utf8PathBuf {
        path.into_path_buf()
    }
}

impl<'a> From<Cow<'a, Utf8Path>> for Utf8PathBuf {
    fn from(path: Cow<'a, Utf8Path>) -> Utf8PathBuf {
        path.into_owned()
    }
}

impl From<Utf8PathBuf> for String {
    fn from(path: Utf8PathBuf) -> String {
        path.into_string()
    }
}

impl From<Utf8PathBuf> for PathBuf {
    fn from(path: Utf8PathBuf) -> PathBuf {
        path.0
    }
}

impl From<Utf8PathBuf> for OsString {
    fn from(path: Utf8PathBuf) -> OsString {
        path.into_os_string()
    }
}

impl<'a> From<Utf8PathBuf> for Cow<'a, Utf8Path> {
    fn from(path: Utf8PathBuf) -> Cow<'a, Utf8Path> {
        Cow::Owned(path)
    }
}

impl From<Utf8PathBuf> for Arc<Utf8Path> {
    fn from(path: Utf8PathBuf) -> Arc<Utf8Path> {
        let arc: Arc<Path> = Arc::from(path.0);
        unsafe { Arc::from_raw(Arc::into_raw(arc) as *const Utf8Path) }
    }
}

impl From<Utf8PathBuf> for Rc<Utf8Path> {
    fn from(path: Utf8PathBuf) -> Rc<Utf8Path> {
        let rc: Rc<Path> = Rc::from(path.0);
        unsafe { Rc::from_raw(Rc::into_raw(rc) as *const Utf8Path) }
    }
}

impl AsRef<Utf8Path> for Utf8Path {
    fn as_ref(&self) -> &Utf8Path {
        self
    }
}

impl AsRef<Utf8Path> for Utf8PathBuf {
    fn as_ref(&self) -> &Utf8Path {
        self.as_path()
    }
}

impl AsRef<Utf8Path> for str {
    fn as_ref(&self) -> &Utf8Path {
        Utf8Path::new(self)
    }
}

impl AsRef<Utf8Path> for String {
    fn as_ref(&self) -> &Utf8Path {
        Utf8Path::new(self)
    }
}

impl AsRef<Path> for Utf8Path {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl AsRef<Path> for Utf8PathBuf {
    fn as_ref(&self) -> &Path {
        &*self.0
    }
}

impl AsRef<str> for Utf8Path {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for Utf8PathBuf {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<OsStr> for Utf8Path {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

impl AsRef<OsStr> for Utf8PathBuf {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

impl Borrow<Utf8Path> for Utf8PathBuf {
    fn borrow(&self) -> &Utf8Path {
        self.as_path()
    }
}

impl ToOwned for Utf8Path {
    type Owned = Utf8PathBuf;

    fn to_owned(&self) -> Utf8PathBuf {
        self.to_path_buf()
    }
}

impl<P: AsRef<Utf8Path>> std::iter::FromIterator<P> for Utf8PathBuf {
    fn from_iter<I: IntoIterator<Item = P>>(iter: I) -> Utf8PathBuf {
        let mut buf = Utf8PathBuf::new();
        buf.extend(iter);
        buf
    }
}

impl PartialEq for Utf8PathBuf {
    fn eq(&self, other: &Utf8PathBuf) -> bool {
        self.components() == other.components()
    }
}

impl Eq for Utf8PathBuf {}

impl PartialEq for Utf8Path {
    fn eq(&self, other: &Utf8Path) -> bool {
        self.components().eq(other.components())
    }
}

impl Eq for Utf8Path {}

impl PartialOrd for Utf8Path {
    fn partial_cmp(&self, other: &Utf8Path) -> Option<Ordering> {
        self.components().partial_cmp(other.components())
    }
}

macro_rules! impl_cmp {
    ($lhs:ty, $rhs: ty) => {
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                <Utf8Path as PartialEq>::eq(self, other)
            }
        }

        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                <Utf8Path as PartialEq>::eq(self, other)
            }
        }

        impl<'a, 'b> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
                <Utf8Path as PartialOrd>::partial_cmp(self, other)
            }
        }

        impl<'a, 'b> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<Ordering> {
                <Utf8Path as PartialOrd>::partial_cmp(self, other)
            }
        }
    };
}

impl_cmp!(Utf8PathBuf, Utf8Path);
impl_cmp!(Utf8PathBuf, &'a Utf8Path);
impl_cmp!(Cow<'a, Utf8Path>, Utf8Path);
impl_cmp!(Cow<'a, Utf8Path>, &'b Utf8Path);
impl_cmp!(Cow<'a, Utf8Path>, Utf8PathBuf);

// invariant: OsStr must be guaranteed to be utf8 data
unsafe fn assert_utf8(string: &OsStr) -> &str {
    &*(string as *const OsStr as *const str)
}
