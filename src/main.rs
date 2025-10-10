use std::fs;

use color_eyre::eyre;
use directories::BaseDirs;
use serde::Serialize;


fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let base_dir = BaseDirs::new().unwrap();
    let mut local_data = base_dir.data_local_dir().to_path_buf();
    local_data.push("dir_link");
    fs::create_dir_all(&local_data)?;
    local_data.push("data.json");

    let terminal = ratatui::init();
    let result = dir_link::run_app(&local_data, terminal);
    ratatui::restore();
    result.map_err(|e| eyre::eyre!(e))
}


fn test() {
    let s = std::ffi::OsString::new();
    let mut vec = serde_json::Serializer::new(Vec::with_capacity(128));
    s.serialize(&mut vec);
}
