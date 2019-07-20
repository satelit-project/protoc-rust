use prost_build::CodeGeneratorRequest;
use prost_build::Message;

use std::io::{stdin, stdout, Read, Write};

use protoc_rust::generate_response;

fn main() -> std::io::Result<()> {
    let mut input = Vec::new();
    stdin().read_to_end(&mut input)?;

    let request = CodeGeneratorRequest::decode(&input).unwrap();
    let response = generate_response(request)?;

    let mut output = Vec::new();
    response.encode(&mut output).unwrap();
    stdout().write_all(&output)
}
