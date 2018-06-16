use super::super::*;

use std::path::Path;
use std::fs::File;
use std::io::Read;

/// Matches the contents (as String) of a file located at the given `Path` against given `Matcher`.
///
/// If the file cannot be read for any reason, the matcher is considered to be failed.
pub fn content<P: AsRef<Path>>(content_matcher: Box<Matcher<String>>) -> Box<Matcher<P>> {
    return Box::new(move |path: &P| {
        match File::open(path) {
            Ok(mut file) => {
                let mut content = String::new();
                if let Err(err) = file.read_to_string(&mut content) {
                    return (Err(err) as Result<(), std::io::Error>).into();
                }
                return content_matcher.check(&content);
            },
            Err(err) => (Err(err) as Result<(), std::io::Error>).into()
        }
    });
}

/// Matches the contents (as bytes) of a file located at the given `Path` against given `Matcher`.
///
/// If the file cannot be read for any reason, the matcher is considered to be failed.
pub fn content_as_bytes<P: AsRef<Path>>(content_matcher: Box<Matcher<Vec<u8>>>) -> Box<Matcher<P>> {
    return Box::new(move |path: &P| {
        match File::open(path) {
            Ok(mut file) => {
                let mut content = Vec::<u8>::new();
                if let Err(err) = file.read_to_end(&mut content) {
                    return (Err(err) as Result<(), std::io::Error>).into();
                }
                return content_matcher.check(&content);
            },
            Err(err) => (Err(err) as Result<(), std::io::Error>).into()
        }
    });
}
