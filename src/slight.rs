use crate::{class::Class, device::Device, io::IO};

#[derive(Default)]
pub struct Slight {
    devices: Option<Vec<Device>>,
}

impl Slight {
    pub fn new() -> Self {
        Slight::default()
    }

    pub fn read_devices(&mut self) {
        let classes = vec![Class::Backlight.path(), Class::Led.path()];
        for class in classes {
            match IO::scan(&class) {
                Ok(device_ids) => {
                    for id in device_ids {
                        let device = match Device::try_new(&class.join(id)) {
                            Ok(v) => v,
                            Err(_) => todo!("Error while reading device, skipping"),
                        };
                        self.devices.as_mut().unwrap().push(device);
                    }
                }
                Err(_) => todo!("Log out error and continue"),
            }
        }
    }
}
