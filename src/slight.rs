use strum::IntoEnumIterator;

use std::path::PathBuf;

use crate::{
    class::Class,
    device::{Device, Id},
    error::Result,
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
        let dev = if let Some(id) = id {
            self.find_device(id)
                .unwrap_or_else(|| todo!("Error! No specified device found!"))
        } else {
            self.default_device()
        };
        let path = dev.my_path();
        dev.brightness.set(new, &path)
    }

    fn default_device(&mut self) -> &mut Device {
        &mut self.devices[0]
    }

    fn find_device(&mut self, id: String) -> Option<&mut Device> {
        self.devices.iter_mut().find(|d| d.id == id)
    }
}
