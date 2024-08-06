// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::PathBuf;

use crate::{cli, r0vm::R0vm, utils::find_installed_extensions};
use anyhow::Result;
use clap::Subcommand;
use regex::Regex;

#[derive(Debug, Subcommand)]
#[command(
    arg_required_else_help = true,
    subcommand_required = true,
    after_help = cli::help::R0VM_HELP
)]
pub enum R0vmSubcmd {
    /// List all installed extensions
    List,
    /// Install cargo-risczero (i.e cargo-risczero v1.0.1)
    #[command(aliases = ["add"])]
    Install {
        /// The version of the extension to install (e.g., v1.0.1)
        version: Option<String>,
        /// Force installation, removing existing directories
        #[arg(short, long)]
        force: bool,
    },
    /// Use an installed extension version
    Use {
        /// The version of cargo-risczero to use (e.g., v1.0.1)
        version: String,
    },
    /// Uninstall an installed extension
    Uninstall,
}

pub async fn handler(subcmd: R0vmSubcmd) -> Result<()> {
    match subcmd {
        R0vmSubcmd::Install { version, force } => R0vm::install(version.as_deref(), force).await,
        R0vmSubcmd::List => {
            let extensions = find_installed_extensions()?;
            for extension in extensions {
                eprintln!("{}", extension.file_name().unwrap().to_string_lossy());
            }
            Ok(())
        }
        R0vmSubcmd::Use { version } => {
            let extension_path = parse_version(version)?;
            R0vm::link(&extension_path)
        }
        R0vmSubcmd::Uninstall => R0vm::unlink(),
    }
}

fn parse_version(version: String) -> Result<PathBuf> {
    let extensions = find_installed_extensions()?;
    let version_pattern = format!(r"^(v)?{}(?:-.+)?$", regex::escape(&version));

    let re = Regex::new(&version_pattern)?;

    for ext in extensions {
        if let Some(dir_name) = ext.file_name().and_then(|name| name.to_str()) {
            if re.is_match(dir_name) {
                return Ok(ext);
            }
        }
    }
    Err(anyhow::anyhow!(format!(
        "No matching {} found for version {}. \n\nFor more information, try '--help'.",
        R0vm::to_str(),
        version
    )))
}
