#[derive(clap::Args, Debug)]
#[clap(about, author, version)]
pub struct Reindex {}

impl Reindex {
    pub fn call(_args: Reindex) -> crate::Result<()> {
        let context = crate::ctx::Context::new(())?;
        crate::services::reindex(&context)
    }
}
