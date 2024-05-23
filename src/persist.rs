//! Persistent storage.

use std::io::Write;

use macroquad::file::load_file;
use nanoserde::{DeBin, SerBin};

/// Persistent data that the application can save and load.
#[derive(Clone, Copy, Default, Debug, DeBin, SerBin)]
pub struct Persistent {
    /// Highest reached score across all runs.
    pub high_score: u32,
}

impl Persistent {
    /// Load the persistent data from file.
    pub async fn load() -> Result<Self, macroquad::Error> {
        //load from file
        let file = load_file("save.bin").await?;
        let persist = DeBin::deserialize_bin(&file).unwrap_or_default();

        Ok(persist)
    }

    /// Save the persistent data into a file.
    pub fn save(&self) -> Result<(), std::io::Error> {
        //save into le file
        let mut file = std::fs::File::create("save.bin")?;

        file.write_all(&self.serialize_bin())?;

        Ok(())
    }
}
