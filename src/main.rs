use rand::{distributions::Alphanumeric, Rng};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let files = std::fs::read_dir(&args[1]).unwrap();
    for dir_entry in files {
        let name = dir_entry.unwrap().path().to_string_lossy().into_owned();
        let file = std::path::Path::new(&name);
        if file.extension().is_none() {
            continue;
        }
        let ext = file.extension().unwrap().to_str().unwrap();
        if ["jpg", "mp4", "png", "webp", "jpeg", "gif"].contains(&ext) {
            let new_name: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(20)
                .map(char::from)
                .collect();
            let new_full_name = file.with_file_name(format!("{}.{}", new_name, ext));
            let _ = std::fs::rename(&name, &new_full_name);
            println!("{} -> {}", name, new_full_name.display());
        }
    }
}
