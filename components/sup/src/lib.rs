// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

//! Habitat helps you build, manage, and run applications - on bare metal, in the cloud, and in
//! containers. You can [read more about it on the website](https://www.habitat.sh/).
//!
//! Habitat contains two main components:
//!
//! * `hab-plan-build`, takes a plan ('plan.sh'), a description of how to build a piece of software,
//! written in [bash](http://tldp.org/HOWTO/Bash-Prog-Intro-HOWTO.html), which produces an atomic
//! package.
//! * `hab-sup`, a run-time executable that knows how to download, install, serve, and
//! manage services defined in packages.
//!
//! # hab-plan-build
//!
//! The documentation for hab-plan-build is generated automatically from the script itself, [and
//! can be found here](hab-plan-build/hab-plan-build.html). You can find it in the source tree at
//! `components/plan-build`.
//!
//! # The Supervisor
//!
//! The Supervisor is primarily utilized through the `hab-sup` command; it can also be used from
//! within Rust as a library. This documentation covers both uses; it explains how things are used
//! from the command line in close proximity to the documentation of the library itself. A few
//! useful starting points:
//!
//! * [The Habitat Command Line Reference](command)
//! * [The Habitat Supervisor Sidecar; http interface to promises](sidecar)

extern crate habitat_butterfly as butterfly;
extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate habitat_depot_client as depot_client;
extern crate handlebars;
#[macro_use]
extern crate log;
extern crate tempdir;
extern crate ansi_term;
extern crate regex;
extern crate libc;
extern crate url;
extern crate iron;
#[macro_use]
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate toml;
extern crate persistent;
#[macro_use]
extern crate prometheus;
#[macro_use]
extern crate lazy_static;

#[macro_export]
/// Creates a new SupError, embedding the current file name, line number, column, and module path.
macro_rules! sup_error {
    ($p: expr) => {
        {
            use $crate::error::SupError;
            SupError::new($p, LOGKEY, file!(), line!(), column!())
        }
    }
}

#[macro_export]
/// Works the same as the print! macro, but uses our StructuredOutput formatter.
macro_rules! output {
    ($content: expr) => {
        {
            use $crate::output::StructuredOutput;
            use $crate::PROGRAM_NAME;
            let so = StructuredOutput::new(PROGRAM_NAME.as_str(),
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           $content);
            print!("{}", so);
        }
    };
    (preamble $preamble: expr, $content: expr) => {
        {
            use $crate::output::StructuredOutput;
            let preamble = &$preamble;
            let so = StructuredOutput::new(preamble,
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           $content);
            print!("{}", so);
        }
    };
    ($content: expr, $($arg:tt)*) => {
        {
            use $crate::output::StructuredOutput;
            use $crate::PROGRAM_NAME;
            let content = format!($content, $($arg)*);
            let so = StructuredOutput::new(PROGRAM_NAME.as_str(),
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           &content);
            print!("{}", so);
        }
    };
    (preamble $preamble: expr, $content: expr, $($arg:tt)*) => {
        {
            use $crate::output::StructuredOutput;
            let content = format!($content, $($arg)*);
            let preamble = &$preamble;
            let so = StructuredOutput::new(preamble,
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           &content);
            print!("{}", so);
        }
    };
}

#[macro_export]
/// Works the same as println!, but uses our structured output formatter.
macro_rules! outputln {
    ($content: expr) => {
        {
            use $crate::output::StructuredOutput;
            use $crate::PROGRAM_NAME;
            let so = StructuredOutput::new(PROGRAM_NAME.as_str(),
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           $content);
            println!("{}", so);
        }
    };
    (preamble $preamble:expr, $content: expr) => {
        {
            use $crate::output::StructuredOutput;
            let so = StructuredOutput::new(&$preamble,
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           $content);
            println!("{}", so);
        }
    };
    ($content: expr, $($arg:tt)*) => {
        {
            use $crate::output::StructuredOutput;
            use $crate::PROGRAM_NAME;
            let content = format!($content, $($arg)*);
            let so = StructuredOutput::new(PROGRAM_NAME.as_str(),
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           &content);
            println!("{}", so);
        }
    };

    (preamble $preamble: expr, $content: expr, $($arg:tt)*) => {
        {
            use $crate::output::StructuredOutput;
            let content = format!($content, $($arg)*);
            let preamble = &$preamble;
            let so = StructuredOutput::new(preamble,
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           &content);
            println!("{}", so);
        }
    }
}

#[macro_export]
/// Works the same as format!, but uses our structured output formatter.
macro_rules! output_format {
    ($content: expr) => {
        {
            use $crate::output::StructuredOutput;
            use $crate::PROGRAM_NAME;
            let so = StructuredOutput::new(PROGRAM_NAME.as_str(),
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           $content);
            format!("{}", so)
        }
    };
    (preamble $preamble:expr, $content: expr) => {
        {
            use $crate::output::StructuredOutput;
            let preamble = &$preamble;
            let so = StructuredOutput::new(preamble,
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           $content);
            format!("{}", so)
        }
    };
    (preamble $preamble:expr, logkey $logkey:expr) => {
        {
            use $crate::output::StructuredOutput;
            let preamble = &$preamble;
            let so = StructuredOutput::new(preamble,
                                           $logkey,
                                           line!(),
                                           file!(),
                                           column!(),
                                           "");
            format!("{}", so)
        }
    };

    ($content: expr, $($arg:tt)*) => {
        {
            use $crate::output::StructuredOutput;
            use $crate::PROGRAM_NAME;
            let content = format!($content, $($arg)*);
            let so = StructuredOutput::new(PROGRAM_NAME.as_str(),
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           &content);
            format!("{}", so)
        }
    };
    (preamble $preamble: expr, $content: expr, $($arg:tt)*) => {
        {
            use $crate::output::StructuredOutput;
            let content = format!($content, $($arg)*);
            let preamble = &$preamble;
            let so = StructuredOutput::new(preamble,
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           &content);
            format!("{}", so)
        }
    }
}

pub mod command;
pub mod config;
pub mod error;
pub mod health_check;
pub mod http_gateway;
pub mod manager;
pub mod output;
pub mod package;
pub mod supervisor;
pub mod templating;
pub mod util;

use std::env;
use std::path::PathBuf;

lazy_static!{
    pub static ref PROGRAM_NAME: String = {
        let arg0 = env::args().next().map(|p| PathBuf::from(p));
        arg0.as_ref().and_then(|p| p.file_stem()).and_then(|p| p.to_str()).unwrap().to_string()
    };
}

const PRODUCT: &'static str = "hab-sup";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
