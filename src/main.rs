use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

mod license;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// A simple open source license generator
struct Cli {
    /// License to generate
    license: Option<String>,

    /// Name of the copyright holder in license
    #[arg(short, long)]
    name: Option<String>,

    /// Path of the license file
    #[arg(short, long, default_value = "./LICENSE")]
    path: String,

    /// List the supported licenses and do not generate any file
    #[arg(short, long)]
    list: bool,

    /// Whether to overwrite the existing file
    #[arg(short, long)]
    force: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // if --list is provided, do not generate any license file
    if args.list {
        license::list_license_kind();
        return Ok(());
    }

    // parse the license kind
    let input = args.license.unwrap_or_else(|| {
        let mut cmd = Cli::command();
        cmd.error(
            clap::error::ErrorKind::MissingRequiredArgument,
            "LICENSE is required without --list, but not provided",
        )
        .exit()
    });
    let kind = license::LicenseKind::from_input(&input).unwrap_or_else(|| {
        let mut cmd = Cli::command();
        cmd.error(
            clap::error::ErrorKind::InvalidValue,
            format!(
                "Unknown license: {}\n       Use -l/--list to see the supported licenses",
                input
            ),
        )
        .exit()
    });

    // parse holder_name
    let name = args.name.or(get_default_holder_name()).unwrap_or_else(|| {
        let mut cmd = Cli::command();
        cmd.error(
            clap::error::ErrorKind::MissingRequiredArgument,
            "--name <NAME> is required since we fail to get a default holder name, but not provided",
        )
        .exit()
    });
    let holder = name.trim();

    let year = get_current_year().context("Fail to get current year")? as usize;
    let license = license::License { kind, holder, year };

    // create license file
    let path = args.path;
    let mut binding = File::options();
    let options = if args.force {
        binding.write(true).truncate(true).create(true)
    } else {
        binding.write(true).truncate(true).create_new(true)
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

    write!(&file, "{}", license).with_context(|| format!("Failed to write file: {}", path))?;
    println!("Generated {:?} in {}", license, path);
    Ok(())
}

// Utils functions

// Get the current year
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
