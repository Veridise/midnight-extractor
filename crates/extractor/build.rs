use cargo_lock::{Lockfile, SourceId};
use std::collections::HashSet;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let lock = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join("Cargo.lock"); //.with_extension("lock");
    let packages: HashSet<&str> = ["halo2-llzk-frontend", "midnight-circuits"].into();

    eprintln!("Cargo lock path: {}", lock.display());
    Lockfile::load(lock)?
        .packages
        .iter()
        .filter(|p| packages.contains(p.name.as_str()))
        .for_each(|p| {
            let name = p.name.as_str().to_uppercase().replace("-", "_");
            println!("cargo::rustc-env={name}_VERSION={}", p.version);
            if let Some(checksum) = p.source.as_ref().and_then(SourceId::precise) {
                println!("cargo::rustc-env={name}_CHECKSUM={}", &checksum[0..8]);
            }
        });
    Ok(())
}
