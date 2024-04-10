use std::fs;
use std::io;


// Driver function - allows you to exit cleanly
fn main(){
    std::process::exit(real_main());
}

// 
fn real_main() -> i32 {
    // We need to supply arguments to the program
    // The arguments are collected inside a vector datatype
    let args: Vec<_> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }

    let fname = std::path::Path::new(&* args[1]);
    let file = fs::File::open(&fname).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name(){
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment)
            }
        }
        // Detecting whether it is a file or a directory ?
        // So, if the filename ends with a forward slash then it most likely
        // is a directory (edge case - unconventional file naming)
        if (*file.name()).ends_with("/"){

            println!("File {} extracted to \"{}\"", i, outpath.display());
            // Recurisvely create directories which do not exist
            fs::create_dir_all(&outpath).unwrap();

        // If it is not a folder, then it is a file and is treated likewise
        } else {
            println!("File {} extracted to \"{}\" ({} bytes)", 
                     i, 
                     outpath.display(), 
                     file.size()
                     );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            // The actual copying happens here
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Unix specific configuration - handling permissions and ownership
        #[cfg(unix)] 
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode(){
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    return 0; // Exit code
}

