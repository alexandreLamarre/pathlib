#![allow(dead_code)] // FIXME: remove when development is done
use std::ops;

/// Drive exists but is not accessible
static _WINERROR_NOT_READY: i32 = 21;
/// (Original python implementation description) : fix for bpo-35306
static _WINERROR_INVALID_NAME: i32 = 123;
/// Broken symlink pointing to itself
static _WINERROR_CANT_RESOLVE_FILENAME: i32 = 1921;

// TODO: don't forget to define ENOENT, ENOTDIR, EBADF, ELOOP as integer error codes
// for handling system error calls

/// Whether this patterns needs actual dynamic matching,
/// or can be looked up directly as a file.
///
/// @warning Not expected to work with escaped wildcard characters,
/// see https://static.rust-lang.org/doc/master/reference.html#literals
fn _is_wildcard_pattern(pattern: &str) -> bool {
    pattern.contains('*') || pattern.contains('?') || pattern.contains('[')
}

/// Pure path for handling abstractions in filesystem paths
/// an object oriented way.
pub trait PurePath {
    // === Conversion ===
    fn new(path: &str) -> Self;
    fn to_str(&self) -> &str;
    fn bytes(&self) -> Vec<u8>;

    // === Getters ===
    fn parts(&self) -> Vec<&str>;
    fn drive(&self) -> Option<&str>;
    fn root(&self) -> Option<&str>;
    fn anchor(&self) -> Option<&str>;
    fn parents(&self) -> Vec<&str>;
    fn parent(&self) -> Option<&str>;
    fn name(&self) -> Option<&str>;
    fn suffix(&self) -> Option<&str>;
    fn suffixes(&self) -> Vec<&str>;
    fn stem(&self) -> Option<&str>;
    // fn as_posix_path(&self) -> &std::path::Path; FIXME: implement later
    fn as_uri(&self) -> &str;

    // === Path properties ===
    fn is_absolute(&self) -> bool;
    fn is_relative_to(&self, other: &Self) -> bool;
    fn is_reserved(&self) -> bool;

    // === Path transformations ===
    fn join_path(&self, other: &Self) -> Self;
    fn match_expr(&self, pattern: &str) -> bool;
    fn relative_to(&self, other: &Self) -> Self;
    fn with_name(&self, name: &str) -> Self;
    fn with_stem(&self, stem: &str) -> Self;
    fn with_suffix(&self, suffix: &str) -> Self;
}

/// Object oriented implementation of a filesystem paths
pub trait SystemPath {
    fn new(path: &str) -> Self;
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;
    fn is_symlink(&self) -> bool;
    fn is_executable(&self) -> bool;
    fn is_readable(&self) -> bool;
    fn is_writable(&self) -> bool;
}

/// Path implementation enum for the `Path` type.
///
/// Used to wrap the system specific implementation of the user exposed `Path` struct.
enum PathImplementation {
    Windows(PureWindowsPath),
    Posix(PurePosixPath),
    Pure(MockPath),
}

/// Mock filesystem path for testing Path trait
#[derive(Debug, Clone, PartialEq, Eq)]
struct MockPath {
    path: String,
    // parts: Vec<String>,
    // drive: String,
    // root: String,
    // anchor: String,
    // parents: Vec<String>,
    // parent: String,
    // name: String,
    // suffix: String,
    // suffixes: Vec<String>,
    // stem: String,
}

impl PurePath for MockPath {
    fn new(path: &str) -> Self {
        MockPath {
            path: path.to_string(),
        }
    }

    fn to_str(&self) -> &str {
        self.path.as_str()
    }
    fn bytes(&self) -> Vec<u8> {
        self.path.as_bytes().to_vec()
    }

    // fn glob(&self, pattern: &str) -> std::slice::Iter<'_, Self>
    // where
    //     Self: std::marker::Sized,
    // {
    //     self.path.into_iter()
    // }
    // fn iterdir(&self) -> std::slice::Iter<'_, Self>
    // where
    //     Self: std::marker::Sized,
    // {
    //     self.path.split('/').map(|s| MockPath { path: s.to_string() }).collect::<Vec<_>>().into_iter()
    // }
}

impl ToString for MockPath {
    fn to_string(&self) -> String {
        self.path.clone()
    }
}

/// Implement '/' operator for Path abstractions between paths
impl ops::Div<MockPath> for MockPath {
    type Output = MockPath;
    fn div(self, rhs: MockPath) -> MockPath {
        MockPath {
            path: format!("{}/{}", self.path, rhs.path),
        }
    }
}

/// Implement '/' for path abstractions between paths and string slices
impl ops::Div<&str> for MockPath {
    type Output = MockPath;
    fn div(self, rhs: &str) -> MockPath {
        MockPath {
            path: format!("{}/{}", self.path, rhs),
        }
    }
}

/// Implement '/' for path abstractions between paths and strings
impl ops::Div<String> for MockPath {
    type Output = String;
    fn div(self, rhs: String) -> String {
        format!("{}/{}", self.path, rhs)
    }
}

/// Pure path implementation for non-windows systems.
///
/// On a POSIX system, instantiating a PurePath should return this object
/// However, you can also instantiate it directly on any system
pub struct PurePosixPath {
    path: String,
}

/// Pure path implementation for windows systems.
///
/// On a Windows system, instantiating a PurePath should return this object
/// However, you can also instantiate it directly on any system
pub struct PureWindowsPath {
    path: String,
}

/// Object oriented filesystems for Rust.
///
/// Should be used as the default object for handling filesystem paths.
/// If explicit control is desired, use `WindowsPath` or `PosixPath` instead.
///
/// For convenience, a `MockPath` is also provided for testing purposes - it doesn't interact with the system's filesystem.
pub struct Path {
    path: PathImplementation,
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! wildcard_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    assert_eq!(expected, _is_wildcard_pattern(input));
                }
            )*
        }
    }
    wildcard_tests! {
        wild_empty : ("", false),
        wild_star : ("*", true),
        wild_question : ("?", true),
        wild_bracket : ("[", true),
        wild_nested : ("dir_name/filename",false),
        wild_nested_n_star : ("dir_name/*.extension",true),
        // wild_escaped : ('\*',false), FIXME: this is not supported by rust as an escape character
    }

    macro_rules! mock_path_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    let path = MockPath::new(input);
                    assert_eq!(path.to_str(), expected);
                }
            )*
        }
    }

    mock_path_tests! {
        path_empty : ("", ""),
        path_slash : ("/", "/"),
        // path_slash_slash : ("//", "/"), #FIXME : should be supported, but isn't
        path_name : ("dirname/filename", "dirname/filename"),
    }

    #[test]
    fn mock_path_concat() {
        let path = MockPath::new("dirname/subdir");
        let concat = path / MockPath::new("filename.extension");
        assert_eq!(concat.to_str(), "dirname/subdir/filename.extension");

        let path2 = MockPath::new("dirname/subdir");
        let concat2 = path2 / "filename.extension";
        assert_eq!(concat2.to_str(), "dirname/subdir/filename.extension");
    }
}
