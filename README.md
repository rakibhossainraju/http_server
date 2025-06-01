# http\_server

A simple, educational HTTP server implemented from scratch in Rust. This project demonstrates low-level networking by using raw `TcpListener`/`TcpStream` instead of high-level frameworks (like Hyper or Actix). The goal is to parse HTTP requests (start-line and query string) manually and log the results, giving Rust developers a hands-on look at systems-level networking and protocol parsing.

## Features

* **Raw TCP Listening:** Listens on a specified address and port (e.g. `127.0.0.1:8080`) using `std::net::TcpListener`. Connections are handled in a loop with `listener.accept()`, reading raw bytes from the stream.
* **HTTP Method Parsing:** Defines an enum for standard HTTP methods. For example:

  ```rust
  pub enum Method {
      GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS, TRACE, CONNECT,
  }
  ```

  This covers the common verbs.
* **Request Struct:** Captures the start-line components. A `Request` struct holds fields like `path`, `method`, and optional `query_string`. These fields are made public for easy access (e.g. `request.path`, `request.method`).
* **Request Parsing:** Implements `TryFrom<&[u8]> for Request` to parse a raw byte buffer into a `Request`. It splits the request line by whitespace/CRLF using a helper `get_next_word` function. For example, the code looks up the next token in the buffer and converts it to a `Method`. Any parsing errors produce a custom `ParseError`.
* **Error Handling:** Defines a `ParseError` enum for various failure cases (invalid UTF-8, unsupported protocol, etc.). Invalid inputs (bad bytes, wrong method, etc.) result in detailed error variants rather than panics.
* **Debug Logging:** On successful parse, the server currently prints debug info (via `dbg!(request)` or selective fields) to standard output. Malformed requests trigger a `println!` with the `ParseError` for easy debugging.
* **In Progress:** Note that this is a work in progress. At present the server *accepts connections and parses requests* but does not generate HTTP responses or serve files. Future enhancements may include full HTTP response handling, additional methods, and concurrent connection support.

## Project Structure

* **`main.rs`** – Entry point. It declares `mod http;`, creates a `Server` with `Server::new("127.0.0.1".to_string(), 8080)`, then calls `start()` and `stop()`. This simply initializes the server and runs it.
* **`http/mod.rs`** – Re-exports key types. It declares the submodules and publicly re-exports `Request`, `Method`, and `ParseError` so they can be used as `http::Request`, `http::Method`, etc..
* **`http/methods.rs`** – Defines the `Method` enum for HTTP verbs. See above for an example of its contents.
* **`http/parse_error.rs`** – Defines the `ParseError` enum for request parsing errors. It covers cases like invalid UTF-8, unsupported protocol, bad method, etc. (e.g. variants like `InvalidRequest`, `InvalidMethod`, `InvalidEncoding`, etc.).
* **`http/request.rs`** – Contains the `Request` struct and parsing logic. The `Request` struct holds fields `path: String`, `method: Method`, and `query_string: Option<String>`. The `impl TryFrom<&[u8]> for Request` attempts to parse a request line from a byte buffer. It uses a helper function `get_next_word` to tokenize by spaces or CRLF.
* **`http/server.rs`** – Implements the `Server` struct and networking. It has an address and port, and its `start()` method binds a `TcpListener` to that address. In a loop, it accepts connections and calls `handle_connection(stream)`. In that handler it reads into a buffer and then calls `process_request(&buffer)`. The `process_request` method uses `Request::try_from` to parse the request and logs success or error.

## Technical Highlights

* **TCP Socket Interaction:** The server uses `TcpListener::bind(&self.address).unwrap()` to listen on the specified address. Connections are accepted in a loop with `listener.accept()`, yielding a `TcpStream`. Each stream is passed to `handle_connection`, which reads up to 1024 bytes into a buffer.
* **Request Parsing (TryFrom):** Parsing is implemented via `impl TryFrom<&[u8]> for Request`. First the raw bytes are converted to a `&str`. Then successive tokens are extracted with a helper like:

  ```rust
  fn get_next_word(req_str: &str) -> Option<(&str, &str)> {
      for (idx, ch) in req_str.chars().enumerate() {
          if ch == ' ' || ch == '\r' || ch == '\n' {
              return Some((&req_str[..idx], &req_str[idx+1..]));
          }
      }
      None
  }
  ```

  This logic splits on spaces or newline, so it correctly isolates the HTTP method, path, and protocol.
* **HTTP Method & Protocol:** After tokenizing, the code matches the method string to the `Method` enum (e.g. `"GET" -> Method::GET`). Similarly it checks the protocol token against an `AllowedProtocol` enum (`"HTTP/1.1"`). Unsupported values yield a `ParseError`.
* **Error Handling:** The `ParseError` enum covers various failure points (invalid UTF-8, missing tokens, invalid method, etc.). For example, failing to parse bytes as UTF-8 yields `ParseError::InvalidEncoding`, while unrecognized methods yield `ParseError::InvalidMethod`. When parsing fails, the server logs `println!("Failed to parse request: {:?}", error)`.
* **Server Structure:** The `Server` struct is simple but extensible. Currently, `start()` just runs the accept loop and calls `process_request`. The server prints startup/shutdown messages, as well as request info using `dbg!`. For example:

  ```rust
  match Request::try_from(buffer) {
      Ok(request) => { dbg!(request); },
      Err(error) => { println!("Failed to parse request: {:?}", error); }
  }
  ```

.

* **Modularity:** The code separates concerns into modules (`server.rs`, `request.rs`, etc.). The `http` module is declared in `http/mod.rs`, and it re-exports types so that elsewhere you simply use `http::Server`, `http::Request`, etc. For example, the entry point does `use http::Server;`.

## Getting Started

*This project is under active development*. Currently it demonstrates connection handling and request parsing, but does not yet fully implement HTTP responses or other features. To build and run:

```bash
git clone https://github.com/rakibhossainraju/http_server.git
cd http_server
cargo build
cargo run
```

By default the server binds to `127.0.0.1:8080`. You can modify the address/port in `main.rs`. Note that features and documentation (including setup and usage examples) will be expanded as development continues.

## Contributing

Contributions are welcome! This is intended as a learning project, so if you’re interested in low-level Rust networking or HTTP internals, please feel free to fork, fix bugs, or add features. Possible contributions include implementing full HTTP response generation, handling more HTTP headers, adding multithreading or async support, improving the parsing logic, or writing tests. Even clarifications in the README or code comments are helpful. Your feedback and pull requests can help make this educational project even better.
