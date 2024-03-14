extern crate winapi;

use std::env;
use winapi::um::fileapi::GetDiskFreeSpaceW;
use winapi::um::winnt::WCHAR;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

fn main() {
    // 從命令列參數中獲取磁碟區路徑
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("使用方式: {} <磁碟區路徑>", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];

    // 將路徑轉換為Windows所需的UTF-16格式
    let wide_path: Vec<WCHAR> = OsStr::new(path).encode_wide().chain(Some(0).into_iter()).collect();

    // 宣告變數以接收資訊
    let mut sectors_per_cluster = 0;
    let mut bytes_per_sector = 0;
    let mut number_of_free_clusters = 0;
    let mut total_number_of_clusters = 0;

    // 調用GetDiskFreeSpaceW獲取磁碟資訊
    let success = unsafe {
        GetDiskFreeSpaceW(
            wide_path.as_ptr(),
            &mut sectors_per_cluster,
            &mut bytes_per_sector,
            &mut number_of_free_clusters,
            &mut total_number_of_clusters,
        )
    };

    if success == 0 {
        eprintln!("無法獲取磁碟'{}'的資訊", path);
        std::process::exit(1);
    }

    let block_size = sectors_per_cluster * bytes_per_sector;
    println!("磁碟'{}'的預設block size為: {} bytes", path, block_size);
}

