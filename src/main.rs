// use std::fs;
use csv::Error;
use std::{env::var_os, path::PathBuf};
use std::path::Path;
use std::ffi::CString;
use std::os::raw::c_char;

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
  let mut reader = csv::Reader::from_path("list.csv")?;
  for record in reader.records() {
    let record = record?;
    let from = format!("{}\\{}", get_userprofile(), &record[0]);
    let to = format!("{}\\Saved Games\\{}", get_userprofile(), &record[1]);
    let from_path = Path::new(&from);
    let to_path = Path::new(&to);

    if from_path.exists() {
      if is_junction(&from) {
        println!("✅ {}", from);
      } else {
        if to_path.exists() {
          println!("⚠️ Can't move {} -> {}", from, to);
          println!("Folder in sync directory already exists");
        } else {
          println!("🔗 Moving and junctioning {} -> {}", from, to);
          move_and_junction(&from, &to);
        }
      }
    } else if to_path.exists() {
      if from_path.exists() {
        println!("⚠️ Can't link {} -> {}", to, from);
        println!("from sync directory. Save data at default location already exists.");
      } else {
        println!("🔗 Restoring junction {} -> {}", from, to);
        restore_junction(&from, &to);
      }
    }
  }

  Ok(())
}

fn is_junction(path: &str) -> bool {
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

fn move_and_junction(old_path: &str, new_path: &str) {
  move_save(old_path, new_path);
  link_save(new_path, old_path);
  hide_directory(old_path);
}

fn restore_junction(normal_path: &str, save_path: &str) {
  link_save(save_path, normal_path);
  hide_directory(normal_path);
}

fn create_junction(target: &str, junction: &str) {
  let _ = junction::create(target, junction);
}

fn move_save(old_path: &str, new_path: &str) {
  let result = std::fs::rename(old_path, new_path);
  match result {
    Ok(()) => return,
    Err(e) => {
      panic!("Problem renaming the directory: {:?}", e)
    }
  }
}

fn link_save(target: &str, junction: &str) {
  create_junction(target, junction);
}

#[cfg(windows)]
fn hide_directory(path: &str) {
  let c_path = CString::new(path).unwrap();
  let c_path_ptr: *const c_char = c_path.as_ptr() as *const c_char;
  unsafe {
    winapi::um::fileapi::SetFileAttributesA(
      c_path_ptr,
      winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN
    );
  }
}
