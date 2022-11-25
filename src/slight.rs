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

const EXPONENT_DEFAULT: f32 = 4.0;
const SLEEP_DURATION: f32 = 1.0 / 60.0;

#[derive(Default)]
pub struct Slight {
    devices: Vec<Device>,
    exponent: Option<f32>,
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
        let exponent = self.exponent.unwrap_or(EXPONENT_DEFAULT);
        let dev = self.get_device(id)?;
        let curr = dev.brightness.as_value();
        let max = dev.brightness.max();
        let range = Self::create_range(curr, new, max, exponent);
        Self::set_brightness_range(range, dev)?;
        Ok(())
    }

    fn create_range(
        curr: usize,
        new: usize,
        max: usize,
        exponent: f32,
    ) -> Box<dyn Iterator<Item = usize>> {
        // TODO: dedup
        let range =
            (0..max).map(move |v| ((v as f32 / max as f32).powf(exponent) * max as f32) as usize);
        if curr < new {
            Box::new(range.filter(move |&v| v > curr && v <= new))
        } else {
            Box::new(range.filter(move |&v| v < curr && v >= new).rev())
        }
    }

    fn set_brightness_range(
        range: Box<dyn Iterator<Item = usize>>,
        device: &mut Device,
    ) -> Result<()> {
        let path = device.my_path();
        for v in range {
            device.brightness.set(v, &path)?;
            std::thread::sleep(std::time::Duration::from_secs_f32(SLEEP_DURATION));
        }
        Ok(())
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
            .find(|d| d.class == Class::Backlight)
    }

    fn find_device(&mut self, id: String) -> Option<&mut Device> {
        self.devices.iter_mut().find(|d| d.id == id)
    }
}
