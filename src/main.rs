// use std::fs;
use csv::Error;
use std::{env::var_os, path::PathBuf};

pub fn get_userprofile() -> String {
    match var_os("USERPROFILE").map(PathBuf::from) {
      Some(path) => {
        return path.to_string_lossy().to_string();
      }
      None => {
        panic!("Path does not exist");
      }
    }
}

fn main() -> Result<(), Error> {
  let mut reader = csv::Reader::from_path("smol_list.csv")?;
  for record in reader.records() {
    let record = record?;
    let path = format!("{}\\{}", get_userprofile(), &record[0]);
    println!("From {} to {}", path, &record[1]);

    if is_junction(path) {
      println!("Junction");
    } else {
      println!("Not Junction");
      // move_and_junction
      // hide_directory
    }
  }

  Ok(())
}

fn is_junction(path: String) -> bool {
  let is_junction = junction::exists(path);

  match is_junction {
    Ok(true) => return true,
    Ok(false) => return false,
    Err(e) => {
      match e.raw_os_error() {
        Some(4390) => return false, // Not a junction
        _ => panic!("Problem checking the directory: {:?}", e)
      }
    }
  }
}

// fn move_and_junction(old_path: &str, new_path: &str) { }
// fn restore_junctions(old_path: &str, new_path: &str) { }
// fn restore_junction(old_path: &str, new_path: &str) { }

#[allow(dead_code)]
fn create_junction(old_path: &str, new_path: &str) {
  let _ = junction::create(old_path, new_path);
}

#[allow(dead_code)]
fn move_save(old_path: &str, new_path: &str) -> std::io::Result<()> {
  println!("{}", format!("Moving... {} -> {}", old_path, new_path));
  std::fs::rename(old_path, new_path)?;
  Ok(())
}

#[allow(dead_code)]
fn link_save(old_path: &str, new_path: &str) {
  create_junction(old_path, new_path);
}

// TODO
// #[allow(dead_code)]
// unsafe fn hide_directory(path: &str) {
//   winapi::um::fileapi::SetFileAttributesA(
//     path,
//     winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN
//   );
// }
