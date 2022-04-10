use super::*;

/// Config options for the build system.
#[derive(Clone, Debug, Default, Deserialize, Parser)]
pub struct ConfigOptsBuild {
    /// The index HTML file to drive the bundling process [default: index.html]
    #[clap(parse(from_os_str))]
    pub target: Option<PathBuf>,

    /// Build in release mode [default: false]
    #[clap(long)]
    #[serde(default)]
    pub release: bool,

    /// Build a example [default: ""]
    #[clap(long)]
    pub example: Option<String>,

    /// Build platform: support Web & Desktop [default: "default_platform"]
    #[clap(long)]
    pub platform: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Parser)]
pub struct ConfigOptsServe {
    /// The index HTML file to drive the bundling process [default: index.html]
    #[clap(parse(from_os_str))]
    pub target: Option<PathBuf>,

    /// Build a example [default: ""]
    #[clap(long)]
    pub example: Option<String>,

    /// Build in release mode [default: false]
    #[clap(long)]
    #[serde(default)]
    pub release: bool,

    /// Build platform: support Web & Desktop [default: "default_platform"]
    #[clap(long)]
    pub platform: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Parser)]
pub struct ConfigOptsAddModule {
    #[clap(parse(from_os_str))]
    pub target: PathBuf,
}

#[derive(Clone, Debug, Default, Deserialize, Parser)]
pub struct ConfigOptsAddComponent {
    /// Path where the component should be added to
    #[clap(parse(from_os_str))]
    pub target: PathBuf,
    /// The name of the new component
    pub name: String,
    /// Do not generate a property struct
    #[clap(long)]
    pub skip_props: bool,
}

/// Ensure the given value for `--public-url` is formatted correctly.
pub fn parse_public_url(val: &str) -> String {
    let prefix = if !val.starts_with('/') { "/" } else { "" };
    let suffix = if !val.ends_with('/') { "/" } else { "" };
    format!("{}{}{}", prefix, val, suffix)
}
