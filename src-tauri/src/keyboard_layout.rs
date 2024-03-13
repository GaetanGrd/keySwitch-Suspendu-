extern crate winapi;

use std::{ffi::CString, ptr::null_mut};

use winapi::um::{winnt::KEY_READ, winreg::{RegOpenKeyExA, RegQueryValueExA, HKEY_LOCAL_MACHINE}, winuser::{GetForegroundWindow, GetKeyboardLayout, GetKeyboardLayoutNameW, GetWindowThreadProcessId, KL_NAMELENGTH}};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct KeyboardLayout{
    pub hkl: u32,
    pub lid: i8,
    pub lang: String,
    pub country: String
}

impl KeyboardLayout {
    pub fn new() -> Self {
        let hkl = Self::get_hkl();
        let lang = Self::loword(hkl);
        let country = Self::get_country_from_lang(lang);
        

        let mut buffer: [u16; KL_NAMELENGTH as usize] = [0; KL_NAMELENGTH as usize];
        unsafe { GetKeyboardLayoutNameW(buffer.as_mut_ptr()) };
        
        let name_id = String::from_utf16_lossy(&buffer);

        
        let lid = Self::hiword(hkl) as i8;
        
        KeyboardLayout {
            hkl,
            lid,
            lang : lang.to_string(),
            country
        }
    }
    fn hiword(int:u32) -> u16 {
        return (int >> 16 & 0xFFFF).try_into().unwrap();
    }
    fn loword(int:u32) -> u16 {
        return (int & 0xFFFF).try_into().unwrap();
    }
    fn get_hkl() -> u32 {
        let curr_windows = unsafe { GetForegroundWindow() };
        let thread_id = unsafe { GetWindowThreadProcessId(curr_windows, null_mut()) };
        let temp_hkl = unsafe { GetKeyboardLayout(thread_id) };
        temp_hkl as u32
    }
    fn get_country_from_lang(lang: u16) -> String {
        unsafe {
            let hklm = HKEY_LOCAL_MACHINE;
            let sub_key = CString::new("SYSTEM\\CurrentControlSet\\Control\\Keyboard Layout\\DosKeybCodes").unwrap();
            let mut h_key = null_mut();
    
            if RegOpenKeyExA(hklm, sub_key.as_ptr(), 0, KEY_READ, &mut h_key) != 0 {
                return "Failed to open registry key".to_string();
            }
    
            let mut country_code = vec![0u8; 256];
            let mut len = country_code.len() as u32;
    
            let value_name = CString::new(format!("{:08X}", lang)).unwrap();
    
            if RegQueryValueExA(h_key, value_name.as_ptr(), null_mut(), null_mut(), country_code.as_mut_ptr(), &mut len) != 0 {
                return "Failed to query registry value".to_string();
            }
    
            String::from_utf8(country_code.into_iter().take_while(|&c| c != 0).collect()).unwrap_or_else(|_| "Invalid UTF-8 sequence".to_string())
        }
        
    }

    pub fn get_installed_keyboard_layouts() {
    }
}

pub fn set_keyboard_layout(layout_id: &str) {  
}

