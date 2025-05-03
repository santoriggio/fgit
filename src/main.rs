use dirs::home_dir;
use inquire::Select;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{self, File, metadata};
use std::io::Read;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

const IGNORE_PATHS: [&str; 3] = ["node_modules", "ios", "android"];
const CACHE_DIR: &str = ".fgit";
const CACHE_FILE: &str = "cache.json";

#[derive(Serialize, Deserialize)]
struct Cache {
    modified: u64,
    paths: Vec<PathBuf>,
}

fn has_git_dir(path: &PathBuf) -> bool {
    path.join(".git").is_dir()
}

fn should_skip_dir(dir_name: &str) -> bool {
    IGNORE_PATHS.iter().any(|ignore| dir_name == *ignore)
}

fn scan_dirs(dir_path: &PathBuf) -> Vec<PathBuf> {
    if should_skip_dir(dir_path.file_name().and_then(|n| n.to_str()).unwrap_or("")) {
        return vec![];
    }

    let mut result = vec![];

    if has_git_dir(dir_path) {
        result.push(dir_path.to_path_buf());
    }

    if let Ok(entries) = fs::read_dir(dir_path) {
        let subdirs: Vec<PathBuf> = entries
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.is_dir())
            .collect();

        let sub_results: Vec<Vec<PathBuf>> =
            subdirs.par_iter().map(|subdir| scan_dirs(subdir)).collect();

        for mut sublist in sub_results {
            result.append(&mut sublist);
        }
    }

    result
}

fn load_cache(cache_path: &PathBuf) -> Option<Cache> {
    let mut file = File::open(cache_path).ok()?;
    let mut data = String::new();
    file.read_to_string(&mut data).ok()?;
    serde_json::from_str(&data).ok()
}

fn save_cache(cache_path: &PathBuf, modified: u64, paths: &[PathBuf]) {
    if let Some(parent) = cache_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let cache = Cache {
        modified,
        paths: paths.to_vec(),
    };
    if let Ok(data) = serde_json::to_string(&cache) {
        let _ = fs::write(cache_path, data);
    }
}

///Return timestamp of a path
fn get_modified_timestamp(path: &PathBuf) -> u64 {
    metadata(path)
        .and_then(|m| m.modified())
        .unwrap_or(UNIX_EPOCH)
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn main() {
    //Get home directory
    let mut path = home_dir().expect("Error while getting home dir");

    //Push Desktop on path
    path.push("Desktop");

    //Create a different path for the cache
    let mut cache_path = home_dir().unwrap();
    cache_path.push(CACHE_DIR);
    cache_path.push(CACHE_FILE);

    //Return the "path" modified timestamp
    let current_modified = get_modified_timestamp(&path);

    let mut dirs = if let Some(cache) = load_cache(&cache_path) {
        if cache.modified == current_modified {
            cache.paths
        } else {
            let scanned = scan_dirs(&path);
            save_cache(&cache_path, current_modified, &scanned);
            scanned
        }
    } else {
        let scanned = scan_dirs(&path);
        save_cache(&cache_path, current_modified, &scanned);
        scanned
    };

    dirs.par_sort_by(|a, b| {
        let a_time = metadata(a).and_then(|m| m.modified()).unwrap_or(UNIX_EPOCH);
        let b_time = metadata(b).and_then(|m| m.modified()).unwrap_or(UNIX_EPOCH);

        b_time.cmp(&a_time)
    });

    let display_dirs: Vec<String> = dirs
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    match Select::new("Select a git directory:", display_dirs.clone())
        .with_page_size(10)
        .prompt()
    {
        Ok(choice) => println!("{}", choice), // Stampa solo il path selezionato
        Err(_) => std::process::exit(1),      // Esce in caso di errore
    }
}
