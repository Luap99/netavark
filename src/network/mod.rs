pub mod types;
use anyhow::Result;
use std::fs::File;

impl types::NetworkOptions {
    pub fn load(path: &str) -> Result<types::NetworkOptions> {
        let file = std::io::BufReader::new(File::open(path)?);
        Ok(serde_json::from_reader(file)?)
    }
    pub fn save(&self, path: &str) -> Result<()> {
        let mut file = std::io::BufWriter::new(File::create(path)?);
        Ok(serde_json::to_writer_pretty(&mut file, self)?)
    }
}
