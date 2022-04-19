pub mod response {
  use std::collections::HashMap;
  use std::io::Write;
  use std::net::{Shutdown,TcpStream};

  pub struct Response {
    headers: HashMap<String, String>,
    content: String,
    pub status_code: u32,
    pub status_message: String,
  }

  impl Response {
    pub fn new() -> Response {
      let mut headers = HashMap::new();

      headers.insert(
        String::from("Content-Type"),
        String::from("text/html")
      );

      Response {
        headers,
        content: String::new(),
        status_code: 200,
        status_message: String::from("OK")
      }
    }

    pub fn set_content(&mut self, content: String) {
      self.content = content.clone();
    }

    fn build_response_string(&self) -> String {
      let mut response_string = String::from("HTTP/1.0");

      response_string.push_str(self.status_code.to_string().as_str());

      for (k, v) in &self.headers {
        let header = k.to_string() + ": " + v;

        response_string.push_str((header + "\r\n").as_str());
      }

      response_string.push_str("\r\n");

      response_string.push_str(self.content.as_str());
      response_string.push_str("\r\n");

      response_string
    }

    pub fn send(&self, mut stream: TcpStream) {
      let response_string = self.build_response_string();

      stream.write(response_string.as_bytes()).unwrap();
      stream.shutdown(Shutdown::Both).unwrap();
    }
  }
}
