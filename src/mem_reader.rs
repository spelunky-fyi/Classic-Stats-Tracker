use std::io::Cursor;
use std::mem::size_of;
use std::thread::sleep;
use std::time::{Duration, Instant};

use anyhow::anyhow;
use byteorder::{ByteOrder, LittleEndian};
use serde::Serialize;
use tokio::sync::broadcast::Sender;
use winapi::shared::minwindef::{DWORD, HMODULE, LPCVOID, LPVOID, MAX_PATH};
use winapi::shared::ntdef::HANDLE;
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::um::minwinbase::STILL_ACTIVE;
use winapi::um::processthreadsapi::{GetExitCodeProcess, OpenProcess};
use winapi::um::psapi::{EnumProcessModules, GetModuleFileNameExA};
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

use crate::stats::Stats;

static EXE_NAME: &'static [i8; 13] = &[
    'S' as i8, 'p' as i8, 'e' as i8, 'l' as i8, 'u' as i8, 'n' as i8, 'k' as i8, 'y' as i8,
    '.' as i8, 'e' as i8, 'x' as i8, 'e' as i8, '\0' as i8,
];

fn get_spelunky_process() -> Result<HANDLE, anyhow::Error> {
    unsafe {
        let process_snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

        let mut process: PROCESSENTRY32 = PROCESSENTRY32 {
            dwSize: size_of::<PROCESSENTRY32>()
                .try_into()
                .expect("Failed to get size of PROCESSENTRY32"),
            cntUsage: 0,
            th32ProcessID: 0,
            th32DefaultHeapID: 0,
            th32ModuleID: 0,
            cntThreads: 0,
            th32ParentProcessID: 0,
            pcPriClassBase: 0,
            dwFlags: 0,
            szExeFile: [0; MAX_PATH],
        };

        if process_snap == INVALID_HANDLE_VALUE {
            return Err(anyhow!("Failed to get process list handle..."));
        }

        if Process32First(process_snap, &mut process) == 0 {
            return Err(anyhow!("Failed to get first process..."));
        }

        loop {
            if &process.szExeFile[..EXE_NAME.len()] == EXE_NAME {
                CloseHandle(process_snap);

                let process_handle = OpenProcess(
                    PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                    0,
                    process.th32ProcessID,
                );

                if process_handle == winapi::shared::ntdef::NULL {
                    return Err(anyhow!("Failed to open process..."));
                }

                return Ok(process_handle);
            }

            if Process32Next(process_snap, &mut process) == 0 {
                return Err(anyhow!("No process found!"));
            }
        }
    }
}

fn get_base_addr(process: HANDLE) -> anyhow::Result<usize> {
    // Get Module name of EXE
    let mut process_image_filename = [0; MAX_PATH];
    unsafe {
        let result = GetModuleFileNameExA(
            process,
            0 as HMODULE,
            process_image_filename.as_mut_ptr(),
            MAX_PATH as u32,
        );
        if result == 0 {
            return Err(anyhow!("Failed to get process module name..."));
        }
    }

    // Get handles for all modules in process.
    let mut module_handles: [HMODULE; 1024] = [0 as HMODULE; 1024];
    let hmodule_size: usize = size_of::<HMODULE>()
        .try_into()
        .expect("Failed to get size of HMODULE");
    let mut bytes_written = 0;

    unsafe {
        let result = EnumProcessModules(
            process,
            module_handles.as_mut_ptr(),
            size_of::<[HMODULE; 1024]>()
                .try_into()
                .expect("Failed to get size for modules"),
            &mut bytes_written,
        );
        if result == 0 {
            return Err(anyhow!("Failed to enumerate process modules..."));
        }
    }

    let num_modules = bytes_written as usize / hmodule_size;

    // Enumerate Modules to find handle for EXE module
    for idx in 0..num_modules {
        let mut module_filename = [0; MAX_PATH];
        unsafe {
            let result = GetModuleFileNameExA(
                process,
                module_handles[idx],
                module_filename.as_mut_ptr(),
                MAX_PATH as u32,
            );
            if result == 0 {
                continue;
            }
        }

        if module_filename != process_image_filename {
            continue;
        }

        // Found the exe module base address
        return Ok(module_handles[idx] as usize);
    }

    Err(anyhow!("Failed to find module..."))
}

pub(crate) fn read_uptr32_t(process: HANDLE, addr: usize) -> anyhow::Result<u32> {
    let mut buf: [u8; 4] = [0; 4];
    let mut bytes_read = 0;
    unsafe {
        if ReadProcessMemory(
            process,
            addr as LPCVOID,
            buf.as_mut_ptr() as LPVOID,
            4,
            &mut bytes_read,
        ) == 0
        {
            return Err(anyhow!("Failed to read memory"));
        }
    };
    Ok(LittleEndian::read_u32(&buf))
}

pub(crate) fn read_n_bytes(
    process: HANDLE,
    addr: usize,
    num_bytes: usize,
) -> anyhow::Result<Cursor<Vec<u8>>> {
    let mut buf: Vec<u8> = Vec::with_capacity(num_bytes);
    let mut bytes_read = 0;
    unsafe {
        if ReadProcessMemory(
            process,
            addr as LPCVOID,
            buf.as_mut_ptr() as LPVOID,
            num_bytes,
            &mut bytes_read,
        ) == 0
        {
            return Err(anyhow!("Failed to read memory"));
        }
        buf.set_len(bytes_read);
    };

    Ok(Cursor::new(buf))
}

#[derive(Clone, Serialize)]
#[serde(tag = "type")]
pub enum TrackerMessage {
    Connecting,
    Payload { stats: Stats },
}

pub fn run_forever(tx: Sender<TrackerMessage>) -> anyhow::Result<()> {
    let mut connecting = false;
    loop {
        if !connecting {
            println!("Looking for Spelunky.exe (Classic)...");
            connecting = true;
        }

        if let Err(_err) = tx.send(TrackerMessage::Connecting) {
            //eprintln!("Message passing backed up... {}", err);
        }

        // Try to open process
        let base_addr;
        let process = match get_spelunky_process() {
            Ok(process) => {
                base_addr = match get_base_addr(process) {
                    Ok(base_addr) => base_addr,
                    Err(_err) => {
                        // eprintln!("Failed to get base addr: {:?}", err);
                        unsafe {
                            CloseHandle(process);
                        };
                        sleep(Duration::from_millis(1000));
                        continue;
                    }
                };
                process
            }
            Err(_) => {
                sleep(Duration::from_millis(1000));
                continue;
            }
        };

        connecting = false;
        println!("Connected!");
        let mut stats_state = Stats::default();
        let mut last_update = Instant::now();
        loop {
            let mut exit_code: DWORD = 0;
            unsafe { GetExitCodeProcess(process, &mut exit_code) };
            if exit_code != STILL_ACTIVE {
                eprintln!("Process went away...");
                sleep(Duration::from_millis(1000));
                break;
            }

            let new_stats_state = match Stats::from_memory(process, base_addr) {
                Ok(stats_state) => stats_state,
                Err(_err) => {
                    // eprintln!("Failed to get base addr: {:?}", err);
                    // unsafe {
                    //     CloseHandle(process);
                    // };
                    sleep(Duration::from_millis(1000));
                    continue;
                }
            };

            if stats_state != new_stats_state
                || Instant::now() - last_update > Duration::from_secs(1)
            {
                stats_state = new_stats_state;
                last_update = Instant::now();
                if let Err(_err) = tx.send(TrackerMessage::Payload {
                    stats: stats_state.clone(),
                }) {
                    //eprintln!("Message passing backed up... {}", err);
                }
            }

            sleep(Duration::from_millis(100));
        }
    }
}
