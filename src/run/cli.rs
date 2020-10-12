// Copyright (c) {{ "now" | date: "%Y" }} {{project-name}} developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `{{crate-name}}` cli

use clap::{App, Arg};

crate fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("{{crate-name}}")
        .version(env!("CARGO_PKG_VERSION"))
        .author("{{authors}}")
        .about("{{project-name}}")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity (multiple turn up the noise)"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .multiple(true)
                .help("Sets the level of quiet (multiple turn down the noise)"),
        )
}
