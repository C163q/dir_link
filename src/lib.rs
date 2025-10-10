use std::{fs::{self, File}, io::{self, BufWriter, Write}, path::Path};

use ratatui::{prelude::Backend, Terminal};

use crate::{data::dirset::LinkDirSet, ui::App};

pub mod data;
pub mod ui;

pub fn run_app<B: Backend>(data_path: &Path, terminal: Terminal<B>) -> io::Result<()> {
    // TODO: warn if data file is corrupted
    let (data, success): (LinkDirSet, bool) = if !data_path.is_file() {
        let mut file = BufWriter::new(File::create(data_path)?);
        let data = LinkDirSet::new();
        file.write_all(serde_json::to_vec(&data)?.as_slice())?;
        (data, true)
    } else {
        let vec = fs::read(data_path)?;
        match serde_json::from_slice(&vec) {
            Ok(data) => (data, true),
            Err(_) => (LinkDirSet::new(), false),
        }
    };

    let result = App::new(data).run(terminal, success);

    match result {
        Ok(data) =>{
            let mut file = BufWriter::new(File::create(data_path)?);
            file.write_all(serde_json::to_vec(&data.1)?.as_slice())?;
            if let Some(link) = data.0.link() {
                println!("{:?}", link.path().as_os_str());
            }
            Ok(())
        }
        // TODO: use color_eyre
        Err(value) => Err(value)
    }
}
