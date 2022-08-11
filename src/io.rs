pub type IOError = std::io::Error;

pub fn read(path: &String) -> Result<String, std::io::Error> {
    Ok(String::from_utf8_lossy(&std::fs::read(path)?)
        .as_ref()
        .to_owned())
}

pub fn write(path: &String, value: u8) -> Result<(), std::io::Error> {
    std::fs::write(path, value.to_string())
}
