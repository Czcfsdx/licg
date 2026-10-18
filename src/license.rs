use licg_macros::{FmtLicenseFn, FromInputUpperFn, ListVariantsWithDocFn};
use std::fmt;

#[derive(Debug, FromInputUpperFn, ListVariantsWithDocFn, FmtLicenseFn)]
/// Supported Licenses
pub enum LicenseKind {
    /// Apache License 2.0
    #[template = "license-templates/Apache2"]
    Apache2,
    /// BSD 2-Clause "Simplified" License
    #[template = "license-templates/BSD2"]
    BSD2,
    /// BSD 3-Clause “New” or “Revised” License
    #[template = "license-templates/BSD3"]
    BSD3,
    /// GNU General Public License v2.0
    #[template = "license-templates/GPL2"]
    GPL2,
    /// GNU General Public License v3.0
    #[template = "license-templates/GPL3"]
    GPL3,
    /// MIT License
    #[template = "license-templates/MIT"]
    MIT,
}

#[derive(Debug)]
pub struct License<'a> {
    pub kind: LicenseKind,
    pub holder: &'a str,
    pub year: i32,
}

impl fmt::Display for License<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let holder = self.holder;
        let year = self.year;
        self.kind.fmt_license(f, year, holder)
    }
}
