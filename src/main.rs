use clap::Parser;

/// An application to control backlight brightness
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Slight {
    /// Path to file
    #[clap(short, long)]
    path: String,

    /// Percent
    #[clap(value_parser(-100..=100), allow_hyphen_values = true)]
    percent: i64, // value_parser only accepts i64 / u64
}

impl Slight {
    fn percent_to_value(percent: i64) -> i64 {
        ((u8::MAX as f32 / 100f32) * percent as f32) as i64
    }

    fn write(&self, value: u8) -> Result<(), std::io::Error> {
        std::fs::write(&self.path, value.to_string())
    }

    fn read(&self) -> Result<String, std::io::Error> {
        Ok(String::from_utf8_lossy(&std::fs::read(&self.path)?)
            .as_ref()
            .to_owned())
    }
}

fn main() {
    let slight = Slight::parse();

    let current_value = match slight.read() {
        Ok(v) => v.parse::<i64>().expect("Given file has invalid data"),
        Err(e) => panic!("{}", e),
    };

    let new_value: i64 = current_value + Slight::percent_to_value(slight.percent);
    let new_value: u8 = match new_value.try_into() {
        Ok(v) => v,
        Err(_) => match new_value {
            v if v > u8::MAX as i64 => u8::MAX,
            v if v < u8::MIN as i64 => u8::MIN,
            _ => unreachable!(),
        },
    };

    if let Err(e) = slight.write(new_value) {
        panic!("{}", e)
    }
}
