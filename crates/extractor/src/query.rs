//! Types for selecting what harness need to be extracted.

use std::fmt;

use crate::{
    chips::{Chip, Type},
    instructions::Instructions,
};

/// The parts that make up a harness name.
pub struct HarnessName<'a> {
    instruction: &'a str,
    method: &'a str,
    chip: &'a str,
    r#type: &'a str,
}

impl<'a> HarnessName<'a> {
    pub fn instruction(&self) -> &'a str {
        self.instruction
    }

    pub fn method(&self) -> &'a str {
        self.method
    }

    pub fn chip(&self) -> &'a str {
        self.chip
    }

    pub fn r#type(&self) -> &'a str {
        self.r#type
    }
}

impl<'a> From<&'a str> for HarnessName<'a> {
    fn from(value: &'a str) -> Self {
        let parts: Vec<_> = value.split('/').collect();
        assert_eq!(parts.len(), 4);
        Self {
            instruction: parts[0],
            method: parts[1],
            chip: parts[2],
            r#type: parts[3],
        }
    }
}

/// Query expression
enum QueryPart {
    Exact(String),
    Negated(String),
    Wildcard,
    Or(Vec<QueryPart>),
    And(Vec<QueryPart>),
}

impl QueryPart {
    fn matches<'s: 'p, 'p>(&'s self, part: &'p str) -> bool {
        match self {
            Self::Exact(s) => s == part,
            Self::Negated(s) => s != part,
            Self::Wildcard => true,
            Self::Or(ops) => ops.iter().any(|op| op.matches(part)),
            Self::And(ops) => ops.iter().all(|op| op.matches(part)),
        }
    }

    fn needs_parens(&self) -> bool {
        matches!(self, Self::Or(_) | Self::And(_))
    }

    fn new_instructions_part(instructions: &[Instructions]) -> Self {
        if instructions.is_empty() {
            QueryPart::Wildcard
        } else {
            QueryPart::Or(
                instructions
                    .iter()
                    .map(ToString::to_string)
                    .map(QueryPart::Exact)
                    .collect::<Vec<_>>(),
            )
        }
    }

    fn new_part<T: ToString>(key: Option<T>) -> Self {
        key.as_ref()
            .map(ToString::to_string)
            .map(QueryPart::Exact)
            .unwrap_or(QueryPart::Wildcard)
    }

    fn new_method_part(whitelist: &[String], blacklist: &[String]) -> Self {
        if whitelist.is_empty() && blacklist.is_empty() {
            return QueryPart::Wildcard;
        }

        let whitelist = (!whitelist.is_empty()).then(|| {
            QueryPart::Or(whitelist.iter().cloned().map(QueryPart::Exact).collect::<Vec<_>>())
        });

        QueryPart::And(
            whitelist
                .into_iter()
                .chain(blacklist.iter().cloned().map(QueryPart::Negated))
                .collect(),
        )
    }
}

fn interleave(parts: &[QueryPart], sep: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for (n, part) in parts.iter().enumerate() {
        if n > 0 {
            write!(f, "{sep}")?;
        }
        if part.needs_parens() {
            write!(f, "(")?;
        }
        write!(f, "{part:?}")?;
        if part.needs_parens() {
            write!(f, ")")?;
        }
    }
    Ok(())
}

fn exact(s: &str, neg: bool, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if neg {
        write!(f, "!")?;
    }
    let has_ws = s.contains(char::is_whitespace);

    if has_ws {
        write!(f, "\"")?;
    }
    write!(f, "{s}")?;
    if has_ws {
        write!(f, "\"")?;
    }
    Ok(())
}

impl fmt::Debug for QueryPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact(s) => exact(s, false, f),
            Self::Negated(s) => exact(s, true, f),
            Self::Wildcard => write!(f, "*"),
            Self::Or(parts) => interleave(parts, "|", f),
            Self::And(parts) => interleave(parts, "&", f),
        }
    }
}

/// The complete query with the 4 parts
pub struct Query {
    instruction: QueryPart,
    method: QueryPart,
    chip: QueryPart,
    r#type: QueryPart,
}

impl Query {
    pub fn matches<'s: 'n, 'n>(&'s self, name: impl Into<HarnessName<'n>>) -> bool {
        let name = name.into();
        self.instruction.matches(name.instruction())
            && self.method.matches(name.method())
            && self.chip.matches(name.chip())
            && self.r#type.matches(name.r#type())
    }

    /// Constructs a query from the CLI arguments
    pub fn new(
        instructions: &[Instructions],
        chip: Option<Chip>,
        r#type: Option<Type>,
        whitelist: &[String],
        blacklist: &[String],
    ) -> Self {
        Self {
            instruction: QueryPart::new_instructions_part(instructions),
            method: QueryPart::new_method_part(whitelist, blacklist),
            chip: QueryPart::new_part(chip),
            r#type: QueryPart::new_part(r#type),
        }
    }
}

impl fmt::Debug for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}/{:?}/{:?}/{:?}",
            self.instruction, self.method, self.chip, self.r#type
        )
    }
}
