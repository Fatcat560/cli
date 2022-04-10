use convert_case::{Case, Casing};

use crate::cfg::{ConfigOptsAddComponent, ConfigOptsAddModule};
use std::fs::{self, OpenOptions};
use std::io::{Seek, SeekFrom, Write};

use super::*;

const COMPONENT_TEMPLATE: &str = r#"use dioxus::prelude::*;

{{PROPS}}pub fn {{NAME}}(ctx: {{SCOPE}}) -> Element {
    ctx.render(rsx!(
        div {
            p {
                "{{NAME}} works!"
            }
        }
    ))
}
"#;

const PROPS_TEMPLATE: &str = r"#[derive(PartialEq, Props)]
pub struct {{NAME}}Props {}

";

const MOD_TEMPLATE: &str = r"mod {{NAME}};
pub use {{NAME}}::*;
";
/// Add a component or module
#[derive(Clone, Debug, Parser)]
#[clap(name = "add")]
pub enum Add {
    /// Create a new module
    Module(ConfigOptsAddModule),
    /// Create a new Dioxus component
    Component(ConfigOptsAddComponent),
}

impl Add {
    pub fn add(self) -> Result<()> {
        match self {
            Add::Module(cfg) => produce_module(cfg),
            Add::Component(cfg) => produce_component(cfg),
        }
    }
}

fn produce_component(cfg: ConfigOptsAddComponent) -> Result<()> {
    //target must be:
    //  - a path to a directory which contains a mod.rs (valid module)
    if !cfg.target.is_dir() {
        log::error!("🛑 Target path is not a directory");
        return Ok(());
    }
    let dir = cfg.target.read_dir()?;
    if !dir
        .filter_map(|x| x.ok())
        .any(|x| x.file_name() == "mod.rs")
    {
        log::error!("🛑 Target path is not a module!");
        return Ok(());
    }
    let pascal_name = cfg.name.to_case(Case::Pascal);
    let snake_name = cfg.name.to_case(Case::Snake);
    // To create the component:
    //  - create a directory with name {name}
    //  - add a mod.rs inside with the component code
    //  - modify parent mod.rs to include and use {name}
    write_component_to_disk(&pascal_name, cfg.skip_props, cfg.target.join(&snake_name))?;
    modify_parent_module(cfg.target.join("mod.rs"), &snake_name)?;
    Ok(())
}

fn write_component_to_disk(name: &str, skip_props: bool, new_dir: PathBuf) -> Result<()> {
    let mut component = COMPONENT_TEMPLATE.replace("{{NAME}}", name);
    let (props, scope) = if skip_props {
        (String::new(), String::from("Scope"))
    } else {
        (
            PROPS_TEMPLATE.replace("{{NAME}}", name),
            format!("Scope<{}Props>", &name),
        )
    };
    component = component
        .replace("{{PROPS}}", &props)
        .replace("{{SCOPE}}", &scope);
    fs::create_dir_all(&new_dir)?;
    let mut file = File::create(new_dir.join("mod.rs"))?;
    write!(file, "{}", component)?;
    Ok(())
}

fn modify_parent_module(mod_file: PathBuf, name: &str) -> Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open(mod_file)?;
    //TODO: Try not to read everything as a string
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(format!("{}\n{}", MOD_TEMPLATE.replace("{{NAME}}", name), content).as_bytes())?;
    Ok(())
}

fn produce_module(cfg: ConfigOptsAddModule) -> Result<()> {
    dbg!(cfg);
    Ok(())
}
