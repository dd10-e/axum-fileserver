use axum::{
    body::{StreamBody, Body},
    http::{header, StatusCode, Request, Response},
    extract::{self, ContentLengthLimit, Multipart},
    response::{Headers, Html, IntoResponse},
    routing::{get, post, get_service},
    Json, Router
};

use tokio_util::io::ReaderStream;
use humansize::{file_size_opts::BINARY, FileSize};
use serde::{Deserializer, Serialize, Deserialize, Serializer};
use std::{env, net::SocketAddr, io::Write, os::unix::prelude::MetadataExt};
use cap_std::{
    ambient_authority,
    fs::{Dir, File, OpenOptions},
    path::Path,
};
use uuid::Uuid;
use time::{Instant, OffsetDateTime};
use tower_http::{
    services::{fs::ServeFileSystemResponseBody, ServeDir, ServeFile},
    cors::{self, CorsLayer, Origin}
};
use tower_service::Service;

const HOST_FOLDER_BASE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/", "uploads");

#[derive(Deserialize)]
pub struct EditFileInput {
    pub file_content: String,
}

pub async fn edit_file(extract::Path((uuid, path)): extract::Path<(Uuid, String)>, Json(input): Json<EditFileInput>) -> impl IntoResponse  {
    let project_path = format!("{}/{}", HOST_FOLDER_BASE, uuid);
    let file_path = format!("{}{}", ".", path);
    let cwd = Dir::open_ambient_dir(project_path.clone(), ambient_authority()).expect("ambient authority not authorized ");

    let mut file = cwd.create(file_path.clone()).expect("Can't create the file");
    file.write(input.file_content.as_bytes()).expect("Can't write the file");

    (StatusCode::OK, format!("Document edited: {}", file_path))
}


#[derive(Serialize, Deserialize)]
pub struct DeleteDocumentResponse {
    path: String,
}

pub async fn delete(extract::Path((uuid, path)): extract::Path<(Uuid, String)>,) -> impl IntoResponse  {
    let project_path = format!("{}/{}", HOST_FOLDER_BASE, uuid);
    let current_project_relative_path = format!("{}{}", ".", path);
    let cwd = Dir::open_ambient_dir(project_path.clone(), ambient_authority()).expect("ambient authority not authorized ");

    println!("{}", current_project_relative_path);
    cwd.remove_file(current_project_relative_path.clone()).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    let response = DeleteDocumentResponse {
        path: path,
    };

    (StatusCode::OK, Json(response))
}

pub async fn serve_file(extract::Path((uuid, path)): extract::Path<(Uuid, String)>, req: Request<Body>) -> Result<Response<ServeFileSystemResponseBody>, String> {
    let project_path = format!("{}/{}", HOST_FOLDER_BASE, uuid);
    let current_project_path = format!("{}{}", project_path, path);

    ServeFile::new(current_project_path)
        .call(req)
        .await
        .map_err(|err| err.to_string())
}

pub async fn download_file(extract::Path((uuid, path)): extract::Path<(Uuid, String)>) -> impl IntoResponse  {
    println!("{:?}", path);
    let project_path = format!("{}/{}", HOST_FOLDER_BASE, uuid);
    let current_project_path = format!("{}{}", project_path, path);

    // `File` implements `AsyncRead`
    let path = Path::new(&current_project_path);
    let file = match tokio::fs::File::open(current_project_path.clone()).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);
    
    let headers = Headers([
        (header::CONTENT_TYPE, format!("{:?}; charset=utf-8", path.metadata().expect("metadata call failed").file_type())),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{:?}\"", path.file_name().expect("file_name call failed")),
        ),
    ]);

    Ok((headers, body))
}

pub fn check_if_file_already_exists(path: String, file_name: String, dir: &Dir) -> String {
    let mut name = file_name.clone();
    let filename_path = format!("{}/{}", path, name);

    if dir.exists(filename_path.clone()) {
        name = format!("{} {}", "Copie - ", file_name.clone());

        return check_if_file_already_exists(path, name.clone(), dir);
    } else {
        return name;
    }
}

pub async fn upload_file(
    extract::Path((uuid, path)): extract::Path<(Uuid, String)>,
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            250 * 1024 * 1024 /* 250mb */
        },
    >,
) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let project_path = format!("{}/{}", HOST_FOLDER_BASE, uuid);
        let current_project_relative_path = format!("{}{}", ".", path);
        let cwd = Dir::open_ambient_dir(project_path.clone(), ambient_authority()).expect("ambient authority not authorized ");
        
        let name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let unique_name = check_if_file_already_exists(current_project_relative_path.clone(), name.clone(), &cwd);
        let path_with_unique_name = format!("{}/{}", current_project_relative_path.clone(), unique_name);


        println!("{}", path_with_unique_name);
        // TODO: Handle request where folder doesn't exist
        // Handle errors properly and don't crash
        let mut file = cwd.create(path_with_unique_name.clone()).expect("!");
        file.write(&data).expect("can't write data on file");

        let response = format!("Length of `{}` is {} bytes.", name, data.len());

        return (StatusCode::OK, response);
    }

    (StatusCode::OK, "No file to upload".to_string())
}


#[derive(Deserialize)]
pub struct InputCreateFolder {
    pub folder_path: String,
}

pub async fn create_folder(
    extract::Path((uuid, path)): extract::Path<(Uuid, String)>,
    Json(input): Json<InputCreateFolder>
) -> impl IntoResponse {
    let project_path = format!("{}/{}", HOST_FOLDER_BASE, uuid);
    let current_project_relative_path = format!("{}{}", ".", path);
    let cwd = Dir::open_ambient_dir(project_path.clone(), ambient_authority()).expect("ambient authority not authorized ");

    let new_folder_path = format!("{}/{}", current_project_relative_path, input.folder_path);
    cwd.create_dir_all(new_folder_path.clone()).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    let response = serde_json::json!({
        "response": "ok",
    });

    let response = Json(response);
    
    (StatusCode::OK, response)
}

#[derive(Serialize, Deserialize)]
pub struct DirEntry {
    name: String,
    path_type: String,
    size: String,
    creation_date: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Breadcrump {
    name: String,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Readme {
    path: String,
    exist: bool,
}

pub async fn list_documents(extract::Path((uuid, path)): extract::Path<(Uuid, String)>) -> impl IntoResponse {
    let project_path = format!("{}/{}", HOST_FOLDER_BASE, uuid);
    let current_project_relative_path = format!("{}{}", ".", path);
    let current_project_absolute_path = format!("{}{}", project_path, path); // TODO: Check securisation
    let cwd = Dir::open_ambient_dir(project_path.clone(), ambient_authority()).expect("ambient authority not authorized ");


    let breadcrumps = path.split("/").map(|path| {
        Breadcrump {name: path.to_string(), path: path.to_string()}
    }).collect::<Vec<Breadcrump>>();
    let parent = breadcrumps.get(breadcrumps.len() - 2);


    let current_document_metadata = Path::new(&current_project_absolute_path).metadata().expect("can't get metadata");

    let current_document = match (current_document_metadata.file_type().is_symlink(), current_document_metadata.file_type().is_dir()) {
        (true, true) => "SymlinkDir".to_string(),
        (false, true) => "Dir".to_string(),
        (true, false) => "SymlinkFile".to_string(),
        (false, false) => "File".to_string(),
    };

    let mut documents = Vec::new();
    match cwd.read_dir(current_project_relative_path.clone()) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                let path = path.unwrap();
                let path_metadata = path.metadata().expect("can't get file metadata");

                let file_type = match (path_metadata.file_type().is_symlink(), path_metadata.file_type().is_dir()) {
                    (true, true) => "SymlinkDir".to_string(),
                    (false, true) => "Dir".to_string(),
                    (true, false) => "SymlinkFile".to_string(),
                    (false, false) => "File".to_string(),
                };

                // let filetype = Path::new("index.jpg");
                let entry = DirEntry {
                    name: path.file_name().into_string().expect("file name access failed"),
                    size: path_metadata.size().file_size(BINARY).unwrap(),
                    path_type: file_type,

                    // TODO: wait https://github.com/bytecodealliance/cap-std/blob/de7442a2fb56550d380d26a9a6c7867a00bb3a21/cap-primitives/src/fs/metadata.rs#L51
                    // for using created date
                    creation_date: OffsetDateTime::from(path_metadata.accessed().expect("created time access failed").into_std()).unix_timestamp(),
                };

                documents.push(entry);
            }
        }
    }

    // Sort by directories first
    documents.sort_by_key(|e| e.path_type != "Dir");

    // let parent = breadcrumps.get(breadcrumps.len());


    let readme_path = "Index.html";
    let readme_exist = cwd.exists(format!("{}{}", current_project_relative_path.clone(), readme_path));

    let response = serde_json::json!({
        "current_document": current_document,
        "documents": documents,
        "breadcrumps": breadcrumps,
        "parent": parent,
        "readme": Readme {
            path: readme_path.to_string(),
            exist: readme_exist,
        },
    });

    let response = Json(response);
    
    (StatusCode::OK, response)
}