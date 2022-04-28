use clap::{Parser, Subcommand};
use std::{path::PathBuf, fs::{read_dir, self, File}, io::Read};
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapData {
    #[serde(rename = "_artist")]
    pub artist: String,
    #[serde(rename = "_difficulties")]
    pub difficulties: Vec<String>,
    #[serde(rename = "_mappers")]
    pub mappers: Vec<String>,
    #[serde(rename = "_music")]
    pub music: String,
    #[serde(rename = "_title")]
    pub title: String,
    #[serde(rename = "_version")]
    pub version: i64,
}


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct AppArgs {
    #[clap(parse(from_os_str))]
    dir: PathBuf,

}

fn sanitize<S>(s:S) -> String where S: Into<String> {
    s.into().chars().map(|c| if c.is_ascii_alphanumeric() || c.is_whitespace() {c} else {'_'} ).collect()
}

fn main() {
    let cmd_args = AppArgs::parse();
    let dir_items = read_dir(&cmd_args.dir).expect("unable to read files in directory");

    for dir_item_unsafe in dir_items {
        if let Ok(dir_item) = dir_item_unsafe {
            // println!("{:?}", dir_item);
            if let Ok(metdata) = dir_item.metadata() {
                if metdata.is_dir() {
                    let path = dir_item.path();

                    let map_data_path = path.join("map.json");
                    if let Ok(mut map_file) = File::open(map_data_path) {
                        let mut file_data = String::new();
                        if map_file.read_to_string(&mut file_data).is_ok() {
                            let map_data:MapData = serde_json::from_str(&file_data).unwrap();
                            let new_music_name = sanitize(&map_data.music);
                            if new_music_name != map_data.music {
                                println!("rename<F> :{}", map_data.music);
                                fs::rename(path.join(map_data.music), path.join(new_music_name)).unwrap();
                            }
                        }

                    }
                    let current_dir_name = dir_item.file_name().to_str().unwrap().to_string();
                    let new_name : String = sanitize(&current_dir_name);
                    if current_dir_name != new_name {
                        println!("rename<D> :{}", current_dir_name);
                        fs::rename(&path, path.parent().unwrap().join(&new_name)).expect("unable to rename file");
                    }
                    // println!("{:?} | {}", dir_item.file_name(),new_name);
                }
            }

        }
    }
}
