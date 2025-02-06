use log::error;
use md5::{Digest, Md5};
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::Foundation::{
    CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, ERROR_SUCCESS, HANDLE,
};
use windows::Win32::System::Threading::CreateMutexW;

pub fn release_mutex(mutex: Option<HANDLE>) {
    unsafe {
        CloseHandle(mutex.unwrap()).expect("Unable to close mutex");
    }
}

pub fn create_mutex() -> Option<HANDLE> {
    let current_exe = std::env::current_exe().unwrap();
    let mut hasher = Md5::new();
    md5::Digest::update(&mut hasher, current_exe.to_str().unwrap());
    let current_exe_hashed = format!("{:x}", hasher.finalize());
    let mutex_name = format!("Global\\{}", current_exe_hashed);
    let h_mutex_name = HSTRING::from(mutex_name);
    let p_mutex_name = PCWSTR::from_raw(h_mutex_name.as_ptr());

    let mutex = unsafe {
        let mutex = CreateMutexW(None, true, p_mutex_name);
        let error = GetLastError();

        if error == ERROR_SUCCESS {
            Some(mutex.unwrap())
        } else if error == ERROR_ALREADY_EXISTS {
            error!("The application is already running: {:?}", error);
            None
        } else {
            error!("Error creating mutex: {:?}", error);
            None
        }
    };

    mutex
}
