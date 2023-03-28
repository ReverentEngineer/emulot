use std::process::{Command, Child, Stdio};
use serde::Deserialize;
use curl::easy::{Easy2 as Easy, Handler, List, WriteError};

#[test]
fn validate_guest_config_toml() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let bin = env!("CARGO_BIN_EXE_emulot");
    let output = Command::new(format!("{bin}"))
        .arg("run")
        .arg("--validate")
        .arg(format!("{0}/tests/data/test.toml", manifest_dir))
        .output()
        .unwrap();
    assert!(output.status.success())
}

struct ChildGuard(Child);

impl Drop for ChildGuard {

    fn drop(&mut self) {
        let _ = self.0.kill();
    }

}

#[derive(Deserialize)]
struct Request {
    method: String,
    url: String,
    headers: Option<Vec<String>>,
    contents: Option<String>
}

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

fn test_socket() -> String {
    format!("{}/daemon.sock", env!("CARGO_TARGET_TMPDIR"))
}

fn testdb() -> String {
    format!("{}/test.db", env!("CARGO_TARGET_TMPDIR"))
}

impl  From<Request> for Easy<Collector> {

    fn from(request: Request) -> Self {
        let Request { method, headers, url, contents } = request;
        let mut easy = Easy::new(Collector(Vec::new()));
        easy.url(&format!("http://localhost{url}")).unwrap();
        easy.unix_socket(&test_socket()).unwrap();
        if let Some(headers) = headers {
            let mut list = List::new();
            for header in headers {
                list.append(&header).unwrap();
            }
            easy.http_headers(list).unwrap();
        }
        match method.as_str() {
            "POST" => {
                easy.post(true).unwrap();
                if let Some(contents) = contents {
                    easy.post_fields_copy(contents.as_bytes()).unwrap();
                }
            }, 
            "GET" => (),
            "DELETE" => {
                easy.custom_request("DELETE").unwrap();
            },
            _ => panic!("Invalid method: {method}")
        }
        easy 
    }

}

#[derive(Deserialize)]
struct ExpectedResponse {
    code: u32,
    contents: Option<String>
}

#[derive(Deserialize)]
struct ApiTest {
    name: String,
    request: Request,
    response: ExpectedResponse
}

#[derive(Deserialize)]
struct ApiTests {
    tests: Vec<ApiTest>
}

fn wait_for_daemon() {
    let mut easy = Easy::new(Collector(Vec::new()));
    easy.url("http://localhost/health").unwrap();
    easy.unix_socket(&test_socket()).unwrap();
    while easy.perform().is_err() {
    }
}

#[test]
fn daemon_api() {
    if std::fs::metadata(testdb()).is_ok() {
        std::fs::remove_file(testdb()).unwrap();
    }
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let config_data = std::fs::read_to_string(format!("{manifest_dir}/tests/data/api.toml")).unwrap();

    let bin = env!("CARGO_BIN_EXE_emulot");
    let _child = ChildGuard(Command::new(format!("{bin}"))
        .arg("daemon")
        .env("EMULOT_LISTEN", format!("unix://{}", test_socket()))
        .env("EMULOT_STORAGE_URI", testdb())
        .spawn()
        .unwrap());

    wait_for_daemon();

    let config: ApiTests = toml::from_str(&config_data).unwrap();
    for ApiTest { name, request, response } in config.tests {
        let mut easy = Easy::<Collector>::from(request);
        if std::env::var("EMULOT_CURL_VERBOSE").is_ok() {
            easy.verbose(true).unwrap();
        }
        easy.perform().unwrap();
        let response_code = easy.response_code().unwrap();
        assert_eq!(response_code, response.code, "'{name}' test had incorrect response code");
        
        if let Some(contents) = response.contents {
            assert_eq!(contents.as_bytes(), easy.get_ref().0, "'{name}' test had in correct response contents"); 
        }
    }
}

#[test]
fn curl() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let curl = Command::new("curl")
        .arg(format!("file://{manifest_dir}/tests/data/test.toml"))
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .spawn().unwrap();
    
    let bin = env!("CARGO_BIN_EXE_emulot");
    let emulot = Command::new(format!("{bin}"))
        .arg("run")
        .arg("--validate")
        .stdin(curl.stdout.unwrap())
        .output()
        .unwrap();
    assert!(emulot.status.success(), "Emulot failed to validate config");
}
