use std::{fs, io};
use std::env::current_dir;
use std::fs::{create_dir, File};
use std::path::Path;

// delete_file 删除指定的文件
pub fn delete_file<P>(path: P) -> io::Result<()>
    where
        P: AsRef<Path>
{
    if path.exists() {
        fs::remove_file(path)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }
}

// create_file 在当前路径下创建文件
pub fn create_file<P>(filename: P) -> io::Result<()>
    where
        P: AsRef<Path>,
{
    let pwd = current_dir()?;
    let file_path = pwd.join(filename);
    if !file_path.exists() {
        File::create(file_path)?;
    }
    Ok(())
}

// copy_file 复制文件
pub fn copy_file(src: &Path, dest: &Path) -> io::Result<()> {
    if !dest.exists() {
        fs::copy(src, dest)?;
    }
    Ok(())
}

// move_file 移动并改名文件
pub fn move_file(src: &Path, dest: &Path) -> io::Result<()> {
    if !dest.exists() {
        fs::rename(src, dest)?;
    }
    Ok(())
}

pub fn mkdir<P>(path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
{
    if !path.exists() {
        create_dir(path)?;
    }
    Ok(())
}

fn create_tar(tar_path: &str, file_path: &str) {
    // TODO
}

fn extract_tar(tar_path: &str, dest_path: &str)  {
    // TODO
}

fn create_zip(src_path: &Path, dest_path: &Path) {
    // TODO
}

fn extract_zip(src_path: &Path, dest_path: &Path){
    // TODO
}

fn upload_file(filepath: &Path, remote_path: &str) {
    // TODO
}

fn download() {
    // TODO
}