pub mod app;
pub mod engine;
pub mod ui;

#[cfg(target_os = "windows")]
pub fn trim_memory() {
    unsafe {
        unsafe extern "system" {
            fn GetCurrentProcess() -> isize;
            fn K32EmptyWorkingSet(h_process: isize) -> i32;
            fn SetProcessWorkingSetSize(h_process: isize, min: usize, max: usize) -> i32;
        }
        let handle = GetCurrentProcess();
        K32EmptyWorkingSet(handle);
        SetProcessWorkingSetSize(handle, usize::MAX, usize::MAX);
    }
}

#[cfg(not(target_os = "windows"))]
pub fn trim_memory() {}
