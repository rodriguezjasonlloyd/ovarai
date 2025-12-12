#[cfg(target_os = "windows")]
use std::io;

use color_eyre::eyre::Result;

#[cfg(target_os = "windows")]
use self_update::{backends::github::Update, cargo_crate_version};

#[cfg(target_os = "windows")]
pub fn check_and_update() -> Result<()> {
    let status = Update::configure()
        .repo_owner("rodriguezjasonlloyd")
        .repo_name("ovarai")
        .bin_name("ovarai")
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    if status.updated() {
        println!(
            "Updated to version {}, press any key to continue...",
            status.version()
        );
        io::stdin().read_line(&mut String::new())?;
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn check_and_update() -> Result<()> {
    Ok(())
}
