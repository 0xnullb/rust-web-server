extern crate chrono;

mod response;
mod response_helper;

use std::env;
use std::fs;
use std::io::{ErrorKind,Read};
use std::net::{TcpListener, TcpStream};
use std::process;

use chrono::Utc;

use response_helper::response_helper::{ok,bad_request,not_found,internal_server_error};

fn main() {
    let (host, port) = get_host_and_port();
    let result = TcpListener::bind(format!("{}:{}", host, port));
    let listener;

    match result {
        Ok(tcp_listener) => {
            listener = tcp_listener;

            println!("Server listening on {}:{}", host, port);
        },
        Err(e) => {
            println!("E: Failed to bind to {}:{} - {}", host, port, e);

            process::exit(1);
        }
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream)
            },
            Err(e) => {
                println!("E: problem with incoming connection - {}", e);
            }
        }
    }
}

// Reads a host:port pair from the command line, splits it by a ":",
// and returns the host and port as two separate variables
fn get_host_and_port() -> (String, String) {
    let args: Vec<String> = env::args().collect();

    // Make sure we got a host and port to listen on
    if args.len() < 2 {
        println!("E: The program never got enough arguments");
        println!("./main <host:port>");

        process::exit(1);
    }

    let host_port_pair = &args[1];

    if !host_port_pair.contains(":") {
        println!("E: host port pair should be <host:port>, missing port");

        process::exit(1);
    }

    let host_port_split: Vec<&str> = host_port_pair.split(":").collect();

    (host_port_split[0].to_string(), host_port_split[1].to_string())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer);

    let data = std::str::from_utf8(&mut buffer).unwrap();
    let lines = data.split("\r\n").collect::<Vec<&str>>();
    let first_line_split = lines[0].split_whitespace().collect::<Vec<&str>>();

    if data.len() == 0 || first_line_split.len() != 3 {
        bad_request(stream);

        println!("Malformed HTTP request received");

        return;
    }

    let method = first_line_split[0];
    let path = first_line_split[1];
    let version = first_line_split[2];
    
    let filename = get_filename_from_path(path);
    let content;

    match fs::read_to_string(&filename) {
        Ok(file_contents) => {
            content = file_contents;
        },
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    not_found(stream);
                },
                _ => {
                    internal_server_error(stream);
                }
            }

            print_log(method, path, version, &0);

            return;
        }
    }

    let content_length = content.len();

    ok(stream, content);

    print_log(method, path, version, &content_length);
}

fn print_log(method: &str, path: &str, version: &str, content_length: &usize) {
    let now = Utc::now();
    let formatted_time = now.format("%a %b %e %T %Y");

    println!("[{}] {} {} {} {}", formatted_time, method, path, version, content_length);
}

fn get_filename_from_path(path: &str) -> String {
    let filename;

    if path == "/" {
        filename = "htdocs/index.html".to_string();
    } else if path.ends_with("/") {
        filename = "htdocs".to_string() + path + "index.html";
    } else if !path.ends_with(".html") {
        filename = "htdocs".to_string() + path + "/index.html"
    } else {
        filename = "htdocs".to_string() + path
    }

    filename
}
