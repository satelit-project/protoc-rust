use prost_build::CodeGeneratorRequest;
use prost_build::Message;

use std::io::{stdin, stdout, Read, Write};

use protoc_rust::generate_response;

fn main() -> std::io::Result<()> {
    // read protoc request
    let mut input = Vec::new();
    stdin().read_to_end(&mut input)?;

    // generate protos
    let request = CodeGeneratorRequest::decode(&input).unwrap();
    let response = generate_response(request)?;

    // write response
    let mut output = Vec::new();
    response.encode(&mut output).unwrap();
    stdout().write_all(&output)
}
