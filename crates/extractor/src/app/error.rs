use std::fmt;

#[derive(Debug)]
enum AppErrorKind {
    HarnessFailed,
    OptFailed,
    IRDumpFailed,
    PicusWriteFailed,
}

#[derive(Debug)]
pub struct AppError {
    kind: AppErrorKind,
    name: &'static str,
    err: anyhow::Error,
}

impl AppError {
    fn create(name: &'static str, kind: AppErrorKind) -> impl FnOnce(anyhow::Error) -> Self {
        move |err| Self { kind, name, err }
    }

    pub fn harness(name: &'static str) -> impl FnOnce(anyhow::Error) -> Self {
        Self::create(name, AppErrorKind::HarnessFailed)
    }

    pub fn opt(name: &'static str) -> impl FnOnce(anyhow::Error) -> Self {
        Self::create(name, AppErrorKind::OptFailed)
    }
    pub fn ir_dump(name: &'static str) -> impl FnOnce(anyhow::Error) -> Self {
        Self::create(name, AppErrorKind::IRDumpFailed)
    }

    pub fn picus(name: &'static str) -> impl FnOnce(anyhow::Error) -> Self {
        Self::create(name, AppErrorKind::PicusWriteFailed)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            AppErrorKind::HarnessFailed => {
                write!(f, "Harness {} failed: {:?}", self.name, self.err)
            }
            AppErrorKind::OptFailed => write!(
                f,
                "IR optimization pass failed for harness {}: {:?}",
                self.name, self.err
            ),
            AppErrorKind::IRDumpFailed => write!(
                f,
                "Failed to write IR dump of harness {}: {:?}",
                self.name, self.err
            ),
            AppErrorKind::PicusWriteFailed => write!(
                f,
                "Failed to write Picus result of harness {}: {:?}",
                self.name, self.err
            ),
        }
    }
}

impl std::error::Error for AppError {}
unsafe impl Sync for AppError {}
unsafe impl Send for AppError {}
