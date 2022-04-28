use clap::{Parser, Subcommand};
use serde_json::Error;
use std::{path::PathBuf, fs::{read_dir, self, File, OpenOptions}, io::Read};
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapData {
    #[serde(rename = "_artist")]
    pub artist: Option<String>,
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

                    let map_data_path = path.join("meta.json");
                    if let Ok(mut map_file) = File::open(&map_data_path) {                    
                        let mut map_data_unsafe:Result<MapData,Error> = serde_json::from_reader(&map_file);
                        match map_data_unsafe {
                            Ok(mut map_data) => {
                                let song_name = map_data.music[..map_data.music.len()-4].to_string();
                                let new_music_name = sanitize(&song_name);
                                // println!("{:?}", new_music_name);
                                if new_music_name != song_name {
                                    let music_with_ext = format!("{}.mp3",new_music_name);
                                    println!("rename<F> : {} @ {:?}", music_with_ext,map_data_path);
                                    if fs::rename(path.join(map_data.music), path.join(&music_with_ext)).is_err(){
                                        println!("unable to rename")
                                    }
                                    map_data.music = music_with_ext;
                                    map_file = OpenOptions::new().write(true).truncate(true).open(&map_data_path).unwrap();
                                    if let Err(e) = serde_json::to_writer(map_file, &map_data) {
                                        println!("unable to write meta.json <{:?}>",e);
                                    }
                                }
                            }
                            Err(e) =>{
                                println!("serde read err {:?} | {:?}", e,&map_data_path);
                            }
                        }

                    }
                    let current_dir_name = dir_item.file_name().to_str().unwrap().to_string();
                    let new_name : String = sanitize(&current_dir_name);
                    if current_dir_name != new_name {
                        println!("rename<D> : {}", current_dir_name);
                        fs::rename(&path, path.parent().unwrap().join(&new_name)).expect("unable to rename file");
                    }
                    // println!("{:?} | {}", dir_item.file_name(),new_name);
                }
            }

        }
    }
}
