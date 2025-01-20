use std::io;
use std::io::Error;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "Mason Hall",
    version,
    about = "A program to remove specified empty directories"
)]
struct Args {
    #[arg(required = true)]
    /// List of empty directories to remove
    paths: Vec<String>,

    #[arg(short, long)]
    /// Include to show verbose output
    verbose: bool,

    #[arg(short, long)]
    /// Remove directory and parents if they are empty
    parents: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.paths.is_empty() {
        eprintln!("No directories given.");
        return Ok(());
    }

    rmdir(args.paths, args.verbose, args.parents)
}

fn rmdir(path_names: Vec<String>, verbose: bool, recursive: bool) -> io::Result<()> {
    let mut path_names = Box::new(path_names);
    if !path_names.is_empty() {
        while let Some(path) = path_names.pop() {
            match remove_path(path.clone()) {
                Err(e) => {
                    eprintln!("rmdir: {}: {}", path, e);
                }
                _ => {
                    if verbose {
                        println!("Removed {}", path);
                    }
                }
            };
            if recursive {
                let len = path.len();
                for (i, &item) in path.as_bytes().iter().rev().enumerate() {
                    if item == b'/' {
                        match remove_path(path[..(len - i - 1)].to_string()) {
                            Err(e) => {
                                eprintln!("rmdir: {}: {}", &path[..(len - i - 1)], e);
                            }
                            _ => {
                                if verbose {
                                    println!("Removed {}", &path[..(len - i - 1)]);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn remove_path(path_name: String) -> io::Result<()> {
    if path_name.is_empty() {
        return Err(Error::new(io::ErrorKind::NotADirectory, "Empty string..."));
    }
    std::fs::remove_dir(path_name.trim())
}

#[cfg(test)]
mod rmdir {
    use std::path::Path;

    use crate::rmdir;

    #[test]
    fn remove_paths() {
        if std::fs::create_dir_all("/tmp/rmdir/test").is_ok() {
            let paths = vec![String::from("/tmp/rmdir"), String::from("/tmp/rmdir/test")];
            match rmdir(paths, false, false) {
                Ok(_) => {
                    assert!(!Path::new("/tmp/rmdir/test").exists());
                    assert!(!Path::new("/tmp/rmdir").exists());
                }
                Err(_) => {
                    panic!("failed to remove directories")
                }
            }
        } else {
            panic!("failed to create directories");
        }
    }

    #[test]
    fn remove_paths_recursive() {
        if std::fs::create_dir_all("/tmp/rmdir/test").is_ok() {
            let paths = vec![String::from("/tmp/rmdir"), String::from("/tmp/rmdir/test")];
            match rmdir(paths, false, true) {
                Ok(_) => {
                    assert!(!Path::new("/tmp/rmdir/test").exists());
                    assert!(!Path::new("/tmp/rmdir").exists());
                }
                Err(_) => {
                    panic!("failed to remove directories")
                }
            }
        } else {
            panic!("failed to create directories");
        }
    }
}
