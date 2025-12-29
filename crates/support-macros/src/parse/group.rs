use quote::format_ident;
use syn::parse::{Parse, ParseStream, Peek};
use syn::{Path, Result, Token};

/// Arguments for the [group](crate::group) attribute macro.
#[derive(Debug)]
pub struct GroupArgs {
    crate_name: Option<Path>,
}

impl GroupArgs {
    /// Returns the name of the configured crate or the default.
    pub fn crate_name(&self) -> Path {
        self.crate_name.clone().unwrap_or_else(|| format_ident!("picus_support").into())
    }

    fn new() -> Self {
        Self { crate_name: None }
    }

    fn new_with_path(path: Path) -> Self {
        Self {
            crate_name: Some(path),
        }
    }
}

impl Default for GroupArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl Parse for GroupArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        log::debug!("input = {input}");
        // Grammar:
        //  args : 'crate' '=' <ident> | ε
        if input.is_empty() {
            return Ok(Self::default());
        }

        ensure(&input, Token![crate])?;
        let _: Token![crate] = input.parse()?;
        ensure(&input, Token![=])?;
        let _: Token![=] = input.parse()?;
        input.parse().map(Self::new_with_path)
    }
}

fn ensure<T: Peek>(input: &ParseStream, token: T) -> Result<()> {
    log::debug!("ensuring for {input}");
    let la = input.lookahead1();
    la.peek(token).then_some(()).ok_or_else(|| la.error())
}
