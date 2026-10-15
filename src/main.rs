use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use time::OffsetDateTime;

mod license;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// A simple open source license generator
struct Cli {
    /// License to generate, required unless provide --list flag
    #[arg(required_unless_present("list"))]
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

    /// Do not perform any operations that modify the filesystem and only show the debug output
    #[arg(long)]
    dry_run: bool
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // if --list is provided, do not generate any license file
    if args.list {
        license::list_license_kind();
        return Ok(());
    }

    // Cli::parse() will signal error and exit when args.license is not provide,
    // So we can safely unwrap it.
    let input = args.license.unwrap();
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

    let year = OffsetDateTime::now_local().context("Fail to get current year")?.year();
    let license = license::License { kind, holder, year };

    let path = args.path;

    if args.dry_run {
        println!("Generate {:?}", license);
        println!("Write following content into file {}:", path);
        println!("==================================================");
        print!("{}", license); // Do not need a new line
        println!("==================================================");
        return Ok(());
    }

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

// Get default holder name
// Now this function just try to get the user name in git as the
// holder name. If process fail, it will return None
fn get_default_holder_name() -> Option<String> {
    match Command::new("git").arg("config").arg("user.name").output() {
        Ok(output) => Some(String::from_utf8_lossy(&output.stdout).to_string()),
        Err(_) => None,
    }
}
