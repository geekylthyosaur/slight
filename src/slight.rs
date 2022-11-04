use std::path::PathBuf;

use crate::{class::Class, device::Device, io::IO};

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
        let classes: Vec<PathBuf> = vec![Class::Backlight.into(), Class::Led.into()];
        for class in classes {
            match IO::scan(&class) {
                Ok(device_ids) => {
                    for id in device_ids {
                        match class.join(id).as_path().try_into() {
                            Ok(device) => {
                                self.devices.push(device);
                            }
                            Err(_) => todo!("Error while reading device, skipping"),
                        }
                    }
                }
                Err(_) => todo!("Log out error and continue"),
            }
        }
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
}
