pub mod response_helper {
  use std::net::TcpStream;

  use crate::response::response::Response;

  pub fn ok(stream: TcpStream, content: String) {
    send_response(stream, 200, "Bad Request", content);
  }

  pub fn bad_request(stream: TcpStream) {
    send_response(stream, 400, "Bad Request", String::from(""));
  }

  pub fn not_found(stream: TcpStream) {
    send_response(stream, 404, "Not Found", String::from(""));
  }

  pub fn internal_server_error(stream: TcpStream) {
    send_response(stream, 500, "Internal Server Error", String::from(""));
  }

  pub fn send_response(stream: TcpStream, status_code: u32, status_message: &str, content: String) {
    let mut response = Response::new();

    response.status_code = status_code;
    response.status_message = String::from(status_message);

    response.set_content(content);

    response.send(stream);
  }
}
