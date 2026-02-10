use std::{borrow::Cow, fmt, fs::File, io::Write as _, path::Path};

use anyhow::{bail, Result};
use haloumi::ir_gen::circuit::resolved::ResolvedIRCircuit;
use haloumi_picus::PicusParamsBuilder;
use log::Log;

use crate::{
    config::{AppConfig, FailMode, LoggingConfig, OutputFormat},
    picus::{write_picus_output, PicusConfig},
    query::Query,
};
use mdnt_extractor_core::harness::{Ctx, Harness};

mod error;

use error::AppError;

pub struct App<Config> {
    config: Config,
    query: Query,
}

fn setup_logging(config: Option<LoggingConfig>) -> Result<()> {
    let env_logger = Box::new(env_logger::Builder::from_default_env().build());
    let mut loggers: Vec<Box<dyn Log>> = vec![env_logger];
    if let Some(config) = config {
        loggers.push(simplelog::WriteLogger::new(
            config.level().to_level_filter(),
            Default::default(),
            File::create(config.path())?,
        ));
    }

    multi_log::MultiLogger::init(loggers, log::Level::Trace)?;
    Ok(())
}

enum OptStep {
    ConstantFold,
    Canonicalization,
}

impl<Config> App<Config>
where
    Config: AppConfig + fmt::Debug,
{
    pub fn new(mut config: Config) -> Result<Self> {
        setup_logging(config.logging())?;
        log::debug!("Config = {config:?}");

        config.setup()?;

        let query = Query::new(
            config.instructions(),
            config.chip(),
            config.ignore_chips(),
            config.r#type(),
            config.method_whitelist(),
            config.method_blacklist(),
        );

        Ok(Self { config, query })
    }

    fn output_base(&self) -> Result<Cow<'_, Path>> {
        let path = if let Some(path) = self.config.output() {
            Cow::Borrowed(path)
        } else {
            // Default is $PWD/picus_files
            std::env::current_dir().map(|dir| Cow::Owned(dir.join("picus_files")))?
        };

        // Create if it doesn't exist.
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }

        if !path.is_dir() {
            bail!("Output path {} must be a directory", path.display());
        }

        Ok(path)
    }

    fn dump_ir(
        &self,
        name: &'static str,
        output_base: impl AsRef<Path>,
        ir: &ResolvedIRCircuit,
    ) -> Result<()> {
        let output_dir = output_base.as_ref().join(name);
        std::fs::create_dir_all(&output_dir)?;

        let output_path = output_dir.join("dump.ir");
        let mut output_file = File::create(&output_path)?;
        writeln!(output_file, "{}", ir.display())?;
        log::info!("Saved IR dump output in {}", output_path.display());
        Ok(())
    }

    fn check_validation(&self, status: Result<()>, step: OptStep) -> Result<()> {
        if let Err(err) = status {
            log::error!(
                "Validation error after {}: {err}",
                match step {
                    OptStep::ConstantFold => "constant folding",
                    OptStep::Canonicalization => "canonicalization",
                }
            );
            return Err(anyhow::anyhow!(
                "{} pass failed",
                match step {
                    OptStep::ConstantFold => "Constant fold",
                    OptStep::Canonicalization => "Canonicalization",
                }
            ));
        }

        Ok(())
    }

    fn optimize_ir(&self, ir: &mut ResolvedIRCircuit) -> Result<()> {
        ir.constant_fold()?;
        self.check_validation(Ok(ir.validate()?), OptStep::ConstantFold)?;
        ir.canonicalize();
        self.check_validation(Ok(ir.validate()?), OptStep::Canonicalization)
    }

    fn select_harness(&self) -> Vec<(&'static str, Harness)> {
        mdnt_harnesses::harnesses()
            .filter_map(|entry| {
                let matched = self.query.matches(entry.name());
                if !matched {
                    log::debug!("Ignoring harness {}", entry.name());
                }
                matched.then_some((entry.name(), entry.harness()))
            })
            .collect()
    }

    pub fn run(&mut self) -> Result<()> {
        log::info!("Selecting harnesses matching {:?}", self.query);
        let harness = self.select_harness();
        match self.config.action() {
            crate::config::Action::List => {
                self.print_harness_list(harness);
                Ok(())
            }
            crate::config::Action::Extract => self.extract(harness),
        }
    }

    fn print_harness_list(&self, harness: Vec<(&'static str, Harness)>) {
        for (name, _) in harness {
            println!("{name}");
        }
    }

    fn extract_one(
        &self,
        name: &'static str,
        harness: Harness,
        ctx: &Ctx,
        output_base: &Path,
        picus_config: &PicusConfig,
    ) -> Result<(), AppError> {
        log::info!("Extracting harness {name}");

        let mut ir = harness(ctx).map_err(AppError::harness(name))?;
        if self.config.optimize_ir() {
            self.optimize_ir(&mut ir).map_err(AppError::opt(name))?;
        }
        if self.config.dump_ir() {
            self.dump_ir(name, output_base, &ir).map_err(AppError::ir_dump(name))?;
        }
        for format in self.config.formats() {
            match format {
                OutputFormat::Picus => write_picus_output(
                    picus_config,
                    name,
                    output_base,
                    &ir,
                    PicusParamsBuilder::new(),
                )
                .map_err(AppError::picus(name))?,
            }
        }

        Ok(())
    }

    fn handle_extract_result(
        &self,
        extract: impl FnOnce() -> Result<(), AppError>,
        summary: &mut Summary,
    ) -> Result<()> {
        summary.generated += 1;
        match extract() {
            Err(err) => match self.config.fail_mode() {
                FailMode::Fast => Err(err.into()),
                FailMode::Continue => {
                    log::error!("{err}");
                    summary.errors += 1;
                    Ok(())
                }
            },
            _ => Ok(()),
        }
    }

    fn extract(&self, harness: Vec<(&'static str, Harness)>) -> Result<()> {
        let harness_config = self.config.harness_config();
        let ctx = harness_config.make_ctx();
        let picus_config = self.config.picus_config();
        let output_base = self.output_base()?;
        let mut summary = Summary::default();
        for (name, harness) in harness.into_iter() {
            self.handle_extract_result(
                || self.extract_one(name, harness, &ctx, &output_base, &picus_config),
                &mut summary,
            )?;
        }
        if summary.errors > 0 {
            bail!("Extraction failed with {} errors", summary.errors);
        }
        if summary.generated == 0 {
            bail!("No circuits were generated!");
        }
        Ok(())
    }
}

#[derive(Default)]
struct Summary {
    errors: usize,
    generated: usize,
}
