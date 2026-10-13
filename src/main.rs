use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// A simple open source license generator
struct Cli {
    /// Name of the copyright holder in license
    #[arg(short, long)]
    name: Option<String>,

    /// Path of the license file
    #[arg(short, long, default_value = "./LICENSE")]
    path: String,

    /// Whether to overwrite the existing file
    #[arg(short, long)]
    force: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let path = args.path;
    let mut binding = File::options();
    let options = if args.force {
        binding.write(true).create(true)
    } else {
        binding.write(true).create_new(true)
    };
    let file = options.open(&path).or_else(|e| {
        if e.kind() == std::io::ErrorKind::AlreadyExists {
            // This will only happen if we set create_new to true in options
            // in which case the args.force should be false
            assert_eq!(args.force, false);
            Err(e).with_context(|| format!("Failed to create file which is already exists: {}\n       Use the flag -f/--force if you want to overwrite this file", path))
        } else {
            Err(e).with_context(|| format!("Failed to create file: {}", path))
        }
    })?;

    let name = args.name.or(get_default_holder_name()).unwrap_or_else(|| {
        let mut cmd = Cli::command();
        cmd.error(
            clap::error::ErrorKind::MissingRequiredArgument,
            "--name is required since we fail to get a default holder name",
        )
        .exit()
    });
    let holder = &name.trim();
    let year = get_current_year().context("Fail to get current year")? as usize;
    let license = MIT { holder, year };
    write!(&file, "{}", license).with_context(|| format!("Failed to write file: {}", path))?;
    Ok(())
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

// TODO(20261012-100823): Use local time instead UTC time.
// We assume that the current year cannot be 0 AD or BC.
fn get_current_year() -> Result<u64> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("SystemTime before UNIX EPOCH")?;
    let days = duration.as_secs() / 86400 + 719468;
    let era = days / 146097;
    let day_of_era = days - era * 146097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36524 - day_of_era / 146096) / 365;

    // TODO(20261012-101205): Maybe we don't need to calculate month since we only need year.
    let day_of_year = day_of_era - (year_of_era * 365 + year_of_era / 4 - year_of_era / 100);
    let month = (5 * day_of_year + 2) / 153;

    let year = year_of_era + era * 400;
    return if month <= 2 { Ok(year + 1) } else { Ok(year) };
}

// Get default holder name
// Now this function just try to get the user name in git as the
// holder name. If process fail, it will return None
fn get_default_holder_name() -> Option<String> {
    match Command::new("git").arg("config").arg("user.name").output() {
        Ok(output) => Some(String::from_utf8_lossy(&output.stdout).to_string()),
        Err(_) => None,
    }
}
