use crate::{class::Class, device::Device, io::IO};

#[derive(Default)]
pub struct Slight {
    devices: Option<Vec<Device>>,
}

impl Slight {
    pub fn new() -> Self {
        Slight::default()
    }

    fn read_devices(&mut self) {
        let classes = vec![Class::Backlight.path(), Class::Led.path()];
        for class in classes {
            if let Some(device_ids) = IO::scan(&class) {
                for id in device_ids {
                    let device = match Device::try_new(&class.join(id)) {
                        Ok(v) => v,
                        Err(e) => todo!("Error while reading device, skipping"),
                    };
                    self.devices.as_mut().unwrap().push(device);
                }
            }
        }
    }
}
