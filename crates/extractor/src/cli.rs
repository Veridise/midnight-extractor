use log::Level;
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use crate::{
    chips::{Chip, Type},
    config::{Action, AppConfig, FailMode, HarnessConfig, LoggingConfig, OutputFormat},
    instructions::Instructions,
    picus::{prelude::Preludes, PicusConfig},
    utils::parse_constants_file,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(value_enum)]
    instructions: Vec<Instructions>,
    #[arg(long, value_enum)]
    chip: Option<Chip>,
    #[arg(long, value_delimiter = ',')]
    format: Vec<OutputFormat>,
    #[arg(long, value_enum)]
    r#type: Option<Type>,
    #[arg(short, long)]
    output: Option<PathBuf>,
    #[arg(long, value_delimiter = ',')]
    constants: Vec<String>,
    #[arg(long)]
    constants_file: Option<PathBuf>,
    #[arg(long, value_delimiter = ',')]
    method_whitelist: Vec<String>,
    #[arg(long, value_delimiter = ',')]
    method_blacklist: Vec<String>,
    //#[arg(long)]
    //pub picus_expr_cutoff: Option<usize>,
    //#[arg(long)]
    //verify_with_picus: bool,
    #[arg(long)]
    pub log: Option<PathBuf>,
    #[arg(long, default_value_t = Level::Info)]
    pub log_level: Level,
    //#[arg(long)]
    //pub picus_short_names: bool,
    //#[arg(long)]
    //pub zeroed_inputs: bool,
    //#[arg(long, default_value_t = 10)]
    //pub prover_k: u32,
    //#[arg(long)]
    //pub constraint_missing_outputs: bool,
    //#[arg(long)]
    //pub prover_constrain_outputs: bool,
    #[arg(long)]
    pub disable_decomposition_rewrite: bool,
    #[arg(long)]
    pub debug_comments: bool,
    #[arg(long)]
    pub picus_no_opt: bool,
    #[arg(long)]
    pub no_opt: bool,
    #[arg(long)]
    pub fail_fast: bool,
    #[arg(long)]
    pub prelude: Option<Preludes>,
    #[arg(long)]
    pub dump_ir: bool,
    #[arg(long)]
    pub list: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("Cannot set --constants and --constants-file at the same time")]
    ConstantsConfigErr,
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

impl AppConfig for Cli {
    type Error = CliError;

    fn logging(&self) -> Option<LoggingConfig> {
        self.log.as_deref().map(|path| LoggingConfig::new(path, self.log_level))
    }

    fn fail_mode(&self) -> FailMode {
        if self.fail_fast {
            FailMode::Fast
        } else {
            FailMode::Continue
        }
    }

    fn setup(&mut self) -> std::result::Result<(), Self::Error> {
        match (self.constants.is_empty(), self.constants_file.as_ref()) {
            (false, Some(_)) => return Err(CliError::ConstantsConfigErr),
            (true, Some(path)) => {
                let f = File::open(path)?;
                let reader = BufReader::new(f);
                self.constants = parse_constants_file(reader)?
            }
            (true, None) => {
                log::warn!("No constants provided! Some circuits may fail to extract due to this.");
            }
            (false, None) => {} // Don't do anything
        }
        Ok(())
    }

    fn instructions(&self) -> &[Instructions] {
        &self.instructions
    }

    fn chip(&self) -> Option<Chip> {
        self.chip
    }

    fn r#type(&self) -> Option<Type> {
        self.r#type
    }

    fn method_whitelist(&self) -> &[String] {
        &self.method_whitelist
    }

    fn method_blacklist(&self) -> &[String] {
        &self.method_blacklist
    }

    fn constants(&self) -> &[String] {
        &self.constants
    }

    fn output(&self) -> Option<&Path> {
        self.output.as_deref()
    }

    fn prelude(&self) -> Option<Preludes> {
        self.prelude
    }

    fn picus_config(&self) -> PicusConfig {
        PicusConfig::new(!(self.picus_no_opt || self.no_opt), self.prelude)
    }

    fn dump_ir(&self) -> bool {
        self.dump_ir
    }

    fn action(&self) -> Action {
        if self.list {
            Action::List
        } else {
            Action::Extract
        }
    }

    fn formats(&self) -> &[OutputFormat] {
        if self.format.is_empty() {
            return &[OutputFormat::Picus];
        }
        &self.format
    }

    fn harness_config(&self) -> HarnessConfig {
        HarnessConfig::new(
            &self.constants,
            self.debug_comments,
            !self.disable_decomposition_rewrite,
        )
    }

    fn optimize_ir(&self) -> bool {
        !self.no_opt
    }
}

//impl Cli {
//    pub fn output(&self) -> Option<&Path> {
//        self.output.as_deref()
//    }
//
//    pub fn chip(&self) -> Option<Chip> {
//        self.chip
//    }
//
//    pub fn r#type(&self) -> Option<Type> {
//        self.r#type
//    }
//
//    pub fn constants(&self) -> &[String] {
//        &self.constants
//    }
//
//    pub fn method_whitelist(&self) -> &[String] {
//        &self.method_whitelist
//    }
//
//    pub fn method_blacklist(&self) -> &[String] {
//        &self.method_blacklist
//    }
//
//    pub fn instructions(&self) -> &[Instructions] {
//        &self.instructions
//    }
//
//    pub fn prepare_constants(&mut self) -> Result<()> {}
//}
