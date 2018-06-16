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

#[macro_use] extern crate galvanic_assert;
extern crate tempfile;

use galvanic_assert::matchers::eq;
use galvanic_assert::matchers::fs::*;
use std::fs::File;
use tempfile::tempdir;


mod content {
    use super::*;
    use std::io::Write;

    #[test]
    fn should_match_content_of_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("file.tmp");
        let file_content = "Temporary file content";
        write!(File::create(file_path.clone()).unwrap(), "{}", file_content).unwrap();

        assert_that!(&file_path, content(eq(file_content.to_owned())))
    }

    #[test]
    fn should_match_bytes_content_of_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("file.tmp");
        let file_content = "Temporary file content";
        write!(File::create(file_path.clone()).unwrap(), "{}", file_content).unwrap();

        assert_that!(&file_path, content_as_bytes(eq(file_content.as_bytes().to_vec())))
    }
}
