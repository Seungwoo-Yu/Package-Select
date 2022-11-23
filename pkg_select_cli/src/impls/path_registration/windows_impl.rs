#![cfg(target_os = "windows")]

use std::collections::HashSet;
use std::u32;
use std::ops::BitOr;
use std::path::PathBuf;
use windows::core::PCSTR;
use windows::Win32::System::Registry::{HKEY, HKEY_CURRENT_USER, KEY_READ, KEY_WRITE, REG_EXPAND_SZ, REG_VALUE_TYPE, RegCloseKey, RegDeleteKeyValueA, RegDeleteValueA, RegGetValueA, RegOpenKeyExA, RegQueryValueExA, RegSetValueExA, RRF_RT_ANY};
use crate::models::errors::path_registration::PathRegistrationError;
use crate::models::errors::path_registration::Type::{DuplicatedTarget, DuplicatedTargets};
use crate::models::errors::path_registration_combo::DirectoryIOPathRegistrationError;
use crate::models::errors::windows_error_wrapper::{ErrorCodes, WindowsError};
use crate::models::path_registration_resolver::PathRegistrationResolver;
use crate::traits::path_registration::{MultiplePathRegistration, PathRegistration, PathRegistrationReset};

impl PathRegistration for PathRegistrationResolver {
    fn registered(&self, target: &PathBuf) -> Result<bool, DirectoryIOPathRegistrationError> {
        let mut path = PathBuf::new().join(target);

        if path.is_file() {
            let _ = &path.pop();
        }

        let mut registry_key = registry_key()?;
        let (data, _) = get_or_create_registry_set(registry_key, WINDOWS_PACKAGE_SELECT_PATH)?;
        let path_string = path.to_string_lossy().to_string();
        let result = append_set_value(
            registry_key,
            WINDOWS_PACKAGE_SELECT_PATH,
            path_string,
            Some(data.to_string()),
        ).is_err();

        invalidate_registry_key(&mut registry_key)?;

        Ok(result)
    }

    fn register(&self, target: &PathBuf) -> Result<(), DirectoryIOPathRegistrationError> {
        let mut path = PathBuf::new().join(target);

        if path.is_file() {
            let _ = &path.pop();
        }

        let mut registry_key = registry_key()?;
        let (data, data_type) = get_or_create_registry_set(registry_key, WINDOWS_PACKAGE_SELECT_PATH)?;
        let path_string = path.to_string_lossy().to_string();
        let changed = append_set_value(
            registry_key,
            WINDOWS_PACKAGE_SELECT_PATH,
            path_string,
            Some(data),
        )?;

        link_select_path_to_path(registry_key)?;
        update_registry_set(
            registry_key,
            WINDOWS_PACKAGE_SELECT_PATH,
            &changed.join(";"),
            data_type
        )?;
        invalidate_registry_key(&mut registry_key)?;

        Ok(())
    }

    fn unregister(&self, target: &PathBuf) -> Result<(), DirectoryIOPathRegistrationError> {
        let mut path = PathBuf::new().join(target);

        if path.is_file() {
            let _ = &path.pop();
        }

        let mut registry_key = registry_key()?;
        let (data, data_type) = get_or_create_registry_set(registry_key, WINDOWS_PACKAGE_SELECT_PATH)?;
        let path_value = path.to_string_lossy().to_string();
        let changed = remove_value_if_applicable(
            registry_key,
            WINDOWS_PACKAGE_SELECT_PATH,
            &path_value,
            Some(data)
        )?;

        update_registry_set(
            registry_key,
            WINDOWS_PACKAGE_SELECT_PATH,
            &changed.join(";"),
            data_type
        )?;
        invalidate_registry_key(&mut registry_key)?;

        Ok(())
    }
}

impl MultiplePathRegistration for PathRegistrationResolver {
    fn register(&self, target: &Vec<&PathBuf>) -> Result<(), DirectoryIOPathRegistrationError> {
        let paths: Vec<String> = target.iter().map(| value | {
            let mut path = PathBuf::new().join(value);

            if path.is_file() {
                let _ = &path.pop();
            }

            path.to_string_lossy().to_string()
        }).collect();

        let mut registry_key = registry_key()?;
        let (data, data_type) = get_or_create_registry_set(registry_key, WINDOWS_PACKAGE_SELECT_PATH)?;
        let changed = append_set_values(
            registry_key,
            WINDOWS_PACKAGE_SELECT_PATH,
            paths,
            Some(data),
        )?;

        link_select_path_to_path(registry_key)?;
        update_registry_set(
            registry_key,
            WINDOWS_PACKAGE_SELECT_PATH,
            &changed.join(";"),
            data_type
        )?;
        invalidate_registry_key(&mut registry_key)?;

        Ok(())
    }

    fn unregister(&self, target: &Vec<&PathBuf>) -> Result<(), DirectoryIOPathRegistrationError> {
        let paths: Vec<String> = target.iter().map(| value | {
            let mut path = PathBuf::new().join(value);

            if path.is_file() {
                let _ = &path.pop();
            }

            path.to_string_lossy().to_string()
        }).collect();

        let mut registry_key = registry_key()?;
        let (data, data_type) = get_or_create_registry_set(registry_key, WINDOWS_PACKAGE_SELECT_PATH)?;
        let changed = remove_values_if_applicable(
            registry_key,
            WINDOWS_PACKAGE_SELECT_PATH,
            &paths,
            Some(data),
        )?;

        update_registry_set(
            registry_key,
            WINDOWS_PACKAGE_SELECT_PATH,
            &changed.join(";"),
            data_type
        )?;
        invalidate_registry_key(&mut registry_key)?;

        Ok(())
    }
}

impl PathRegistrationReset for PathRegistrationResolver {
    fn reset(&self) -> Result<(), DirectoryIOPathRegistrationError> {
        reset_registry()
    }
}

const WINDOWS_PACKAGE_SELECT_PATH: &str = "Package_Select_Path";
const WINDOWS_PATH: &str = "Path";
const MAX_VALUE_LENGTH: u32 = 16383;

fn reset_registry() -> Result<(), DirectoryIOPathRegistrationError> {
    let mut registry_key = registry_key()?;

    let data_string = WINDOWS_PACKAGE_SELECT_PATH.to_string();
    let data_var_string = format!("%{}%", &data_string);
    remove_value_if_applicable(
        registry_key,
        WINDOWS_PATH,
        &data_var_string,
        None,
    )?;
    remove_registry_set(registry_key, WINDOWS_PACKAGE_SELECT_PATH)?;
    invalidate_registry_key(&mut registry_key)
}

fn link_select_path_to_path(registry_key: HKEY) -> Result<(), DirectoryIOPathRegistrationError> {
    let path = format!("%{}%", WINDOWS_PACKAGE_SELECT_PATH);
    let (data, data_type) = get_or_create_registry_set(registry_key, WINDOWS_PATH)?;
    let changed = match append_set_value(
        registry_key,
        WINDOWS_PATH,
        path,
        Some(data),
    ) {
        Ok(value) => value,
        Err(error) => {
            if let DirectoryIOPathRegistrationError::PathRegistrationError(value) = &error {
                if let DuplicatedTarget(_) = value.error_type {
                    return Ok(())
                }
                if let DuplicatedTargets = value.error_type {
                    return Ok(())
                }
            }

            return Err(error);
        }
    };

    update_registry_set(
        registry_key,
        WINDOWS_PATH,
        &changed.join(";"),
        data_type
    )
}

fn registry_key() -> Result<HKEY, DirectoryIOPathRegistrationError> {
    let root_key = HKEY(HKEY_CURRENT_USER.0);

    let mut registry_key = HKEY(0);
    let registry_key_ptr: *mut HKEY = &mut registry_key;

    let sub_key = format!("Environment\0");
    let sub_key_byte = sub_key.as_bytes();
    let sub_key_cstr = PCSTR::from_raw(sub_key_byte.as_ptr());

    unsafe {
        match RegOpenKeyExA(
            root_key,
            sub_key_cstr,
            0,
            KEY_WRITE.bitor(KEY_READ),
            registry_key_ptr,
        ).ok() {
            Ok(_) => {}
            Err(error) => {
                return Err(
                    DirectoryIOPathRegistrationError::WindowsError(
                        WindowsError::from(error)
                    )
                );
            }
        }
    }

    Ok(registry_key)
}

fn registry_set(registry_key: HKEY, name: &str) -> Result<(String, REG_VALUE_TYPE), DirectoryIOPathRegistrationError> {
    let value_key = format!("{}\0", name);
    let value_key_byte = value_key.as_bytes();
    let value_key_cstr = PCSTR::from_raw(value_key_byte.as_ptr());

    let mut value_type = REG_VALUE_TYPE::default();
    let value_type_ptr: *mut REG_VALUE_TYPE = &mut value_type;

    let mut data_length = MAX_VALUE_LENGTH;
    let data_length_ptr: *mut u32 = &mut data_length;
    let mut data: Vec<u8> = Vec::with_capacity(data_length as usize);

    unsafe {
        match RegQueryValueExA(
            registry_key,
            value_key_cstr,
            None,
            Some(value_type_ptr),
            Some((&mut data).as_mut_ptr()),
            Some(data_length_ptr),
        ).ok() {
            Ok(_) => {}
            Err(error) => {
                return Err(
                    DirectoryIOPathRegistrationError::WindowsError(
                        WindowsError::from(error)
                    )
                );
            }
        }

        (&mut data).set_len(data_length as usize);
    }



    match String::from_utf8(data) {
        Ok(value) => {
            Ok(((&value).replace("\0", ""), value_type))
        },
        Err(error) => {
            return Err(DirectoryIOPathRegistrationError::FromUtf8Error(error));
        }
    }
}

fn get_or_create_registry_set(registry_key: HKEY, name: &str) -> Result<(String, REG_VALUE_TYPE), DirectoryIOPathRegistrationError> {
    let data = match registry_set(registry_key, name) {
        Ok(value) => value,
        Err(error) => {
            match &error {
                DirectoryIOPathRegistrationError::WindowsError(error2) => {
                    match &error2.code {
                        ErrorCodes::ErrCodeRegNotFound => {}
                        _ => {
                            return Err(error);
                        }
                    }
                }
                _ => {
                    return Err(error);
                }
            }

            update_registry_set(registry_key, WINDOWS_PACKAGE_SELECT_PATH, &format!(""), REG_EXPAND_SZ)?;
            registry_set(registry_key, WINDOWS_PACKAGE_SELECT_PATH)?
        }
    };

    Ok(data)
}

fn registry_set_value(name: &String) -> Result<(String, HKEY, REG_VALUE_TYPE), DirectoryIOPathRegistrationError> {
    let root_key = HKEY(HKEY_CURRENT_USER.0);

    let sub_key = format!("Environment\0");
    let sub_key_byte = sub_key.as_bytes();
    let sub_key_cstr = PCSTR::from_raw(sub_key_byte.as_ptr());

    let value_key = format!("{}\0", name);
    let value_key_byte = value_key.as_bytes();
    let value_key_cstr = PCSTR::from_raw(value_key_byte.as_ptr());

    let mut value_type = u32::default();
    let value_type_ptr: *mut u32 = &mut value_type;

    let mut data_length = MAX_VALUE_LENGTH;
    let data_length_ptr: *mut u32 = &mut data_length;
    let mut data: Vec<u8> = Vec::with_capacity(data_length as usize);

    unsafe {
        match RegGetValueA(
            root_key,
            sub_key_cstr,
            value_key_cstr,
            RRF_RT_ANY,
            Some(value_type_ptr),
            Some(data.as_mut_ptr() as _),
            Some(data_length_ptr),
        ).ok() {
            Ok(_) => {}
            Err(error) => {
                return Err(
                    DirectoryIOPathRegistrationError::WindowsError(
                        WindowsError::from(error)
                    )
                );
            }
        }

        data.set_len(data_length as usize);
    }

    if data.len() > 0 {
        data.pop();
    }

    match String::from_utf8(data) {
        Ok(value) => Ok((value, root_key, REG_VALUE_TYPE(value_type))),
        Err(error) => {
            return Err(DirectoryIOPathRegistrationError::FromUtf8Error(error));
        }
    }
}

fn append_set_value(
    registry_key: HKEY,
    name: &str,
    data: String,
    existed_registry_set: Option<String>,
) -> Result<Vec<String>, DirectoryIOPathRegistrationError> {
    let _existed = match existed_registry_set {
        None => get_or_create_registry_set(registry_key, name)?.0,
        Some(value) => value
    };
    let existed: Vec<&str> = _existed.split(";")
        .filter(| value | !value.is_empty())
        .chain(vec![data.as_str()])
        .collect();
    let updated: HashSet<String> = existed.iter().map(| value | value.to_string()).collect();

    if existed.len() != updated.len() {
        return Err(
            DirectoryIOPathRegistrationError::PathRegistrationError(
                PathRegistrationError { error_type: DuplicatedTarget(data) }
            )
        )
    }

    Ok(updated.into_iter().collect())
}

fn append_set_values(
    registry_key: HKEY,
    name: &str,
    data: Vec<String>,
    existed_registry_set: Option<String>,
) -> Result<Vec<String>, DirectoryIOPathRegistrationError> {
    let _existed = match existed_registry_set {
        None => get_or_create_registry_set(registry_key, name)?.0,
        Some(value) => value
    };
    let existed: Vec<String> = _existed.split(";")
        .filter(| value | !value.is_empty())
        .map(| value | value.to_string())
        .chain(data)
        .collect();
    let existed_count = existed.len();
    let updated: HashSet<String> = existed.into_iter().collect();
    let updated_count = updated.len();

    if existed_count != updated_count {
        return Err(
            DirectoryIOPathRegistrationError::PathRegistrationError(
                PathRegistrationError { error_type: DuplicatedTargets }
            )
        )
    }

    Ok(updated.into_iter().collect())
}

fn remove_value_if_applicable(
    registry_key: HKEY,
    name: &str,
    data_string: &String,
    existed_registry_set: Option<String>,
) -> Result<Vec<String>, DirectoryIOPathRegistrationError> {
    let _existed = match existed_registry_set {
        None => get_or_create_registry_set(registry_key, name)?.0,
        Some(value) => value
    };
    let existed: Vec<&str> = _existed.split(";")
        .filter(| value | value.is_empty())
        .collect();

    let filtered: Vec<String> = existed.iter()
        .filter(| value | (**value).eq(data_string.as_str()))
        .map(| value | value.to_string())
        .collect();

    Ok(filtered)
}

fn remove_values_if_applicable(
    registry_key: HKEY,
    name: &str,
    data_strings: &Vec<String>,
    existed_registry_set: Option<String>,
) -> Result<Vec<String>, DirectoryIOPathRegistrationError> {
    let _existed = match existed_registry_set {
        None => get_or_create_registry_set(registry_key, name)?.0,
        Some(value) => value
    };
    let existed: Vec<String> = _existed.split(";")
        .filter(| value | value.is_empty())
        .map(| value | value.to_string())
        .collect();

    let filtered: Vec<String> = existed.into_iter()
        .filter(| value | !data_strings.contains(value))
        .collect();

    Ok(filtered)
}

fn update_registry_set(
    registry_key: HKEY,
    name: &str,
    data: &String,
    data_type: REG_VALUE_TYPE,
) -> Result<(), DirectoryIOPathRegistrationError> {
    let value_key = format!("{}\0", name);
    let value_key_byte = value_key.as_bytes();
    let value_key_cstr = PCSTR::from_raw(value_key_byte.as_ptr());

    let converted = data.as_bytes();

    unsafe {
        match RegSetValueExA(
            registry_key,
            value_key_cstr,
            0,
            data_type,
            Some(converted),
        ).ok() {
            Ok(_) => {}
            Err(error) => {
                return Err(
                    DirectoryIOPathRegistrationError::WindowsError(
                        WindowsError::from(error)
                    )
                );
            }
        }
    }

    Ok(())
}

fn remove_registry_set(registry_key: HKEY, name: &str) -> Result<(), DirectoryIOPathRegistrationError> {
    let sub_key = "";
    let sub_key_byte = sub_key.as_bytes();
    let sub_key_cstr = PCSTR::from_raw(sub_key_byte.as_ptr());

    let value_key = format!("{}\0", name);
    let value_key_byte = value_key.as_bytes();
    let value_key_cstr = PCSTR::from_raw(value_key_byte.as_ptr());

    unsafe {
        match RegDeleteKeyValueA(registry_key, sub_key_cstr, value_key_cstr).ok() {
            Ok(_) => {}
            Err(error) => {
                return Err(
                    DirectoryIOPathRegistrationError::WindowsError(
                        WindowsError::from(error)
                    )
                );
            }
        }
    }

    Ok(())
}

fn remove_set_value(registry_key: HKEY, name: &str) -> Result<(), DirectoryIOPathRegistrationError> {
    let value_key = format!("{}\0", name);
    let value_key_byte = value_key.as_bytes();
    let value_key_cstr = PCSTR::from_raw(value_key_byte.as_ptr());

    unsafe {
        match RegDeleteValueA(registry_key, value_key_cstr).ok() {
            Ok(_) => {}
            Err(error) => {
                return Err(
                    DirectoryIOPathRegistrationError::WindowsError(
                        WindowsError::from(error)
                    )
                );
            }
        }
    }

    Ok(())
}

fn invalidate_registry_key(registry_key: &mut HKEY) -> Result<(), DirectoryIOPathRegistrationError> {
    unsafe {
        match RegCloseKey(registry_key.clone()).ok() {
            Ok(_) => {}
            Err(error) => {
                return Err(
                    DirectoryIOPathRegistrationError::WindowsError(
                        WindowsError::from(error)
                    )
                );
            }
        }
    }

    registry_key.0 = -1;

    Ok(())
}