use std::fmt;
use std::fs::File;
use std::io::Write;

fn main() {
    // TODO: If LICENSE file already exists, don't modify it
    // unless given --force/-f flag.
    let file = File::create("LICENSE").unwrap();
    let year = 2026;
    let license = MIT {
        holder: "Czcfsdx",
        year: year,
    };
    write!(&file, "{}", license).unwrap();
}

#[derive(Debug)]
struct MIT<'a> {
    /// The MIT License
    holder: &'a str,
    year: usize,
}

impl fmt::Display for MIT<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let holder = self.holder;
        let year = self.year;
        write!(
            f,
            r#"MIT License

Copyright (c) {year} {holder}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE."#
        )
    }
}

//  1) BSD-2-Clause
//  2) BSD-3-Clause
//  3) Apache-2.0
//  4) MIT
//  5) MPL-2.0
//  6) ISC
//  7) GPL-2.0-only
//  8) GPL-3.0-only
//  9) LGPL-2.1-only
// 10) LGPL-3.0-only
// 11) AGPL-3.0-only
// 12) GPL-2.0-or-later
// 13) GPL-3.0-or-later
// 14) LGPL-2.1-or-later
// 15) LGPL-3.0-or-later
// 16) AGPL-3.0-or-later
// 17) Other (specify)

// check https://spdx.org/licenses/

// Utils functions
// use std::time::{SystemTime, UNIX_EPOCH};
// TODO: Use local time instead UTC time.
// fn get_current_year() -> u32 {
//     match SystemTime::now().duration_since(UNIX_EPOCH) {
//         Ok(duration) => {
//             let timestamp = duration.as_secs();

//         },
//         Err(e) => { panic!("Fail to get SystemTime: {}", e) },
//     }
// }
