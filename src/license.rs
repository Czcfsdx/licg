use licg_macros::{FmtLicenseFn, FromInputUpperFn, ListVariantsWithDocFn};
use std::fmt;

#[derive(Debug, FromInputUpperFn, ListVariantsWithDocFn, FmtLicenseFn)]
/// Supported Licenses
pub enum LicenseKind {
    /// Apache License 2.0
    #[template = "license-templates/Apache2"]
    Apache2,
    /// GNU Affero General Public License v3.0
    #[template = "license-templates/AGPL3"]
    AGPL3,
    /// BSD 2-Clause "Simplified" License
    #[template = "license-templates/BSD2"]
    BSD2,
    /// BSD 3-Clause “New” or “Revised” License
    #[template = "license-templates/BSD3"]
    BSD3,
    /// Boost Software License 1.0
    #[template = "license-templates/BSL"]
    BSL,
    /// Eclipse Public License 2.0
    #[template = "license-templates/EPL2"]
    EPL2,
    /// GNU General Public License v2.0
    #[template = "license-templates/GPL2"]
    GPL2,
    /// GNU General Public License v3.0
    #[template = "license-templates/GPL3"]
    GPL3,
    /// GNU Lesser General Public License v2.1
    #[template = "license-templates/LGPL2"]
    LGPL2,
    /// GNU Lesser General Public License v3.0
    #[template = "license-templates/LGPL3"]
    LGPL3,
    /// MIT License
    #[template = "license-templates/MIT"]
    MIT,
    /// Mozilla Public License 2.0
    #[template = "license-templates/MPL2"]
    MPL2,
    /// The Unlicense
    #[template = "license-templates/Unlicense"]
    Unlicense,
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
