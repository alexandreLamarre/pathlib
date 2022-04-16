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

/// Pure path for handling IO operations in
/// an object oriented way.
pub trait PurePath {
    fn new(path: &str) -> Self;
    fn to_str(&self) -> &str;
}

/// High-Level Object oriented wrapper for handling filesystem paths
pub trait Path {
    fn new(path: &str) -> Self;
    fn to_str(&self) -> &str;
}

/// Mock filesystem path for testing Path trait
struct MockPath {
    path: String,
}

impl Path for MockPath {
    fn new(path: &str) -> Self {
        MockPath {
            path: path.to_string(),
        }
    }

    fn to_str(&self) -> &str {
        self.path.as_str()
    }
}

impl ToString for MockPath {
    fn to_string(&self) -> String {
        self.path.clone()
    }
}

/// Implement '/' operator for Path abstractions
/// FIXME: this should avoid using Box to wrap the arguments because it
/// produces a more verbose user API
impl ops::Div<Box<dyn ToString>> for MockPath {
    type Output = MockPath;
    fn div(self, rhs: Box<dyn ToString>) -> MockPath {
        MockPath {
            path: format!("{}/{}", self.path, rhs.to_string()),
        }
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
        let concat = path / Box::new(MockPath::new("filename.extension"));
        assert_eq!(concat.to_str(), "dirname/subdir/filename.extension");

        let path2 = MockPath::new("dirname/subdir");
        let concat2 = path2 / Box::new("filename.extension");
        assert_eq!(concat2.to_str(), "dirname/subdir/filename.extension");
    }
}
