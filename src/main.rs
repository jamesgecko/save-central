// use std::fs;

fn main() {
  if is_junction(r"C:\Users\james\AppData\Roaming\Baba_Is_You") {
    println!("Junction");
  } else {
    println!("Not Junction");
  }
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

// fn move_and_junction(old_path: &str, new_path: &str) { }
// fn restore_junctions(old_path: &str, new_path: &str) { }
// fn restore_junction(old_path: &str, new_path: &str) { }

#[allow(dead_code)]
fn create_junction(old_path: &str, new_path: &str) {
  let _ = junction::create(old_path, new_path);
}

#[allow(dead_code)]
fn move_save(old_path: &str, new_path: &str) -> std::io::Result<()> {
  println!(format!("Moving... {} -> {}", old_path, new_path));
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
