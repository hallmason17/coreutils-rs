use std::io;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "Mason Hall",
    version,
    about = "A program to remove specified empty directories"
)]
struct Args {
    #[arg(required = true)]
    /// List of directories to create
    paths: Vec<String>,

    #[arg(short, long)]
    /// Include to show verbose output
    verbose: bool,

    #[arg(short, long)]
    /// Create directory and parents if necessary
    parents: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.paths.is_empty() {
        eprintln!("No directories given.");
        return Ok(());
    }

    mkdir(args.paths, args.verbose, args.parents)
}

fn mkdir(path_names: Vec<String>, verbose: bool, recursive: bool) -> io::Result<()> {
    let mut path_names = Box::new(path_names);
    if !path_names.is_empty() {
        while let Some(path) = path_names.pop() {
            if recursive {
                match std::fs::create_dir_all(path.clone()) {
                    Err(e) => {
                        eprintln!("mkdir: {}: {}", path, e)
                    }
                    _ => {
                        if verbose {
                            println!("{}", path)
                        }
                    }
                }
            } else {
                match std::fs::create_dir(path.clone()) {
                    Err(e) => {
                        eprintln!("mkdir: {}: {}", path, e)
                    }
                    _ => {
                        if verbose {
                            println!("{}", path)
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod mkdir {
    use crate::mkdir;
    use std::path::Path;
    #[test]
    fn make_paths() {
        let paths = vec![String::from("/tmp/mkdir/test"), String::from("/tmp/mkdir")];
        match mkdir(paths, false, false) {
            Ok(_) => {
                assert!(Path::new("/tmp/mkdir").exists());
                assert!(Path::new("/tmp/mkdir/test").exists());
                let _ = std::fs::remove_dir("/tmp/mkdir/test");
                let _ = std::fs::remove_dir("/tmp/mkdir");
            }
            Err(_) => {
                panic!("failed to make directories")
            }
        }
    }

    #[test]
    fn make_paths_r() {
        let paths = vec![String::from("/tmp/mkdir/test")];
        match mkdir(paths, false, true) {
            Ok(_) => {
                assert!(Path::new("/tmp/mkdir").exists());
                assert!(Path::new("/tmp/mkdir/test").exists());
                let _ = std::fs::remove_dir("/tmp/mkdir/test");
                let _ = std::fs::remove_dir("/tmp/mkdir");
            }
            Err(_) => {
                panic!("failed to make directories")
            }
        }
    }
}
