use std::{
    env::current_dir,
    ffi::CString,
    fs,
    fs::{File, Metadata},
    io::{self, BufReader},
    path::{Path, PathBuf},
};
use std::time::SystemTime;

use libc::{c_short, stat};
use walkdir::WalkDir;

// FileInfo 结构体包含了一个文件或目录的路径和元数据
pub struct FileInfo {
    path: PathBuf,  // 使用 PathBuf 而不是 &Path，因为 FileInfo 可能会比 path 存活的更久
    metadata: Metadata,
}

impl FileInfo {
    // new 方法用于创建一个新的 Metadata 对象
    pub fn from<P>(path: P) -> io::Result<Self>
        where
            P: AsRef<Path>
    {
        let metadata = fs::metadata(path)?;
        Ok(Self {
            path: path.to_path_buf(),
            metadata,
        })
    }

    // get_file_user 方法用于获取文件的用户uid和gid
    pub fn user_id(&self) -> Result<(c_short, c_short), io::Error> {
        let c_path = CString::new(self.path.as_os_str().as_encoded_bytes())?;
        let mut s: stat = unsafe { std::mem::zeroed() };
        let result = unsafe { libc::stat(c_path.as_ptr(), &mut s) };

        if result != 0 {
            return Err(std::io::Error::last_os_error());
        }

        Ok((unsafe { s.st_uid.try_into()? }, unsafe { s.st_gid.try_into()? }))
    }

    // create_time 方法用于获取文件的创建时间
    pub fn create_time(&self) -> std::io::Error::Result<SystemTime> {
        Ok(self.metadata.created())
    }

    // get_file_update_time 方法用于获取文件的修改时间
    pub fn update_time(&self) -> io::Result<SystemTime> {
        self.metadata.modified()
    }
}

// get_current_directory 获取当前所在目录的路径
fn get_current_directory() -> std::io::Result<std::path::PathBuf> {
    current_dir()
}

// get_file_contents 直接获取文件的全部内容。
pub async fn get_file_contents<P>(filename: P) -> io::Result<String>
    where
        P: AsRef<Path>,
{
    fs::read_to_string(filename)
}

// get_file_contents_by_line 按行获取文件的内容。
pub async fn get_file_contents_by_line<P>(filename: P) -> io::Result<BufReader<File>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file))
}



pub fn get_dir_size<P>(path: P) -> io::Result<u64>
    where
        P: AsRef<Path>
{
    let mut size = 0;
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            size += fs::metadata(entry.path())?.len();
        }
    }
    Ok(size)
}