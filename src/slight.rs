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
        // TODO: borrow checker is angry
        let dev = self.get_device(id)?;
        let curr = dev.brightness.as_value();
        let max = dev.brightness.max();
        let path = dev.my_path();
        for v in self.create_range(curr, new, max, 4.0) {
            dev.brightness.set(new, &path)?
        }
        Ok(())
    }

    pub fn create_range(
        &self,
        curr: usize,
        new: usize,
        max: usize,
        exponent: f32,
    ) -> Box<dyn Iterator<Item = usize>> {
        let range =
            (0..max).map(move |v| ((v as f32 / max as f32).powf(exponent) * max as f32) as usize);
        if curr < new {
            Box::new(range.filter(move |&v| v > curr && v <= new))
        } else {
            Box::new(range.filter(move |&v| v < curr && v >= new).rev())
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
