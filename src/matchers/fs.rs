/* Copyright 2017 Christopher Bacher
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! **TODO**

use super::super::*;

use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Matches if the given `Path` exists in the file system.
pub fn exists<P: AsRef<Path>>() -> Box<Matcher<P>> {
    Box::new(move |path: &P| {
        let path = path.as_ref();
        let builder = MatchResultBuilder::for_("path_exists");
        if !path.exists() {
            return builder.failed_because(&format!("Path `{}` does not exist", path.to_str().unwrap()));
        }
        builder.matched()
    })
}

/// Matches if the given `Path` exists and points to a file.
pub fn is_file<P: AsRef<Path>>() -> Box<Matcher<P>> {
    Box::new(move |path: &P| {
        let path = path.as_ref();
        let builder = MatchResultBuilder::for_("path_is_file");
        if !path.is_file() {
            return builder.failed_because(&format!("Path `{}` is not a file", path.to_str().unwrap()));
        }
        builder.matched()
    })
}

/// Matches if the given `Path` exists and points to a directory.
pub fn is_dir<P: AsRef<Path>>() -> Box<Matcher<P>> {
    Box::new(move |path: &P| {
        let path = path.as_ref();
        let builder = MatchResultBuilder::for_("path_is_dir");
        if !path.is_dir() {
            return builder.failed_because(&format!("Path `{}` is not a directory", path.to_str().unwrap()));
        }
        builder.matched()
    })
}


/// Matches the contents (as String) of a file located at the given `Path` against given `Matcher`.
///
/// If the file cannot be read for any reason, the matcher is considered to be failed.
pub fn content<P: AsRef<Path>>(content_matcher: Box<Matcher<String>>) -> Box<Matcher<P>> {
    Box::new(move |path: &P| {
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
    })
}

/// Matches the contents (as bytes) of a file located at the given `Path` against given `Matcher`.
///
/// If the file cannot be read for any reason, the matcher is considered to be failed.
pub fn content_as_bytes<P: AsRef<Path>>(content_matcher: Box<Matcher<Vec<u8>>>) -> Box<Matcher<P>> {
    Box::new(move |path: &P| {
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
    })
}


/// Wraps a `matcher` targeting a file with a given `name` which should reside under the checked diectory path.
///
/// The checked path is joined with the file `name`.
/// The resulting path is then checked with the stored `matcher`.
/// Note that no assumption is made if the path actually points to a valid file system entry.
/// This is the obligation of he wrapped matcher if it assumes the existance of the entry.
///
/// The preferred way to create a `FileMatcher` is by using the `fs_structure!` macro.
/// ```
/// assert_that!("/some/dir", fs_structure! {
///     "some file"; matches content(eq("The file content"))
/// });
/// ```
pub struct FileMatcher {
    name: String,
    matcher: Box<Matcher<PathBuf>>,
}

impl<P: AsRef<Path>> Matcher<P> for FileMatcher {
    fn check(&self, dir_path: &P) -> MatchResult {
        let file_path = dir_path.as_ref().join(&self.name);
        match self.matcher.check(&file_path) {
            MatchResult::Failed { name, reason } => MatchResult::Failed {
                name: format!("`{}` for file `{}`", name, file_path.to_str().unwrap()),
                reason
            },
            r@MatchResult::Matched { .. } => r
        }
    }
}


/// Wraps both a list of matchers targeting `files` inside of a directory and recursively further `DirectoryMatcher` representing `sub_directories`.
///
/// The checked path is joined with the directory `name` if present.
/// Otherwise the `DirectoryMatcher` is assumed to target the checked path directly, i.e., the same as an empty `name` or `"."` (in POSIX file systems).
/// Note that the path is assumed to actually point to a valid file system entry.
/// The resulting path is then checked all wrapped file matchers and recursively with all sub directory matchers.
///
/// The preferred way to create a `DirectoryMatcher` is by using the `fs_structure!` macro.
/// ```
/// assert_that!("/some/dir", fs_structure! {
///     "some file"; matches content(eq("The file content")),
///     "some other dir"; {
///         "file inside"; matches exists()
///     }
/// });
/// ```
pub struct DirectoryMatcher {
    name: Option<String>,
    sub_directories: Vec<DirectoryMatcher>,
    files: Vec<FileMatcher>,
    is_exhaustive: bool,
}

impl<P: AsRef<Path>> Matcher<P> for DirectoryMatcher {
    fn check(&self, dir_path: &P) -> MatchResult {
        let dir_path = dir_path.as_ref().join(self.name.as_ref().unwrap_or(&String::new()));
        let builder = MatchResultBuilder::for_("fs_structure");

        if !dir_path.is_dir() {
            return builder.failed_because(&format!("Path `{}` is not a directory", dir_path.to_str().unwrap()));
        }

        let mut entries = match self._collect_dir_entries(&dir_path) {
            Ok(set) =>  set,
            err@Err(_) => return err.into()
        };

        for file in self.files.iter() {
            entries.remove(&file.name);
            if let failed@MatchResult::Failed { .. } = file.matcher.check(&dir_path) {
                return failed;
            }
        }

        for sub_dir in self.sub_directories.iter() {
            entries.remove(sub_dir.name.as_ref().expect("Sub directories without name are not supported by `DirectoryMatcher`"));
            if let failed@MatchResult::Failed { .. } = sub_dir.check(&dir_path) {
                return failed;
            }
        }

        if self.is_exhaustive && !entries.is_empty() {
            return builder.failed_because(&format!(
                "Matchers for directory `{}` should be exhaustive but the following entries are not listed: {}",
                dir_path.to_string_lossy(),
                entries.into_iter().collect::<Vec<_>>().join(", ")
            ))
        }
        builder.matched()
    }
}

impl DirectoryMatcher {
    fn _collect_dir_entries(&self, dir_path: &Path) -> std::io::Result<BTreeSet<String>> {
        let mut entries = BTreeSet::new();
        match dir_path.read_dir() {
            Ok(iter) => for maybe_entry in iter {
                match maybe_entry {
                    Ok(entry) => entries.insert(entry.file_name().into_string().unwrap()),
                    Err(err) => return Err(err)
                };
            },
            Err(err) => return Err(err)
        }
        Ok(entries)
    }
}


/// Creates a `DirectoryMatcher` for inspecting whole file system structures.
///
/// The syntax is as follows.
/// Files and sub directory entries are listed in a comma separated list.
/// A file entry is written as follows: `FILE_NAME_EXPR; matches MATCHER_EXPR`.
/// The `FILE_NAME_EXPR` is an expression evaluating to an object implementing `std::convert::Into<String>`, e.g., `&str` or `String`.
/// The `MATCHER_EXPR` must evaluate to an `Matcher<std::path::PathBuf>`.
/// Note that this is true for an matcher factory returning a `Matcher<P> where P: AsRef<Path>`, e.g., `fs::exists, fs::content, ...`
///
/// A directory entry is written as: `DIRECTORY_NAME_EXPR; { CONTENTS }`
/// `DIRECTORY_NAME_EXPR` must evaluate to an object implementing `std::convert::Into<String>`, e.g., `&str` or `String`.
/// The `CONTENTS` can be anything which is accepted by `fs_structure!`, i.e., the macro is recursive.
///
/// By default the created `DirectoryMatcher` is non-exhaustive.
/// To create an exhaustive `DirectoryMatcher` (have a look at the struct's documentation) precede the entry listing by `exhaustive:`.
/// Note that `exhaustive:` applies to the current directory only, not to its sub directories.
/// Sub directories need their own `exhaustive:` directive if intended.
///
/// #Examples
/// **TODO**
#[macro_export]
macro_rules! fs_structure {

    ( @exhaustive $(exhaustive)+ ) => { true };

    ( @exhaustive ) => { false };

    ( @dir $name:expr ; { $($inner:tt)* } ) => {
        {
            let DirectoryMatcher { sub_directories, files, is_exhaustive, .. } = fs_structure!( $($inner)* );
            DirectoryMatcher {
                name: $name.into(),
                sub_directories,
                files,
                is_exhaustive,
            }
        }
    };

    ( @file $name:expr ; $matcher:expr ) => {
        FileMatcher {
            name: $name.into(),
            matcher: $matcher,
        }
    };

    ( @expand files [ $($files:tt)* ] dirs [ $($dirs:tt)* ] [ $($exhaustive:tt)* ] <- [ $name:expr ; { $($inner:tt)* }, $($rest:tt)* ] ) => {
        fs_structure!( @expand files [ $($files)* ] dirs [ fs_structure!( @dir $name ; { $($inner)* } ) , $($dirs)* ] [ $($exhaustive)* ] <- [ $($rest)* ] )
    };

    ( @expand files [ $($files:tt)* ] dirs [ $($dirs:tt)* ] [ $($exhaustive:tt)* ] <- [ $name:expr ; { $($inner:tt)* } ] ) => {
        fs_structure!( @expand files [ $($files)* ] dirs [ fs_structure!( @dir $name ; { $($inner)* } ) , $($dirs)* ] [ $($exhaustive)* ] <- [ ] )
    };

    ( @expand files [ $($files:tt)* ] dirs [ $($dirs:tt)* ] [ $($exhaustive:tt)* ] <- [ $name:expr; matches $matcher:expr , $($rest:tt)* ] ) => {
        fs_structure!( @expand files [ fs_structure!( @file $name ; $matcher ) , $($files)* ] dirs [ $($dirs)* ] [ $($exhaustive)* ] <- [ $($rest)* ] )
    };

    ( @expand files [ $($files:tt)* ] dirs [ $($dirs:tt)* ] [ $($exhaustive:tt)* ] <- [ $name:expr; matches $matcher:expr ] ) => {
        fs_structure!( @expand files [ fs_structure!( @file $name ; $matcher ) , $($files)* ] dirs [ $($dirs)* ] [ $($exhaustive)* ] <- [ ] )
    };

    ( @expand files [ $($files:tt)* ] dirs [ $($dirs:tt)* ] [ $($exhaustive:tt)* ] <- [ ] ) => {
        DirectoryMatcher {
            name: String::new(),
            sub_directories: vec![$($dirs)*],
            files: vec![$($files)*],
            is_exhaustive: fs_structure!( @exhaustive $($exhaustive)* ),
        }
    };

    ( exhaustive: $($tokens:tt)* ) => {
        fs_structure!( @expand files [ ] dirs [ ] [ exhaustive ] <- [ $($tokens)* ])
    };

    ( $($tokens:tt)* ) => {
        fs_structure!( @expand files [ ] dirs [ ] [ ] <- [ $($tokens)* ])
    };
}
