use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

use http_download::download_to_file;

fn spawn_single_response_server(response: &'static [u8]) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();

    thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();

        let mut request = Vec::new();
        let mut buffer = [0_u8; 1024];
        while !request.windows(4).any(|window| window == b"\r\n\r\n") {
            let size = stream.read(&mut buffer).unwrap();
            if size == 0 {
                break;
            }
            request.extend_from_slice(&buffer[..size]);
        }

        stream.write_all(response).unwrap();
    });

    format!("http://{address}/resource")
}

#[test]
fn saves_response_body_to_destination_file() {
    let url = spawn_single_response_server(
        b"HTTP/1.1 200 OK\r\nContent-Length: 12\r\nConnection: close\r\n\r\nhello, world",
    );
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("resource.bin");

    download_to_file(&url, &path).unwrap();

    assert_eq!(fs::read(&path).unwrap(), b"hello, world");
}

#[test]
fn replaces_existing_destination_file() {
    let url = spawn_single_response_server(
        b"HTTP/1.1 200 OK\r\nContent-Length: 3\r\nConnection: close\r\n\r\nnew",
    );
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("resource.bin");
    fs::write(&path, b"old").unwrap();

    download_to_file(&url, &path).unwrap();

    assert_eq!(fs::read(&path).unwrap(), b"new");
}

#[test]
fn leaves_only_destination_file_in_directory() {
    let url = spawn_single_response_server(
        b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\nConnection: close\r\n\r\nbody",
    );
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("resource.bin");

    download_to_file(&url, &path).unwrap();

    let entry_count = fs::read_dir(directory.path()).unwrap().count();
    assert_eq!(entry_count, 1);
}

#[test]
fn returns_error_for_error_status_response() {
    let url = spawn_single_response_server(
        b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
    );
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("resource.bin");

    let result = download_to_file(&url, &path);

    assert!(result.is_err());
}

#[test]
fn does_not_create_destination_file_for_error_status_response() {
    let url = spawn_single_response_server(
        b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
    );
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("resource.bin");

    let _ = download_to_file(&url, &path);

    assert!(!path.exists());
}
