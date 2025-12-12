use color_eyre::eyre::Result;

#[cfg(target_os = "windows")]
pub fn check_and_update() -> Result<()> {
    use self_update::{backends::github::Update, cargo_crate_version};

    let status = Update::configure()
        .repo_owner("rodriguezjasonlloyd")
        .repo_name("ovarai")
        .bin_name("ovarai")
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    if status.updated() {
        use std::{io, process};

        println!(
            "Updated to version {}, press any key to continue...",
            status.version()
        );
        io::stdin().read_line(&mut String::new())?;
        process::exit(0);
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn check_and_update() -> Result<()> {
    Ok(())
}
