use std::io::Write;

use macroquad::file::load_file;
use nanoserde::{DeBin, SerBin};

#[derive(Clone, Copy, Default, Debug, DeBin, SerBin)]
pub struct Persistent {
    pub high_score: u32,
}

impl Persistent {
    pub async fn load() -> Result<Self, macroquad::Error> {
        //load from file
        let file = load_file("save.bin").await?;
        let persist = DeBin::deserialize_bin(&file).unwrap_or_default();

        Ok(persist)
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        //save into le file
        let mut file = std::fs::File::create("save.bin")?;

        file.write_all(&self.serialize_bin())?;

        Ok(())
    }
}
