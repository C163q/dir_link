use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};

use ratatui::{Terminal, prelude::Backend};

use crate::{
    data::{dirset::LinkDirSet, link::Link},
    ui::App,
};

pub mod data;
pub mod debug;
pub mod ui;

#[derive(Debug)]
pub struct DataTransfer {
    pub link: Option<Link>,
    pub config: Option<Config>,
}

impl Default for DataTransfer {
    fn default() -> Self {
        Self::new()
    }
}

impl DataTransfer {
    pub fn new() -> Self {
        Self {
            link: None,
            config: None,
        }
    }

    pub fn with_link(link: Link) -> Self {
        Self {
            link: Some(link),
            config: None,
        }
    }

    pub fn with_config(path: PathBuf) -> Self {
        Self {
            link: None,
            config: Some(Config { path: Some(path) }),
        }
    }

    pub fn link(&self) -> Option<&Link> {
        self.link.as_ref()
    }

    pub fn config(&self) -> Option<&Config> {
        self.config.as_ref()
    }
}

#[derive(Debug)]
pub struct Config {
    pub path: Option<PathBuf>,
}

// 临时函数
pub fn output_result(bytes: &[u8], path: Option<&PathBuf>) -> io::Result<()> {
    match path {
        Some(path) => fs::write(path, bytes),
        None => io::stdout().write_all(bytes),
    }
}

pub fn run_app<B: Backend>(
    data_path: &Path,
    terminal: Terminal<B>,
    mut config: Config,
) -> io::Result<()> {
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

    let path = config.path.take();
    let data_transfer = DataTransfer {
        config: Some(config),
        link: None,
    };

    let result = App::new(data).run(terminal, success, data_transfer);

    match result {
        Ok(data) => {
            let mut file = BufWriter::new(File::create(data_path)?);
            file.write_all(serde_json::to_vec(&data.1)?.as_slice())?;
            if let Some(link) = data.0.link() {
                output_result(
                    link.path().as_os_str().as_encoded_bytes(),
                    path.as_ref(),
                )?;
            }
            Ok(())
        }
        // TODO: use color_eyre
        Err(value) => Err(value),
    }
}
