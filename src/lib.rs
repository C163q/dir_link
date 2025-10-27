use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};

use ratatui::{Terminal, prelude::Backend};

use crate::{
    app::{
        App,
        data::{Config, DataTransfer, RuntimeError},
    },
    data::{dirset::LinkDirSet, link::Link},
};

pub mod app;
pub mod data;
pub mod debug;
pub mod ui;

// 临时函数
pub fn output_result(bytes: &[u8], path: &PathBuf) -> io::Result<()> {
    fs::write(path, bytes)
}

pub fn get_data(data_path: &Path) -> io::Result<LinkDirSet> {
    if !data_path.is_file() {
        let mut file = BufWriter::new(File::create(data_path)?);
        let data = LinkDirSet::new();
        file.write_all(serde_json::to_vec(&data)?.as_slice())?;
        Ok(data)
    } else {
        let vec = fs::read(data_path)?;
        let data = serde_json::from_slice(&vec);
        data.map_err(|err| err.into())
    }
}

pub fn try_save(save: bool, data_path: &Path, data: &LinkDirSet) -> io::Result<()> {
    if save {
        let mut file = BufWriter::new(File::create(data_path)?);
        file.write_all(serde_json::to_vec(data)?.as_slice())?;
    }
    Ok(())
}

pub fn run_app<B: Backend>(
    data_path: &Path,
    mut terminal: Terminal<B>,
    mut config: Config,
) -> io::Result<Option<Link>> {
    let (data, read_result) = match get_data(data_path) {
        Ok(data) => (data, Ok(())),
        Err(err) => (LinkDirSet::new(), Err(err)),
    };

    let path = config.path.take();
    let data_transfer = DataTransfer {
        config: Some(config),
        link: None,
        data: None,
    };

    let mut runtime = RuntimeError::new();
    runtime.read_data = read_result.err();

    let mut transfer = App::new(data).run(&mut terminal, runtime, data_transfer)?;
    let link_data_set = transfer.data.take().unwrap();

    // TODO: handle save error
    let _result = try_save(
        transfer.config.as_ref().unwrap().save,
        data_path,
        &link_data_set,
    );

    if let Some(link) = transfer.link() {
        path.map_or_else(
            || Ok(Some(link.clone())),
            |path| {
                output_result(link.path().as_os_str().as_encoded_bytes(), &path)?;
                Ok(None)
            },
        )
    } else {
        Ok(None)
    }
}
