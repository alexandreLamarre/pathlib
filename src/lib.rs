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
        wild_nested_n_star : ("dir_name/*filename",true),
        // wild_escaped : ('\*',false), FIXME: this is not supported by rust as an escape character
    }
}
