use common_models::models::errors::canonical_path::CanonicalPathError;
use common_models::models::errors::canonical_path_combo::IOCanonicalError;
use common_models::models::errors::directory_resolve::DirectoryResolveError;
use common_models::models::errors::directory_resolve::Type::{ProjectDirNotFound, UserDirNotFound};
use directories::{ProjectDirs, UserDirs};
use sha2::{Digest, Sha256};
use std::ffi::OsStr;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};
use std::{env, io};
use std::hash::{BuildHasher, Hash};
use indexmap::IndexSet;

pub mod common_models;
pub mod config_resolver;
pub mod argument_parser;

#[macro_export]
macro_rules! print_dbg_on_debug {
    ($($rest:tt)*) => {
        #[cfg(debug_assertions)]
        std::dbg!($($rest)*)
    }
}

#[macro_export]
macro_rules! println_on_debug {
    ($($rest:tt)*) => {
        #[cfg(debug_assertions)]
        std::println!($($rest)*)
    }
}

#[cfg(target_family = "windows")]
pub fn safe_canonicalize(path: &Path) -> Result<PathBuf, IOCanonicalError> {
    let absolute_path = match path.canonicalize() {
        Ok(value) => value,
        Err(error) => {
            return Err(IOCanonicalError::IOError(error));
        }
    };

    match remove_windows_prefix_path(&absolute_path) {
        Ok(value) => Ok(value),
        Err(error) => {
            return Err(IOCanonicalError::CanonicalError(error));
        }
    }
}

#[cfg(not(target_family = "windows"))]
pub fn safe_canonicalize(path: &Path) -> Result<PathBuf, IOCanonicalError> {
    match path.canonicalize() {
        Ok(value) => Ok(value),
        Err(error) => {
            return Err(IOCanonicalError::IOError(error));
        }
    }
}

#[cfg(target_family = "windows")]
pub fn remove_windows_prefix_path<P: AsRef<Path>>(path: &P) -> Result<PathBuf, CanonicalPathError> {
    let prefix = r"\\?\";
    let mut str_path = match path.as_ref().to_str() {
        None => {
            return Err(CanonicalPathError {});
        }
        Some(value) => value,
    };

    if str_path.starts_with(prefix) {
        str_path = &str_path[prefix.len()..];
    }

    Ok(PathBuf::from(str_path))
}

#[cfg(not(target_family = "windows"))]
pub fn remove_windows_prefix_path<P: AsRef<Path>>(path: &P) -> Result<PathBuf, CanonicalPathError> {
    Ok(PathBuf::from(path.as_ref()))
}

pub fn current_working_path() -> Result<PathBuf, IOCanonicalError> {
    let path = match env::current_dir() {
        Ok(value) => value,
        Err(error) => {
            return Err(IOCanonicalError::IOError(error));
        }
    };

    safe_canonicalize(&path)
}

#[cfg(target_os = "linux")]
pub fn current_exec_file_path() -> Result<PathBuf, IOCanonicalError> {
    let filename = match env::args().nth(0) {
        None => {
            return Err(IOCanonicalError::CanonicalError(
                // TODO: may need to separate error in future
                CanonicalPathError {},
            ));
        }
        Some(value) => {
            match value.len() > 2 && value[0..2].to_string().eq("./") {
                true => {
                    PathBuf::from(value[2..].to_string())
                }
                false => {
                    PathBuf::from(value)
                }
            }
        },
    };

    let path = if filename.is_absolute() {
        filename
    } else {
        current_working_path()?.join(filename)
    };

    return Ok(path);
}

#[cfg(not(target_os = "linux"))]
pub fn current_exec_file_path() -> Result<PathBuf, IOCanonicalError> {
    let path = match env::current_exe() {
        Ok(value) => value,
        Err(error) => {
            return Err(IOCanonicalError::IOError(error));
        }
    };

    safe_canonicalize(&path)
}

pub fn pause_command_line() -> io::Result<String> {
    print!("Press enter to continue ... ");
    read_input()
}

pub fn read_input() -> io::Result<String> {
    let mut input = String::new();
    stdout().flush()?;
    stdin().read_line(&mut input)?;

    Ok(input)
}

#[cfg(debug_assertions)]
pub fn pause_project_for_debug() {
    println!("Paused because of pause_project_for_debug().");
    pause_command_line().expect("couldn't pause.");
}

#[cfg(not(debug_assertions))]
pub fn pause_project_for_debug() {}

pub trait PathCompare {
    fn contains(&self, another_path: &Path) -> bool;
}

pub trait PathBufCompare {
    fn contains(&self, another_path: &PathBuf) -> bool;
}

impl PathCompare for Path {
    fn contains(&self, another_path: &Path) -> bool {
        compare_paths(self, another_path)
    }
}

impl PathBufCompare for PathBuf {
    fn contains(&self, another_path: &PathBuf) -> bool {
        compare_paths(self, another_path)
    }
}

fn compare_paths<P: AsRef<Path> + ?Sized>(path_1: &P, path_2: &P) -> bool {
    let current_vec: Vec<&OsStr> = path_1.as_ref().iter().collect();
    let another_vec: Vec<&OsStr> = path_2.as_ref().iter().collect();

    if current_vec.len() > another_vec.len() {
        return false;
    }

    for (index, value) in current_vec.iter().enumerate() {
        if !value.eq(&another_vec[index]) {
            return false;
        }
    }

    true
}

#[cfg(target_family = "windows")]
pub fn fix_color_options_on_windows() {
    colored::control::set_virtual_terminal(true).unwrap();
    colored::control::set_override(true);
}

#[cfg(not(target_family = "windows"))]
pub fn fix_color_options_on_windows() {}

pub fn string_to_hash(value: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn project_dirs() -> Result<ProjectDirs, DirectoryResolveError> {
    return match ProjectDirs::from("com", "ysw2k", "package-select") {
        None => Err(DirectoryResolveError {
            error_type: ProjectDirNotFound,
        }),
        Some(value) => Ok(value),
    };
}

pub fn user_dirs() -> Result<UserDirs, DirectoryResolveError> {
    return match UserDirs::new() {
        None => Err(DirectoryResolveError {
            error_type: UserDirNotFound,
        }),
        Some(value) => Ok(value),
    };
}

pub trait Upsert<T>
where
    Self: Sized,
{
    fn upsert_by<F>(&mut self, data: T, func: F)
        where
            F: FnMut(&T, &T) -> bool;
}

impl<T> Upsert<T> for Vec<T> {
    fn upsert_by<F>(&mut self, data: T, mut func: F)
        where
            F: FnMut(&T, &T) -> bool, {
        for (index, value) in self.iter().enumerate() {
            match func(value, &data) {
                true => {
                    self.insert(index + 1, data);
                    self.remove(index);
                    return;
                }
                false => {}
            }
        }

        self.push(data);
    }
}

impl<T, S> Upsert<T> for IndexSet<T, S>
    where
        Self: InsertTo<T>,
        T: Hash + Eq,
        S: BuildHasher, {
    fn upsert_by<F>(&mut self, data: T, mut func: F)
        where
            F: FnMut(&T, &T) -> bool {
        for (index, value) in self.iter().enumerate() {
            match func(value, &data) {
                true => {
                    self.insert_to(data, index);
                    self.shift_remove_index(index + 1);
                    return;
                }
                false => {}
            }
        }

        self.insert(data);
    }
}

pub enum ProjectType {
    ProjectSelectCLI,
    ProjectSelectRunner,
}

pub fn project_filename(project_type: ProjectType) -> String {
    let filename = match project_type {
        ProjectType::ProjectSelectCLI => "pkg_select_cli",
        ProjectType::ProjectSelectRunner => "pkg_select_runner",
    };

    if cfg!(windows) {
        let filename_with_extension = format!("{}.exe", filename);

        return filename_with_extension;
    }

    return filename.to_string();
}

pub trait PathPop {
    fn pop_path(&self) -> PathBuf;
}

impl PathPop for Path {
    fn pop_path(&self) -> PathBuf {
        pop_remaining_path(&self)
    }
}

impl PathPop for PathBuf {
    fn pop_path(&self) -> PathBuf {
        pop_remaining_path(&self)
    }
}

fn pop_remaining_path<P: AsRef<Path> + ?Sized + AsRef<OsStr>>(path: &P) -> PathBuf {
    let mut new_path = PathBuf::new();
    let copied = PathBuf::from(path);
    let mut path_vec: Vec<&OsStr> = copied.iter().collect();
    path_vec.pop();

    for value in path_vec.iter() {
        new_path.push(value);
    }

    new_path
}

pub struct MutationLocker<'t, T> {
    value: &'t mut T,
    unlocked: bool,
}

impl<T> MutationLocker<'_, T> {
    pub fn create(value: &mut T, unlocked: bool) -> MutationLocker<T> {
        MutationLocker {
            value,
            unlocked,
        }
    }

    pub fn value(&self) -> &T {
        self.value as &T
    }

    pub fn value_mut(&mut self) -> Option<&mut T> {
        if !self.unlocked {
            return None;
        }

        Some(self.value)
    }
}

impl<'t, T> From<&'t mut MutationLocker<'t, T>> for MutationLocker<'t, T> {
    fn from(data: &'t mut MutationLocker<'t, T>) -> MutationLocker<'t, T> {
        MutationLocker::create(data.value, data.unlocked)
    }
}

pub trait InsertTo<T> {
    fn insert_to(&mut self, value: T, index: usize) -> bool;
}

impl<T, S> InsertTo<T> for IndexSet<T, S>
    where T: Eq + Hash,
          S: BuildHasher, {
    fn insert_to(&mut self, value: T, index: usize) -> bool {
        if !self.insert(value) {
            return false;
        }
        if self.len() > 1 {
            let length = self.len();
            self.move_index( length - 1, index);
        }

        true
    }
}
