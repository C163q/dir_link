use std::{env, fs};

use color_eyre::eyre;
use dir_link::Config;
use directories::BaseDirs;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // TODO: use clap later
    let args: Vec<_> = env::args_os().collect();
    let config = Config {
        path: args.get(1).map(|s| s.into()),
        save: true,
    };

    let base_dir = BaseDirs::new().unwrap();
    let mut local_data = base_dir.data_local_dir().to_path_buf();
    local_data.push("dir_link");
    fs::create_dir_all(&local_data)?;
    local_data.push("data.json");

    let terminal = ratatui::init();
    let result = dir_link::run_app(&local_data, terminal, config);
    ratatui::restore();

    result.map_or_else(
        |e| Err(eyre::eyre!(e)),
        |data| {
            data.inspect(|link| println!("{:?}", link.path().as_os_str()));
            Ok(())
        },
    )
}
