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


// this works by getting all potential candidates for the map files, and then checking if they have a bad name by removing all symbols from the name.
// it will not do anything if the name is already good.

fn main() {
    
    // retrive the first command line argument, which is the directory to scan.
    let cmd_args = AppArgs::parse();
    let dir_items = read_dir(&cmd_args.dir).expect("unable to read files in directory"); // get the directory contents


    
    for dir_item_unsafe in dir_items { // iterate over the directory contents
        if let Ok(dir_item) = dir_item_unsafe { // if the directory item is valid
            // println!("{:?}", dir_item);
            if let Ok(metdata) = dir_item.metadata() { // if we're able to get the metadata for the file
                if metdata.is_dir() { // and if the file is a directory
                    let path = dir_item.path(); // get the path to where we have potential map

                    let map_data_path = path.join("meta.json");
                    if let Ok(mut map_file) = File::open(&map_data_path) {      // try to open meta.json              
                        let mut map_data_unsafe:Result<MapData,Error> = serde_json::from_reader(&map_file); // parse the json
                        match map_data_unsafe {
                            Ok(mut map_data) => { // if we're able to parse the json
                                let song_name = map_data.music[..map_data.music.len()-4].to_string();
                                let new_music_name = sanitize(&song_name);
                                // println!("{:?}", new_music_name);


                                // check the difference between them, if there is a difference use the new name
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
                            Err(e) =>{ // else we couldn't parse the json
                                println!("serde read err {:?} | {:?}", e,&map_data_path);
                            }
                        }

                    }
                    // try fixing the current directory name aswell
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
