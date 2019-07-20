mod tree;

use prost_build::Config as ProtoConfig;
use prost_build::{CodeGeneratorRequest, CodeGeneratorResponse};
use tower_grpc_build::ServiceGenerator;

pub fn generate_response(request: CodeGeneratorRequest) -> std::io::Result<CodeGeneratorResponse> {
    let config = Config::from_request(&request);

    let mut service_gen = ServiceGenerator::default();
    service_gen.enable_client(config.gen_grpc_client);
    service_gen.enable_server(config.gen_grpc_server);

    let mut proto_config = ProtoConfig::new();
    proto_config.service_generator(Box::new(service_gen));

    let response = proto_config.compile_request(request)?;
    Ok(modularize_response(response))
}

fn modularize_response(response: CodeGeneratorResponse) -> CodeGeneratorResponse {
    response
}

/// Code generation configuration
struct Config {
    gen_grpc_client: bool,
    gen_grpc_server: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            gen_grpc_client: false,
            gen_grpc_server: false,
        }
    }
}

impl Config {
    /// Parses configuration from request parameters
    ///
    /// To pass custom parameters you need to specify `--rust_opts` argument
    /// for `protoc` compiler. Multiple parameters may be passed by separating
    /// them via comma.
    fn from_request(request: &CodeGeneratorRequest) -> Self {
        let mut conf = Self::default();
        let opts = request.parameter.as_ref().map(|s| s.as_str()).unwrap_or("");

        for opt in opts.split(',') {
            match opt {
                "grpc" => {
                    conf.gen_grpc_client = true;
                    conf.gen_grpc_server = true;
                }
                "grpc-client" => conf.gen_grpc_client = true,
                "grpc-server" => conf.gen_grpc_server = true,
                _ => (),
            }
        }

        conf
    }
}
