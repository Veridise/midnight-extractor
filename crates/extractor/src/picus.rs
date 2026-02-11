use std::{
    fs::{self, File},
    io::Write as _,
    path::Path,
};

use haloumi::{driver::Driver, ir_gen::circuit::resolved::ResolvedIRCircuit};
use haloumi_picus::PicusParamsBuilder;
use header::Header;
use prelude::Preludes;

pub mod header;
pub mod prelude;

pub struct PicusConfig {
    opt: bool,
    prelude: Option<Preludes>,
}

impl PicusConfig {
    pub fn new(opt: bool, prelude: Option<Preludes>) -> Self {
        Self { opt, prelude }
    }
}

pub fn write_picus_output(
    config: &PicusConfig,
    name: &'static str,
    output_base: impl AsRef<Path>,
    ir: &ResolvedIRCircuit,
    mut params: PicusParamsBuilder,
) -> anyhow::Result<()> {
    let output_dir = output_base.as_ref().join(name);
    fs::create_dir_all(&output_dir)?;
    params.short_names().no_expr_cutoff().entrypoint(name);
    if !config.opt {
        params.no_optimize();
    }
    let output = Driver::default().picus(ir, params.build())?;

    let output_path = output_dir.join("output.picus");
    let mut output_file = File::create(&output_path)?;
    writeln!(output_file, "{}", Header)?;
    if let Some(prelude) = config.prelude {
        writeln!(output_file, "{prelude}")?;
    }
    writeln!(output_file, "{}", output.display())?;
    log::info!("Saved picus output in {}", output_path.display());
    Ok(())
}
