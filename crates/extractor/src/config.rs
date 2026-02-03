use std::path::{Path, PathBuf};

use mdnt_extractor_core::harness::Ctx;

use crate::{
    chips::{Chip, Type},
    instructions::Instructions,
    picus::{prelude::Preludes, PicusConfig},
};

pub enum Action {
    List,
    Extract,
}

pub enum FailMode {
    Fast,
    Continue,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum OutputFormat {
    Picus,
}

pub trait AppConfig {
    type Error: std::error::Error + Sync + Send + 'static;

    fn logging(&self) -> Option<LoggingConfig>;

    fn setup(&mut self) -> Result<(), Self::Error>;

    fn instructions(&self) -> &[Instructions];

    fn chip(&self) -> Option<Chip>;

    fn ignore_chips(&self) -> &[Chip];

    fn r#type(&self) -> Option<Type>;

    fn method_whitelist(&self) -> &[String];

    fn method_blacklist(&self) -> &[String];

    fn constants(&self) -> &[String];

    fn output(&self) -> Option<&Path>;

    fn prelude(&self) -> Option<Preludes>;

    fn picus_config(&self) -> PicusConfig;

    fn harness_config(&self) -> HarnessConfig;

    fn dump_ir(&self) -> bool;

    fn fail_mode(&self) -> FailMode;

    fn action(&self) -> Action;

    fn formats(&self) -> &[OutputFormat];

    fn optimize_ir(&self) -> bool;
}

pub struct LoggingConfig {
    path: PathBuf,
    level: log::Level,
}

impl LoggingConfig {
    pub fn new(path: impl AsRef<Path>, level: log::Level) -> Self {
        LoggingConfig {
            path: PathBuf::from(path.as_ref()),
            level,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn level(&self) -> log::Level {
        self.level
    }
}

pub struct HarnessConfig<'s> {
    constants: &'s [String],
    debug_comments: bool,
    enable_decomposition_rewrite: bool,
    allow_injected_ir_for_outputs: bool,
}

impl<'s> HarnessConfig<'s> {
    pub fn new(
        constants: &'s [String],
        debug_comments: bool,
        enable_decomposition_rewrite: bool,
        allow_injected_ir_for_outputs: bool,
    ) -> Self {
        Self {
            constants,
            debug_comments,
            enable_decomposition_rewrite,
            allow_injected_ir_for_outputs,
        }
    }

    pub(crate) fn make_ctx(&self) -> Ctx {
        Ctx::new(
            self.constants,
            self.debug_comments,
            !self.enable_decomposition_rewrite,
            self.allow_injected_ir_for_outputs,
        )
    }
}
