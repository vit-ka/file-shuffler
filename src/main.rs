use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let files: Vec<_> = std::fs::read_dir(&args[1])
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension().and_then(|x| x.to_str()).map_or(false, |ext| {
                ["jpg", "mp4", "png", "webp", "jpeg", "gif"].contains(&ext)
            })
        })
        .collect();

    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("{bar:40} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("=> "),
    );

    files.par_iter().for_each(|dir_entry| {
        let name = dir_entry.path().to_string_lossy().into_owned();
        let file = std::path::Path::new(&name);
        let ext = file.extension().unwrap().to_str().unwrap();
        let stem = file.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if stem.len() == 32 && stem.chars().all(|c| c.is_ascii_hexdigit()) {
            pb.set_message(format!("skip: {}.{}", stem, ext));
            pb.inc(1);
            return;
        }
        let content = std::fs::read(&name).unwrap();
        let hash = format!("{:x}", md5::compute(&content));
        let new_full_name = file.with_file_name(format!("{}.{}", hash, ext));
        let new_file_name = format!("{}.{}", hash, ext);
        if file == new_full_name {
            pb.set_message(format!("skip: {}", new_file_name));
        } else {
            match std::fs::rename(&name, &new_full_name) {
                Ok(_) => pb.set_message(format!("rename: {}", new_file_name)),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    std::fs::remove_file(&name).unwrap();
                    pb.set_message(format!("duplicate: {}", file.file_name().unwrap().to_string_lossy()));
                }
                Err(e) => panic!("{}", e),
            }
        }
        pb.inc(1);
    });

    pb.finish_with_message("done");
}
