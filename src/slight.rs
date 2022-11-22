use strum::IntoEnumIterator;

use std::path::PathBuf;

use crate::{
    brightness::percent_to_value,
    brightness::value_to_percent,
    class::Class,
    device::Device,
    error::{Result, SlightError},
    io::IO,
};

#[derive(Default)]
pub struct Slight {
    devices: Vec<Device>,
}

impl Slight {
    pub fn new() -> Self {
        Slight::default()
    }

    pub fn read_devices(&mut self) {
        self.devices = Vec::new();
        Class::iter().map(|c| PathBuf::from(&c)).for_each(|class| {
            IO::scan(&class).map_or_else(
                |_| todo!("Log out error"),
                |ids| {
                    ids.iter().for_each(|id| {
                        class.join(id).as_path().try_into().map_or_else(
                            |_| todo!("Log out error"),
                            |device| self.devices.push(device),
                        )
                    })
                },
            );
        })
    }

    pub fn print_devices(&self) {
        if self.devices.is_empty() {
            println!("No devices found!");
        } else {
            println!("Found devices:");
            for dev in &self.devices {
                println!("\t{}", dev);
            }
        }
    }

    pub fn set_brightness(&mut self, new: usize, id: Option<String>) -> Result<()> {
        let dev = self.get_device(id)?;
        let path = dev.my_path();
        dev.brightness.set(new, &path)
    }

    pub fn create_range(&self, curr: usize, new: usize, max: usize, exponent: f32) -> Vec<usize> {
        if curr < new {
            return (0..max)
                .map(|v| ((v as f32 / max as f32).powf(exponent) * max as f32) as usize)
                .filter(|&v| v > curr && v <= new)
                .collect();
        } else {
            return (0..max)
                .map(|v| ((v as f32 / max as f32).powf(exponent) * max as f32) as usize)
                .filter(|&v| v < curr && v >= new)
                .rev()
                .collect();
        }
    }

    fn get_device(&mut self, id: Option<String>) -> Result<&mut Device> {
        // TODO: to mut or not to mut
        if let Some(id) = id {
            self.find_device(id).ok_or(
                SlightError::Parse, /*todo!("Error! No specified device found!")*/
            )
        } else {
            self.default_device().ok_or(
                SlightError::Parse, /*todo!("Error! No suitable default device!")*/
            )
        }
    }

    fn default_device(&mut self) -> Option<&mut Device> {
        self.devices
            .iter_mut()
            .filter(|d| d.class == Class::Backlight)
            .nth(0)
    }

    fn find_device(&mut self, id: String) -> Option<&mut Device> {
        self.devices.iter_mut().find(|d| d.id == id)
    }
}
