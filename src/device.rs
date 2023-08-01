use dbus::blocking::{BlockingSender, Connection};
use std::cell::OnceCell;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

use crate::{
    brightness::Brightness,
    class::Class,
    error::{Error, Result},
    ToggleState, SLEEP_DURATION_DEFAULT,
};

const CURRENT_BRIGHTNESS: &str = "brightness";
const MAX_BRIGHTNESS: &str = "max_brightness";

#[derive(Debug, Clone)]
pub struct Device(udev::Device);

impl Device {
    pub fn new(path: &Path) -> Result<Self> {
        Ok(Device(
            udev::Device::from_syspath(path).map_err(|e| Error::DeviceBroken(path.into(), e))?,
        ))
    }

    pub fn set_range(&mut self, mut range: Box<dyn Iterator<Item = usize>>) -> Result<()> {
        let sysfs_permissions = self.is_syspath_writable().is_ok();
        let conn = OnceCell::new();
        range.try_for_each(|v| {
            if sysfs_permissions {
                self.set_sysfs(v)?;
            } else {
                // FIXME: #![feature(once_cell_try)]
                self.set_dbus(
                    v,
                    conn.get_or_init(Connection::new_system)
                        .as_ref()
                        .map_err(|e| dbus::Error::new_failed(e.message().unwrap()))?,
                )?;
            }

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
            self.set_range(Box::new(std::iter::once(new)))
        } else {
            Err(Error::CannotToggle {
                id: self.id(),
                brightness: self.brightness(),
            })
        }
    }

    fn is_toggleable(&self) -> bool {
        self.brightness().max == 1
    }

    fn set_sysfs(&mut self, value: usize) -> Result<()> {
        Ok(self
            .0
            .set_attribute_value(CURRENT_BRIGHTNESS, value.to_string())?)
    }

    fn set_dbus(&mut self, value: usize, conn: &Connection) -> Result<()> {
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

    pub fn brightness(&self) -> Brightness {
        let [current, max] = [CURRENT_BRIGHTNESS, MAX_BRIGHTNESS].map(|s| {
            self.0
                .attribute_value(s)
                .and_then(OsStr::to_str)
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or_else(|| unreachable!())
        });
        Brightness::new(current, max)
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

    pub fn select<'a>(devices: &'a [Device], id: &Option<Id>) -> Result<&'a Device> {
        if let Some(id) = id {
            Self::find(devices, id).ok_or(Error::SpecifiedDeviceNotFound)
        } else {
            Self::find_default(devices).ok_or(Error::SuitableDeviceNotFound)
        }
    }

    fn find<'a>(devices: &'a [Device], id: &Id) -> Option<&'a Device> {
        devices.iter().find(|&d| d.id().as_ref() == id.as_ref())
    }

    fn find_default(devices: &[Device]) -> Option<&Device> {
        devices.iter().find(|&d| d.class() == Class::Backlight)
    }

    pub fn all() -> Result<Vec<Self>> {
        [Class::Backlight, Class::Led]
            .iter()
            .flat_map(|&class| class.scan())
            .flatten()
            .collect()
    }

    fn is_syspath_writable(&self) -> Result<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.0.syspath().join(CURRENT_BRIGHTNESS))?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        file.write_all(content.as_bytes())?;
        Ok(())
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
