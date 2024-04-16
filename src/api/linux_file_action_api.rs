use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use axum::{Json, Router, routing::{get, post}};
use axum::routing::put;
use serde::Deserialize;

use crate::node_exporter::fileutils::fileinfo::{FileInfo, get_file_contents, get_file_contents_by_line};
use crate::hand::host::file_operation::{copy_file, create_file, delete_file, mkdir, move_file};

pub fn linux_file_action_api() -> Router{
    Router::new()
        .route("/uid", post(get_file_uid_handler))
        .route("/gid", post(get_file_gid_handler))
        .route("/id", post(get_file_id_handler))
        .route("/createTime", get(get_create_time_handler))
        .route("/updateTime", get(get_update_time_handler))
        .route("/contents", post(get_file_contents_handler))
        .route("/contentsButBig", post(get_file_contents_by_line_handler))
        .route("/create", put(create_handler))
        .route("/createDir", put(create_dir_handler))
        .route("/delete", put(delete_handler))
        .route("/copy-file", put(copy_file_handler))
        .route("/move-file", put(move_file_handler))
        .route("/mkdir", put(mkdir_handler))
}

#[derive(Deserialize)]
struct FilePathRequest {
    path: PathBuf,
    dest: PathBuf
}

async fn init_file_info(path: &Path) -> io::Result<FileInfo> {
    FileInfo::from(path)
}

async fn get_file_uid_handler(request: Json<FilePathRequest>) -> Json<Option<i16>> {
    let path = &request.path;

    match init_file_info(path).await {
        Ok(metadata) => {
            let (user_id, _) = metadata.user_id()?;
            Json(Some(user_id as i16))
        }
        Err(_e) => {
            Json(None)
        }
    }
}

async fn get_file_gid_handler(request: Json<FilePathRequest>) -> Json<Option<i16>> {
    let path = &request.path;

    match init_file_info(path).await {
        Ok(metadata) => {
            let (_, gid) = metadata.user_id()?;
            Json(Some(gid as i16))
        }
        Err(_e) => {
            Json(None)
        }
    }
}

async fn get_file_id_handler(request: Json<FilePathRequest>) -> Json<Option<(i16, i16)>>{
    let path = &request.path;

    match init_file_info(path).await {
        Ok(metadata) => {
            let (uid, gid) = metadata.user_id()?;
            Json(Some((uid as i16, gid as i16)))
        }
        Err(_e) => {
            Json(None)
        }
    }
}

async fn get_create_time_handler(request: Json<FilePathRequest>) -> io::Result<SystemTime>
{
    let path = &request.path;
    Ok(init_file_info(path).await?.create_time().unwrap_or(SystemTime::UNIX_EPOCH))
}

async fn get_update_time_handler(request: Json<FilePathRequest>) -> io::Result<SystemTime> {
    let path = &request.path;
    Ok(init_file_info(path).await?.update_time().unwrap_or(SystemTime::UNIX_EPOCH))
}

async fn get_file_contents_handler(request: Json<FilePathRequest>)
    -> Result<Json<io::Result<String>>, io::Error>
{
    let path = &request.path;
    Ok(Json(get_file_contents(path).await))
}

async fn get_file_contents_by_line_handler(request: Json<FilePathRequest>)
    -> Result<Json<io::Result<BufReader<File>>>, io::Error>
{
    let path = &request.path;
    Ok(Json(get_file_contents_by_line(path).await))
}

fn create_handler(request: Json<FilePathRequest>) -> Result<io::Result<()>, io::Error> {
    let path = &request.path;
    Ok(create_file(path))
}

fn create_dir_handler(request: Json<FilePathRequest>) -> Result<io::Result<()>, io::Error> {
    let path = &request.path;
    Ok(mkdir(path))
}

fn delete_handler(request: Json<FilePathRequest>) -> Result<io::Result<()>, io::Error> {
    let path = &request.path;
    Ok(delete_file(path))
}

fn copy_file_handler(request: Json<FilePathRequest>) -> Result<io::Result<()>, io::Error> {
    let path = &request.path;
    let dest = &request.dest;
    Ok(copy_file(path, dest))
}

fn move_file_handler(request: Json<FilePathRequest>) -> Result<io::Result<()>, io::Error> {
    let path = &request.path;
    let dest = &request.dest;
    Ok(move_file(path, dest))
}

fn mkdir_handler(request: Json<FilePathRequest>) -> Result<io::Result<()>, io::Error> {
    let path = &request.path;
    Ok(mkdir(path))
}