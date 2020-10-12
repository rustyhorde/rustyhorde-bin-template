// Copyright (c) {{ "now" | date: "%Y" }} {{project-name}} developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `{{crate_name}}` runtime

mod cli;
mod header;
mod log;

use crate::error::{ErrCode, Error, Result};
use slog::info;
use std::fs::DirBuilder;

crate fn run() -> Result<()> {
    // Parse the command line
    let matches = cli::app().get_matches_safe()?;

    // Setup the config directory
    let mut config_dir = dirs2::config_dir().ok_or_else(no_config_dir)?;
    config_dir.push("{{project-name}}");
    let _ = DirBuilder::new()
        .recursive(true)
        .create(config_dir.clone())?;

    // Output the pretty header
    header::header();

    // Setup logging
    let stdout = log::initialize(&matches)?;
    info!(stdout, "{{project-name}} Started!");

    Err("not implemented".into())
}

fn no_config_dir() -> Error {
    Error::new(ErrCode::Protocol, "There is no valid config dir", None)
}
