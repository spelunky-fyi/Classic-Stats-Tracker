use static_files::resource_dir;
use std::io;
#[cfg(windows)]
use winres::WindowsResource;

fn main() -> io::Result<()> {
    #[cfg(windows)]
    {
        WindowsResource::new()
            .set_icon("assets/icons/logo.ico")
            .compile()?;
    }

    resource_dir("./frontend/public").build()?;
    Ok(())
}
