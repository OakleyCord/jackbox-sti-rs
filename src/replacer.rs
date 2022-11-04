use std::{fs, thread};
use std::path::{Path, PathBuf};
use image::DynamicImage;
use rand::Rng;


const DIR_REPLACE: &str = "./replacement images";
const DIR_ORIGINAL: &str = "./original images";
const DIR_OUTPUT: &str = "output";
const DIR_OUTPUT_TUMB: &str = "output/Thumbnails";

fn create_folders() {
    let original_path = PathBuf::from(DIR_ORIGINAL);
    let replace_path = PathBuf::from(DIR_REPLACE);
    let output_path = PathBuf::from(DIR_OUTPUT);
    let output_tumb_path = PathBuf::from(DIR_OUTPUT_TUMB);

    if !original_path.exists() {
        fs::create_dir(original_path).expect("Could not create original images folder");
    }
    if !replace_path.exists() {
        fs::create_dir(replace_path).expect("Could not create replacement images folder");
    }
    if !output_path.exists() {
        fs::create_dir(output_path).expect("Could not create output folder");
    }
    if !output_tumb_path.exists() {
        fs::create_dir(output_tumb_path).expect("Could not create output thumbnails folder");
    }
}


pub fn start() {
    create_folders();
    let files: Vec<PathBuf> = find_files_from_dir(DIR_ORIGINAL);
    println!("Found {} files in {}", files.len(), DIR_ORIGINAL);
    let mut files_replace: Vec<PathBuf> = find_files_from_dir(DIR_REPLACE);
    println!("Found {} files in {}", files_replace.len(), DIR_REPLACE);

    println!("Starting to replace images...");

    let file_count = files.len();
    let mut thread_handles = vec![];
    for filename in files {
        if files_replace.len() == 0 {
            break;
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..files_replace.len());
        let filename_replace = files_replace.remove(index);

        //run on separate thread
        let handle = thread::spawn(move || {
            let image = image::open(filename_replace).unwrap();
            convert_and_copy(&image, &filename);
            convert_and_copy_thumb(image, filename);
        });
        thread_handles.push(handle);
    }


    let mut i = 1;
    for handle in thread_handles {
        println!("{} / {}", i, file_count);
        handle.join().unwrap();
        i += 1;
    }
}

fn convert_and_copy_thumb(image: DynamicImage, filename: PathBuf) {
    //resize to 120x90 for thumbnail
    image.resize_exact(120, 90, image::imageops::FilterType::Nearest);

    let mut tumb_filename = filename.to_str().unwrap().replace(".jpg", "-thumb.jpg");
    tumb_filename = tumb_filename.replace("original images", DIR_OUTPUT_TUMB);
    //println!("Saving thumbnail to {}", tumb_filename);
    let out = Path::new(&tumb_filename);
    image.save(out).unwrap();
}

fn convert_and_copy(image: &DynamicImage, filename: &PathBuf) {
    //resize replacement image to 425x320;
    let image = image.resize_exact(425, 320, image::imageops::FilterType::Nearest);
    //unwrap hell
    let out = Path::new(&filename).parent().unwrap().parent().unwrap().join(DIR_OUTPUT).join(filename.file_name().unwrap());
    //println!("Saving to {}", out.to_str().unwrap());
    image.save(out).unwrap();
}




fn find_files_from_dir(dir: &str) -> Vec<PathBuf> {
    let mut filenames: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to get entry");
        let path = entry.path();
        if path.is_file() {
            filenames.push(path);
        }
    }
    filenames
}
