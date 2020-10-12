// Copyright (c) {{ "now" | date: "%Y" }} {{project-name}} developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `{{crate-name}}` logging

use crate::error::Result;
use clap::ArgMatches;
use slog::{o, Drain, Level, Logger};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};

crate fn initialize(matches: &ArgMatches<'_>) -> Result<Logger> {
    let quiet = matches.occurrences_of("quiet");
    let verbose = matches.occurrences_of("verbose");
    let effective_level = get_effective_level(quiet, verbose);
    let term_decorator = TermDecorator::new().build();
    let drain = FullFormat::new(term_decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();
    let drain = drain.filter_level(effective_level).fuse();
    Ok(Logger::root(drain, o!()))
}

#[cfg(debug_assertions)]
fn get_effective_level(quiet: u64, verbosity: u64) -> Level {
    let mut level = match verbosity {
        0 => Level::Info,
        1 => Level::Debug,
        _ => Level::Trace,
    };
    level = match quiet {
        0 => level,
        1 => Level::Warning,
        _ => Level::Error,
    };
    level
}

#[cfg(not(debug_assertions))]
fn get_effective_level(_quiet: u64, verbosity: u64) -> Level {
    match verbosity {
        0 => Level::Error,
        1 => Level::Warning,
        2 => Level::Info,
        3 => Level::Debug,
        4 | _ => Level::Trace,
    }
}
