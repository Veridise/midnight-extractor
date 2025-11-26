/// Information required for executing a harness.
pub struct Ctx<'s> {
    constants: &'s [String],
    debug_comments: bool,
    disable_decomposition_pattern: bool,
}

impl<'s> Ctx<'s> {
    pub fn constants(&self) -> &[String] {
        self.constants
    }
}
