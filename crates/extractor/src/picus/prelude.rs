#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum Preludes {
    Spread,
}

const SPREAD_PRELUDE: &str = include_str!("spread.picus.inc");

impl std::fmt::Display for Preludes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Preludes::Spread => {
                writeln!(f, "{SPREAD_PRELUDE}")
            }
        }
    }
}
