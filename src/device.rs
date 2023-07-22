use dbus::blocking::{BlockingSender, Connection};
use std::ffi::OsStr;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::Path;

use crate::{brightness::Brightness, class::Class, error::{Error, Result}, SLEEP_DURATION_DEFAULT, ToggleState};

const BASE_PATH: &str = "/sys/class";
const CURRENT_BRIGHTNESS: &str = "brightness";
const MAX_BRIGHTNESS: &str = "max_brightness";

#[derive(Debug, Clone)]
pub struct Device(udev::Device);

impl Device {
    pub fn new(path: &Path) -> Result<Self> {
        Ok(Device(udev::Device::from_syspath(path)?))
    }


    pub fn set_brightness(
        &mut self,
        mut range: Box<dyn Iterator<Item = usize>>,
    ) -> Result<()> {
        range.try_for_each(|v| {
            self.set(v)?;
            std::thread::sleep(std::time::Duration::from_secs_f32(SLEEP_DURATION_DEFAULT));
            Ok::<(), Error>(())
        })
    }

    pub fn toggle(&mut self, state: Option<ToggleState>) -> Result<()> {
        if self.is_toggleable() {
            let new = if let Some(ToggleState::On) = state {
                self.brightness().max
            } else if let Some(ToggleState::Off) = state {
                usize::MIN
            } else {
                self.brightness().current ^ 1
            };
            self.set(new)
        } else {
            Err(Error::CannotToggle(self.to_owned()))
        }
    }

    fn is_toggleable(&self) -> bool {
        self.brightness().max == 1
    }

    fn set(&mut self, value: usize) -> Result<()> {
        if crate::check_write_permissions(self.path()).is_ok() {
            Ok(self
                .0
                .set_attribute_value(CURRENT_BRIGHTNESS, value.to_string())?)
        } else {
            let conn = Connection::new_system()?;
            let msg = dbus::Message::new_method_call(
                "org.freedesktop.login1",
                "/org/freedesktop/login1/session/auto",
                "org.freedesktop.login1.Session",
                "SetBrightness",
            )
            .map_err(|e| Error::Dbus(dbus::Error::new_failed(e.as_ref())))?
            .append2(self.class().filename(), self.id().as_ref())
            .append1(value as u32);

            conn.send_with_reply_and_block(msg, std::time::Duration::from_secs(1))?;
            Ok(())
        }
    }

    pub fn brightness(&self) -> Brightness {
        let current = self
            .0
            .attribute_value(CURRENT_BRIGHTNESS)
            .and_then(OsStr::to_str)
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or_else(|| unreachable!());
        let max = self
            .0
            .attribute_value(MAX_BRIGHTNESS)
            .and_then(OsStr::to_str)
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or_else(|| unreachable!());
        Brightness::new(current, max)
    }

    pub fn path(&self) -> &Path {
        self.0.syspath()
    }

    pub fn class(&self) -> Class {
        self.0
            .subsystem()
            .and_then(OsStr::to_str)
            .and_then(Class::from_filename)
            .unwrap_or_else(|| unreachable!())
    }

    pub fn id(&self) -> Id {
        self.0.sysname().to_string_lossy().to_string().into()
    }

    pub fn select(devices: &[Device], id: Option<Id>) -> Result<&Device> {
        if let Some(id) = id {
            Self::find(devices, &id).ok_or(Error::SpecifiedDeviceNotFound)
        } else {
            Self::find_default(devices).ok_or(Error::SuitableDeviceNotFound)
        }
    }

    pub fn find<'a>(devices: &'a [Device], id: &Id) -> Option<&'a Device> {
        devices.iter().find(|&d| d.id().as_ref() == id.as_ref())
    }

    fn find_default(devices: &[Device]) -> Option<&Device> {
        devices.iter().find(|&d| d.class() == Class::Backlight)
    }

    pub fn all() -> Vec<Result<Self>> {
        [Class::Backlight, Class::Led]
            .iter()
            .flat_map(|&class| {
                let path = Path::new(BASE_PATH).join(class.filename());
                path.read_dir()
                    .map(|v| {
                        v.filter_map(std::result::Result::ok)
                            .map(|v| v.file_name().into_string())
                            .filter_map(std::result::Result::ok)
                    })
                    .map(|ids| {
                        ids.map(|id| {
                            let path = path.join(id);
                            Self::new(&path)
                        })
                        .collect::<Vec<_>>()
                    })
            })
            .flatten()
            .collect()
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} '{}': {}", self.class(), self.id(), self.brightness())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Id(String);

impl From<String> for Id {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}
