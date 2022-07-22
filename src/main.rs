use clap::Parser;

/// An application to control backlight brightness
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to file
    #[clap(short, long)]
    path: String,

    /// Percent
    #[clap(value_parser(-100..=100), allow_hyphen_values = true)]
    percent: i64, // value_parser only accepts i64 / u64
}

#[derive(Debug)]
enum SlightError {
    IO(std::io::Error),
    Parse,
}

impl From<std::io::Error> for SlightError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<std::num::ParseIntError> for SlightError {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::Parse
    }
}

impl std::fmt::Display for SlightError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(e) => write!(f, "{}", e),
            Self::Parse => write!(f, "Given file has invalid data"),
        }
    }
}

struct Slight {
    path: String,
    current_value: u8,
    new_value: u8,
}

impl Slight {
    fn percent_to_value(percent: i64) -> i64 {
        ((u8::MAX as f32 / 100f32) * percent as f32) as i64
    }

    fn write_range(&self, r: std::slice::Iter<u8>) -> Result<(), std::io::Error> {
        for &v in r {
            write(&self.path, v)?;
            std::thread::sleep(std::time::Duration::from_millis(16)); // 100/60
        }
        Ok(())
    }
}

impl TryFrom<Args> for Slight {
    type Error = SlightError;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let current_value = read(&args.path)?.trim().parse::<u8>()?;
        let new_value: i64 = current_value as i64 + Slight::percent_to_value(args.percent);
        let new_value: u8 = match new_value.try_into() {
            Ok(v) => v,
            Err(_) => match new_value {
                v if v > u8::MAX as i64 => u8::MAX,
                v if v < u8::MIN as i64 => u8::MIN,
                _ => unreachable!(),
            },
        };

        Ok(Self {
            current_value,
            new_value,
            path: args.path,
        })
    }
}

fn read(path: &String) -> Result<String, std::io::Error> {
    Ok(String::from_utf8_lossy(&std::fs::read(path)?)
        .as_ref()
        .to_owned())
}

fn write(path: &String, value: u8) -> Result<(), std::io::Error> {
    std::fs::write(path, value.to_string())
}

fn main() {
    let args = Args::parse();

    let slight: Slight = match args.try_into() {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    let v = if slight.current_value < slight.new_value {
        (slight.current_value..=slight.new_value).collect::<Vec<u8>>()
    } else {
        ((slight.new_value..=slight.current_value).rev()).collect::<Vec<u8>>()
    };

    if let Err(e) = slight.write_range(v.iter()) {
        panic!("{}", e);
    }
}
