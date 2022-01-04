#[derive(clap::Args, Debug)]
#[clap(about, author, version)]
pub struct Init {}

pub fn call(_args: Init) -> crate::Result<()> {
    if crate::project_root::ProjectRoot::current(()).is_some() {
        return Err(crate::errors::project_already_initialized());
    }

    let project_root = crate::project_root::ProjectRoot::new(())?;

    std::fs::create_dir_all(project_root.log_file_directory())?;
    std::fs::File::create(project_root.log_file_path())?;

    std::fs::File::create(project_root.config_file_path())?;

    Ok(())
}
