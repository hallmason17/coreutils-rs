use std::io;
use std::io::Error;

use clap::Parser;
use env_logger::Env;
use log::debug;
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
    /// Include to reverse the order of directories to remove
    reverse: bool,
}
fn main() -> io::Result<()> {
    let env = Env::default().filter_or("RUST_LOG", "error");
    env_logger::init_from_env(env);
    let args = Args::parse();
    debug!("{:?}", args);

    if args.paths.is_empty() {
        eprintln!("No directories given.");
        return Ok(());
    }

    rmdir(args.paths, args.verbose)
}

fn rmdir(paths: Vec<String>, verbose: bool) -> io::Result<()> {
    let mut paths = Box::new(paths);
    if !paths.is_empty() {
        while let Some(path) = paths.pop() {
            match remove_path(path.clone()) {
                Err(e) => {
                    eprintln!("rmdir: {}: {}", path, e);
                }
                _ => {
                    if verbose {
                        println!("Removed {}", path);
                    }
                }
            }
        }
    }
    Ok(())
}

fn remove_path(path: String) -> io::Result<()> {
    if path.is_empty() {
        return Err(Error::new(io::ErrorKind::NotADirectory, "Empty string..."));
    }
    std::fs::remove_dir(path.trim())
}

#[cfg(test)]
mod rmdir {
    use std::cell::RefCell;

    use crate::remove_path;

    #[test]
    fn remove_paths() {
        if std::fs::create_dir_all("/tmp/rmdir/test").is_ok() {
            let paths = RefCell::new(vec![
                String::from("/tmp/rmdir"),
                String::from("/tmp/rmdir/test"),
            ]);
            while let Some(path) = paths.borrow_mut().pop() {
                let result = remove_path(path);
                assert!(result.is_ok());
            }
        } else {
            panic!("failed to create directories");
        }
    }
}
