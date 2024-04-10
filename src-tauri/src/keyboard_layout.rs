extern crate winapi;

use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

use serde::{Deserialize, Serialize};

use winapi::shared::minwindef::{HIWORD, LOWORD};
use winapi::shared::{
    minwindef::HKL,
    winerror::ERROR_SUCCESS,
};

use winapi::um::{
    winuser::{ GetKeyboardLayout, KL_NAMELENGTH, GetKeyboardLayoutNameA},
};


use std::io;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;
use winreg::enums::HKEY_LOCAL_MACHINE;
use winapi::um::winuser as user32;
use winreg::enums::*;

#[derive(Serialize, Deserialize)]
pub struct KeyboardLayoutInfo {
    #[serde(serialize_with = "serialize_hkl", deserialize_with = "deserialize_hkl")]
    pub hkl: HKL,
}
impl KeyboardLayoutInfo {
    pub fn new() -> Option<Self> {
        let hkl = Self::get_hkl();
        println!("hkl: {:?}", hkl);
        println!("Primary language: {:?}", Self::get_primary_lang(hkl).unwrap());
        println!("Sub language: {:?}", Self::get_sub_lang(hkl).unwrap());
    
        Some(Self {
            hkl,
        })
    }
    
    fn get_hkl() -> HKL {
        let hwnd = unsafe { user32::GetForegroundWindow() };
        let thread_id = unsafe { user32::GetWindowThreadProcessId(hwnd, ptr::null_mut()) };
        unsafe { GetKeyboardLayout(thread_id) }
    }

    pub fn get_primary_lang(hkl: HKL) -> Result<String, Box<dyn std::error::Error>> {
        let lang_id = LOWORD(hkl as u32);

        if lang_id == 0 {
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Failed to get primary language")));
        }

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let key = hklm.open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\Keyboard Layout\\DosKeybCodes", KEY_READ).unwrap();
        let valeur: String = key.get_value(format!("{:08X}", lang_id)).unwrap();
        Ok(valeur)
    }

    pub fn get_sub_lang(hkl: HKL) -> Result<String, Box<dyn std::error::Error>> {
        let sub_lang_id = HIWORD(hkl as u32);

        if sub_lang_id == 0 {
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Failed to get sub language")));
        }

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let key = hklm.open_subkey_with_flags("SYSTEM\\CurrentControlSet\\Control\\Keyboard Layout\\DosKeybCodes", KEY_READ).unwrap();
        let valeur: String = key.get_value(format!("{:08X}", sub_lang_id)).unwrap();
        Ok(valeur)
    }
}
// Fonction personnalisée pour la sérialisation de HKL
fn serialize_hkl<S>(hkl: &HKL, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let hkl_as_ptr: *const c_void = *hkl as *const c_void; // Convertit HKL en pointeur brut    
    let hkl_as_usize = hkl_as_ptr as usize; // Convertit le pointeur brut en usize
    serializer.serialize_u64(hkl_as_usize as u64) // Sérialise comme un entier
}

// Fonction personnalisée pour la désérialisation de HKL
fn deserialize_hkl<'de, D>(deserializer: D) -> Result<HKL, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let hkl_as_usize = u64::deserialize(deserializer)? as usize; // Désérialise comme un entier puis convertit en usize
    let hkl_as_ptr = hkl_as_usize as *mut c_void; // Convertit usize en pointeur brut
    Ok(hkl_as_ptr as HKL) // Convertit le pointeur brut en HKL
}