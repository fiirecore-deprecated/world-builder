use std::path::PathBuf;

use firecore_world::map::warp::WarpEntry;

pub fn load_warp_entries(warp_path: PathBuf) -> crate::ResultT<Vec<WarpEntry>> {
    let mut warps = Vec::new();
    if let Ok(dir) = std::fs::read_dir(warp_path) {
        for entry in dir {
            if let Ok(entry) = entry {
                let file = entry.path();
                let data =  std::fs::read_to_string(&file)?;
                match ron::from_str(&data) {
                    Ok(warp_entry) => {
                        warps.push(warp_entry);
                    }
                    Err(err) => {
                        panic!("Could not parse warp entry at {:?} with error {}", &file, err);
                    }
                }
            } 
        }
    }
    Ok(warps)
}