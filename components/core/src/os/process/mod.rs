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

use std::fmt;
use std::process::Child;

use error::Result;

#[allow(unused_variables)]
#[cfg(windows)]
#[path = "windows.rs"]
mod imp;

#[cfg(not(windows))]
#[path = "linux.rs"]
mod imp;

pub use self::imp::become_command;

pub enum ShutdownMethod {
    AlreadyExited,
    GracefulTermination,
    Killed,
}

impl fmt::Display for ShutdownMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            ShutdownMethod::AlreadyExited => "Already Exited",
            ShutdownMethod::GracefulTermination => "Graceful Termination",
            ShutdownMethod::Killed => "Killed",
        };
        write!(f, "{}", printable)
    }
}

pub struct HabChild {
    inner: imp::Child,
}

impl HabChild {
    pub fn from(inner: &mut Child) -> Result<HabChild> {
        match imp::Child::new(inner) {
            Ok(child) => Ok(HabChild { inner: child }),
            Err(e) => Err(e),
        }
    }

    pub fn id(&self) -> u32 {
        self.inner.id()
    }

    pub fn status(&mut self) -> Result<HabExitStatus> {
        self.inner.status()
    }

    pub fn kill(&mut self) -> Result<ShutdownMethod> {
        self.inner.kill()
    }
}

impl fmt::Debug for HabChild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pid: {}", self.id())
    }
}

pub struct HabExitStatus {
    status: Option<u32>,
}

impl HabExitStatus {
    pub fn no_status(&self) -> bool {
        self.status.is_none()
    }
}

pub trait ExitStatusExt {
    fn code(&self) -> Option<u32>;
    fn signal(&self) -> Option<u32>;
}
