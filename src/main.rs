//mod data;
use std::{fs, thread, time, env};
use std::collections::HashSet;

const TIME_REFRESH: time::Duration = time::Duration::from_secs(60);

fn get_folder(type_file: &str) -> &str{
    match type_file{
        "doc" => return "Documentos",
        "media" => return "Media",
        "comp" => return "Comprimido",
        &_ => return "Otros"
    };
}

#[inline]
fn create_folder(dir: &String){ fs::create_dir(dir).expect("Error creando el directorio"); }

fn reorganize(hashes: &[(&str, HashSet<&str>)], path_dir: fs::ReadDir, dir: &String) -> (){
    for file in path_dir{
        let file = file.unwrap().path();
        // Checks where should be allocated
        if let Some(file_ext) = file.extension(){
            let mut folder = "Otros";
            for hash in hashes {                                                                                    
                if !hash.1.contains(file_ext.to_str().unwrap()) { continue; }
                folder = get_folder(hash.0);
                break;
            }
            let new_path = format!("{}{}/{}", dir, &folder, &file.file_name().unwrap().to_str().unwrap());
            if let Err(_) = fs::rename(&file, &new_path) {
                create_folder(&format!("{}{}/", dir, &folder));
                fs::rename(&file, &new_path).unwrap();
            }
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let dir = if args.len() == 1 { format!("/home/{}/Downloads/", whoami::username()) } else { args[1].clone() };
    let documentos: HashSet<&str> = HashSet::from(["doc", "docx", "pdf", "xls", "xlsx", "ppt", "pptx", "txt", "rtf", "odt", "pages", "wpd", "md"]);
    let media: HashSet<&str> = HashSet::from(["jpg", "jpeg", "png", "gif", "bmp", "tiff", "ico", "svg", "heif", "webp", "mp4", "mkv", "mov", "avi", "wmv", "flv", "m4v", "webm", "3gp", "ogg", "mpg", "mpeg"]);
    let compress: HashSet<&str> = HashSet::from(["zip", "rar", "7z", "tar", "gz", "bz2", "tgz", "tbz", "z", "cab", "lzh", "lzma", "xz", "sit", "dmg", "wim", "esd", "rpm", "deb", "jar"]);
    let hashes: [(&str, HashSet<&str>); 3] = [("doc", documentos), ("media", media), ("comp",compress)];
    loop{
        let path_dir: fs::ReadDir = fs::read_dir(&dir)
            .expect("Ha habido un error con el directorio.");
        reorganize(&hashes, path_dir, &dir);
        thread::sleep(TIME_REFRESH);
    }
}
