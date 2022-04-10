use clap::crate_version;

use crate::crate_root;

use super::*;

#[derive(Clone, Debug, Parser)]
#[clap(name = "version")]
pub struct Version {}

const ART: &str = r"
 ___   _                      ||---||   ___  _     ___ 
|   \ (_) ___ __ __ _  _  ___  \\-//   / __|| |   |_ _|
| |) || |/ _ \\ \ /| || |(_-/  //-\\  | (__ | |__  | | 
|___/ |_|\___//_\_\ \_._|/__/ ||---||  \___||____||___|
";

impl Version {
    pub fn version(self) -> Result<()> {
        println!("{}", ART);
        log::info!("🚀 Dioxus CLI Version: {}", crate_version!());
        if let Some(vers) = get_installed_dioxus_version()? {
            log::info!("🚀 Dioxus Version: {}", vers);
        } else {
            log::warn!("🛑 No Version of Dioxus found in Cargo.toml!");
        }
        if let Some(rustc) = get_installed_rustc_version()? {
            log::info!("🦀 Rust Version: {}", rustc);
        } else {
            log::warn!("🛑 Did not find an installation of Rust!")
        }
        Ok(())
    }
}

fn get_installed_dioxus_version() -> Result<Option<String>> {
    let p = crate_root()?.join("Cargo.toml");
    let manifest = cargo_toml::Manifest::from_path(p).map_err(|e| format!("{}", e))?;
    if let Some(x) = manifest.dependencies.get("dioxus") {
        match x {
            cargo_toml::Dependency::Simple(s) => Ok(Some(s.to_owned())),
            cargo_toml::Dependency::Detailed(d) => {
                let version = d
                    .version
                    .clone()
                    .unwrap_or_else(|| "No version found".to_owned());
                if d.features.is_empty() {
                    Ok(Some(version))
                } else {
                    Ok(Some(format!(
                        "{}, activated features: {}",
                        version,
                        d.features.join(", ")
                    )))
                }
            }
        }
    } else {
        Ok(None)
    }
}

fn get_installed_rustc_version() -> Result<Option<String>> {
    let output = Command::new("rustc").arg("--version").output()?;
    if !output.status.success() {
        Ok(None)
    } else {
        let stdout = String::from_utf8(output.stdout).map_err(|e| format!("{}", e))?;
        Ok(Some(stdout))
    }
}
