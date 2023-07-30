#![warn(clippy::pedantic)]
mod brightness;
mod class;
mod device;
mod error;
mod range;

use crate::{device::Device, range::Range};
pub use crate::{
    device::Id,
    error::{Error, Result},
    range::Input,
};
use nix::unistd;
use std::path::Path;

/// Default value for exponent when using `--exponent` flag without given value
const EXPONENT_DEFAULT: f32 = 4.0;
/// Default value for exponent when `--exponent` flag was not provided
const NO_EXPONENT_DEFAULT: f32 = 1.0;
// TODO: std::time::Duration::from_secs_f32 is not stable as const fn yet
/// Default time interval between brightness changes
pub const SLEEP_DURATION_DEFAULT: f32 = 1.0 / 30.0;

#[derive(Clone)]
pub enum Mode {
    Regular { input: Input },
    Exponential { input: Input, exponent: Option<f32> },
    List(Vec<Id>),
    Toggle(Option<ToggleState>),
}

#[derive(Clone, Copy)]
pub enum ToggleState {
    On,
    Off,
}

#[derive(Default)]
struct Flags {
    /// Write to `stdout` instead of `sysfs`
    pub stdout: bool,
    /// Being verbose about what is going on
    pub verbose: bool,
}

pub struct Slight {
    id: Option<Id>,
    flags: Flags,
}

impl Slight {
    pub fn new(id: impl Into<Option<Id>>) -> Slight {
        Slight {
            id: id.into(),
            flags: Flags::default(),
        }
    }

    pub fn verbose(&mut self, v: bool) {
        self.flags.verbose = v;
    }

    pub fn stdout(&mut self, v: bool) {
        self.flags.stdout = v;
    }

    pub fn run(&self, mode: Mode) -> Result<()> {
        let devices = Device::all()
            .into_iter()
            .map(Result::unwrap) // TODO: error handling
            .collect::<Vec<_>>();
        let mut device = Device::select(&devices, &self.id)?.clone();
        let curr = device.brightness().current;
        let max = device.brightness().max;

        match mode {
            Mode::List(ids) => Self::print_devices(&devices, &ids),
            Mode::Toggle(toggle_state) => device.toggle(toggle_state),
            Mode::Regular { input } => {
                let r = Range::new(curr, max, NO_EXPONENT_DEFAULT);
                let r = input.iter(r);
                device.set_range(r)
            }
            Mode::Exponential { input, exponent } => {
                let exponent = exponent.unwrap_or(EXPONENT_DEFAULT);
                let r = Range::new(curr, max, exponent);
                let r = input.iter(r);
                device.set_range(r)
            }
        }
    }

    fn print_devices(devices: &[Device], ids: &[Id]) -> Result<()> {
        if devices.is_empty() {
            Err(Error::NoDevices)?;
        }

        ids.is_empty()
            .then(|| devices.iter().for_each(|d| println!("{d}")))
            .unwrap_or_else(|| {
                devices
                    .iter()
                    .filter(|d| ids.contains(&d.id()))
                    .for_each(|d| println!("{d}"));
            });

        Ok(())
    }
}

// FIXME: false positive
pub(crate) fn check_write_permissions(path: &Path) -> Result<()> {
    let user_groups = unistd::getgroups().map_err(|_| Error::Permission)?;
    let video_group = unistd::Group::from_name("video")
        .ok()
        .flatten()
        .ok_or(Error::Permission)?
        .gid;
    let is_path_writable = path
        .metadata()
        .map(|m| !m.permissions().readonly())
        .map_err(|_| Error::Permission)?;

    (user_groups.contains(&video_group) && is_path_writable)
        .then_some(())
        .ok_or(Error::Permission)
}
