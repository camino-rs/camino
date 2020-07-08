use std::borrow::Borrow;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs;
use std::io;
use std::iter::FusedIterator;
use std::ops::Deref;
use std::path::*;

// NB: Internal PathBuf must only contain utf8 data
#[derive(Clone, Default, Hash)]
#[repr(transparent)]
pub struct Utf8PathBuf(PathBuf);

impl Utf8PathBuf {
    pub fn new() -> Utf8PathBuf {
        Utf8PathBuf(PathBuf::new())
    }

    pub fn from_path_buf(path: PathBuf) -> Result<Utf8PathBuf, PathBuf> {
        match path.into_os_string().into_string() {
            Ok(string)      => Ok(Utf8PathBuf::from(string)),
            Err(os_string)  => Err(PathBuf::from(os_string)),
        }
    }

    pub fn with_capacity(capacity: usize) -> Utf8PathBuf {
        Utf8PathBuf(PathBuf::with_capacity(capacity))
    }

    pub fn as_path(&self) -> &Utf8Path {
        unsafe { Utf8Path::assert_utf8(&*self.0) }
    }

    pub fn push(&mut self, path: impl AsRef<Utf8Path>) {
        self.0.push(&path.as_ref().0)
    }

    pub fn pop(&mut self) -> bool {
        self.0.pop()
    }

    pub fn set_file_name(&mut self, file_name: impl AsRef<str>) {
        self.0.set_file_name(file_name.as_ref())
    }

    pub fn set_extension(&mut self, extension: impl AsRef<str>) -> bool {
        self.0.set_extension(extension.as_ref())
    }

    pub fn into_string(self) -> String {
        self.into_os_string().into_string().unwrap()
    }

    pub fn into_os_string(self) -> OsString {
        self.0.into_os_string()
    }

    pub fn into_boxed_path(self) -> Box<Utf8Path> {
        unsafe { Box::from_raw(Box::into_raw(self.0.into_boxed_path()) as *mut Utf8Path) }
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.0.reserve_exact(additional)
    }

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

// NB: Internal Path must only contain utf8 data
#[repr(transparent)]
#[derive(Hash)]
pub struct Utf8Path(Path);

impl Utf8Path {
    pub fn new(s: &(impl AsRef<str> + ?Sized)) -> &Utf8Path {
        unsafe { Utf8Path::assert_utf8(Path::new(s.as_ref())) }
    }

    pub fn from_path(path: &Path) -> Option<&Utf8Path> {
        path.as_os_str().to_str().map(|s| Utf8Path::new(s))
    }

    pub fn as_str(&self) -> &str {
        unsafe { assert_utf8(self.as_os_str()) }
    }

    pub fn as_os_str(&self) -> &OsStr {
        self.0.as_os_str()
    }

    pub fn to_path_buf(&self) -> Utf8PathBuf {
        Utf8PathBuf(self.0.to_path_buf())
    }

    pub fn is_absolute(&self) -> bool {
        self.0.is_absolute()
    }

    pub fn is_relative(&self) -> bool {
        self.0.is_relative()
    }

    pub fn has_root(&self) -> bool {
        self.0.has_root()
    }

    pub fn parent(&self) -> Option<&Utf8Path> {
        self.0.parent().map(|path| unsafe { Utf8Path::assert_utf8(path) })
    }

    pub fn ancestors(&self) -> Utf8Ancestors<'_> {
        Utf8Ancestors(self.0.ancestors())
    }

    pub fn file_name(&self) -> Option<&str> {
        self.0.file_name().map(|s| unsafe { assert_utf8(s) })
    }

    pub fn strip_prefix(&self, base: impl AsRef<Path>) -> Result<&Utf8Path, StripPrefixError> {
        self.0.strip_prefix(base).map(|path| unsafe { Utf8Path::assert_utf8(path) })
    }

    pub fn starts_with(&self, base: impl AsRef<Path>) -> bool {
        self.0.starts_with(base)
    }

    pub fn ends_with(&self, base: impl AsRef<Path>) -> bool {
        self.0.ends_with(base)
    }

    pub fn file_stem(&self) -> Option<&str> {
        self.0.file_stem().map(|s| unsafe { assert_utf8(s) })
    }

    pub fn extension(&self) -> Option<&str> {
        self.0.extension().map(|s| unsafe { assert_utf8(s) })
    }

    pub fn join(&self, path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
        Utf8PathBuf(self.0.join(&path.as_ref().0))
    }

    pub fn join_os(&self, path: impl AsRef<Path>) -> PathBuf {
        self.0.join(path)
    }

    pub fn with_file_name(&self, file_name: impl AsRef<str>) -> Utf8PathBuf {
        Utf8PathBuf(self.0.with_file_name(file_name.as_ref()))
    }

    pub fn with_extension(&self, extension: impl AsRef<str>) -> Utf8PathBuf {
        Utf8PathBuf(self.0.with_extension(extension.as_ref()))
    }

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
        unsafe {
            Utf8PathBuf(Box::from_raw(Box::into_raw(self) as *mut Path).into_path_buf())
        }
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
        self.0.next().map(|path| unsafe { Utf8Path::assert_utf8(path) })
    }
}

impl<'a> FusedIterator for Utf8Ancestors<'a> { }

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
        self.0.next().map(|component| unsafe { Utf8Component::new(component) })
    }
}

impl<'a> FusedIterator for Utf8Components<'a> { }

impl<'a> DoubleEndedIterator for Utf8Components<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|component| unsafe { Utf8Component::new(component) })
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
            Component::Prefix(prefix)   => Utf8Component::Prefix(Utf8PrefixComponent(prefix)),
            Component::RootDir          => Utf8Component::RootDir,
            Component::CurDir           => Utf8Component::CurDir,
            Component::ParentDir        => Utf8Component::ParentDir,
            Component::Normal(s)        => Utf8Component::Normal(assert_utf8(s)),
        }
    }

    pub fn as_str(&self) -> &'a str {
        unsafe { assert_utf8(self.as_os_str()) }
    }

    pub fn as_os_str(&self) -> &'a OsStr {
        match *self {
            Utf8Component::Prefix(prefix)   => prefix.as_os_str(),
            Utf8Component::RootDir          => Component::RootDir.as_os_str(),
            Utf8Component::CurDir           => Component::CurDir.as_os_str(),
            Utf8Component::ParentDir        => Component::ParentDir.as_os_str(),
            Utf8Component::Normal(s)        => OsStr::new(s),
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

// invariant: OsStr must be guaranteed to be utf8 data
unsafe fn assert_utf8(string: &OsStr) -> &str {
    &*(string as *const OsStr as *const str)
}
