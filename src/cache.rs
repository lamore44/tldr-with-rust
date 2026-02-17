use std::fs;
use std::path::PathBuf;

fn cache_dir() -> Option<PathBuf>{
    dirs::cache_dir().map(|d| d.join("tldr-rust"))
}

//path file cache
fn page_path(command: &str) -> Option<PathBuf>{
    cache_dir().map(|d| d.join(format!("{}.md", command)))

}

//load cache local
//jika gagal load, return None (bukan error)
pub fn load(command: &str) -> Option<String>{
    let path = page_path(command)?;
    fs::read_to_string(path).ok()
}

//save ke cache local, error dihiraukan
pub fn save(command: &str, content: &str){
    if let Some(dir) = cache_dir(){
        if fs::create_dir_all(&dir).is_err(){
            return; //skip caching, soalnya gak bisa buat directory
        }
        if let Some(path)= page_path(command){
            let _ = fs::write(path, content); //error gak usah di handle, soalnya cuma cache
        }
    }
}

/// clear semua hal cache
#[allow(dead_code)]
pub fn clear_all(){
    if let Some(dir) = cache_dir(){
        let _ = fs::remove_dir_all(dir);

    }
}