#[cfg(windows)] extern crate winapi;
use std::io::Error;

use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS, Process32First, PROCESSENTRY32};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::winnt::{HANDLE};
use winapi::shared::minwindef::{TRUE, MAX_PATH};

// Taking a Snapshot and Viewing Processes
// https://docs.microsoft.com/en-us/windows/win32/toolhelp/taking-a-snapshot-and-viewing-processes

#[cfg(windows)]
fn get_process_list() -> Result<(), Error> {

    let snapshot_handle: HANDLE = unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot
        CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
    };
    if snapshot_handle == INVALID_HANDLE_VALUE {
        Err(Error::last_os_error())
    } else {
        let mut process: PROCESSENTRY32 = unsafe { std::mem::zeroed() };
        // https://stackoverflow.com/questions/29346365/process32first-is-not-returning-true-even-if-process-is-running
        // > Before calling the Process32First function, set this member to sizeof(PROCESSENTRY32).
        //   If you do not initialize dwSize, Process32First fails.
        process.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        let found = unsafe {
            Process32First(snapshot_handle, &mut process)
        };

        if found == TRUE {
            let name = process_name(&process);
            println!("name: {:?}\nid: {:?}", name, process.th32ProcessID);
            unsafe {
                CloseHandle(snapshot_handle);
            }
            Ok(())
        } else {
            unsafe {
                CloseHandle(snapshot_handle);
            }
            Err(Error::last_os_error())
        }
    }
}
#[cfg(not(windows))]
fn get_process_list(name: &str) -> Result<(), Error> {
    println!("can't find process named '{}'", name);
    Ok(())
}

fn process_name(process: &PROCESSENTRY32) -> &str {
    let exe_file = &process.szExeFile;
    let ptr = exe_file as *const i8 as *const u8;
    let len = exe_file.iter().position(|&ch| ch == 0).unwrap_or(MAX_PATH);
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    std::str::from_utf8(slice).unwrap()
}

fn main() {
    get_process_list().unwrap();
}