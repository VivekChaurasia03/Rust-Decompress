use std::fs;
use std::io;

/* The below structure between the main and real_main function is a pretty common
thing in a lot of RUST projects. Where real_main() is where all the task happens
and main() just helps you cleanly exit from the program. */
fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    /* To be able to work with the zips files we need to work with the arguments
    that the user will enter. */
    let args: Vec<_> = std::env::args().collect();

    // Just a basic check on the user input and the braces {} will print "cargo run"
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }

    /*
    &*args[1]: This involves referencing and dereferencing. Let's break it down:
     - args[1]: Accessing the element at index 1 in the args array (assuming args
    is an array or slice).
     - *args[1]: Dereferencing the value at index 1. If args[1] is a pointer, this
    would give you the value it points to.
     - &*args[1]: Referencing the value again. This is taking the reference of the
    dereferenced value.
    It's common to see this pattern when dealing with pointers or types that implement
    the Copy trait, where you want to take a reference instead of moving the value. */
    let fname = std::path::Path::new(&*args[1]);

    // To open the file
    let file = fs::File::open(&fname).unwrap();

    /* This mutable archive is going to help us work with that file and process that file.
    After the above code reads the file based on the name we need to copy the content somewhere
    and this mutable archive is going to help with that. */
    let mut archive = zip::ZipArchive::new(file).unwrap();

    /* There could be multiple files inside the archive and in order to go over them one by
    one we need the for loop. That's what the file inside the for loop is also doing, by
    going one by one. */
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        /* Just some steps picked up from RUST Documentations
        You can use this method to validate the name as a safe path */
        let outpath = match file.enclosed_name() {
            /* As we are working with borrowed data, we need to convert the data to_owned()
            We are just cloning the path and getting the path Somewhere */
            Some(path) => path.to_owned(),
            None => continue,
        };

        // As of now the Zip doesn't have any comments
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment:{}", i, comment);
            }
        }

        /* Logic to maintain the same folder structure as the Zip file
        ends_with a way to check if it is a directory or not */
        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            // Recursively create a directory and all of its parent components if they are missing.
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );

            // Checking for parents
            if let Some(p) = outpath.parent() {
                /* There could be multiple files that could have the same parent but
                we dont need to create parent again and again thats why the check. */
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            /* In the above print code we are just printing the file and their path
            but the actually writing is and file creation is done below. */
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        /* This will only work on Linux to set/change the permission after the folder is created.
        Get and Set permissions for the extracted files */
        /* #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        } */

        // Havent been able to find any documentation around set permissions for windows.
        /* #[cfg(windows)]
        {} */
    }
    // The way this function is implemented it returns an integer so just sending 0.
    0
}
