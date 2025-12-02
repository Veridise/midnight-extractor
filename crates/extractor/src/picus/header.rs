use chrono::Utc;
use std::fmt;

fn dep_header(
    f: &mut fmt::Formatter<'_>,
    name: &str,
    version: &str,
    checksum: Option<&str>,
) -> fmt::Result {
    write!(f, "; {name} {version}")?;
    if let Some(checksum) = checksum {
        write!(f, " ({checksum})")?;
    }
    writeln!(f)
}

pub struct Header;

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "; vim: set filetype=scheme :")?;
        writeln!(f, "; Midnight analyzer {}", env!("CARGO_PKG_VERSION"))?;
        dep_header(
            f,
            "Haloumi frontend",
            env!("HALO2_LLZK_FRONTEND_VERSION"),
            option_env!("HALO2_LLZK_FRONTEND_CHECKSUM"),
        )?;
        dep_header(
            f,
            "Midnight circuits",
            env!("MIDNIGHT_CIRCUITS_VERSION"),
            option_env!("MIDNIGHT_CIRCUITS_CHECKSUM"),
        )?;
        writeln!(f, "; Timestamp {}", Utc::now())
    }
}
