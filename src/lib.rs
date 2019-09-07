mod tree;

use prost_build::Config as ProtoConfig;
use prost_build::{CodeGeneratorRequest, CodeGeneratorResponse};
use tower_grpc_build::ServiceGenerator;

use std::string::ToString;

use tree::ModuleTree;

/// Generates response for 'protoc' compiler by processing it's request
///
/// This function will generate code for proto files from `CodeGeneratorRequest` that
/// 'protoc' compiler passes to it's plugins and will return it in expected format.
pub fn generate_response(request: CodeGeneratorRequest) -> std::io::Result<CodeGeneratorResponse> {
    let config = Config::from_request(&request);

    let mut service_gen = ServiceGenerator::default();
    service_gen.enable_client(config.gen_grpc_client);
    service_gen.enable_server(config.gen_grpc_server);

    let mut proto_config = ProtoConfig::new();
    proto_config.service_generator(Box::new(service_gen));

    for (proto, rust) in &config.extern_paths {
        proto_config.extern_path(proto, rust);
    }

    let mut response = proto_config.compile_request(request)?;
    if !config.flat_modules {
        response = modularize_response(response)
    }

    Ok(response)
}

/// Modifies `CodeGeneratorResponse` to have module hierarchy that respects
/// package declaration for provided proto files
fn modularize_response(response: CodeGeneratorResponse) -> CodeGeneratorResponse {
    let modules = ModuleTree::from(&response);
    modules.into()
}

/// Code generation configuration
struct Config {
    /// Generate client side RPC services
    gen_grpc_client: bool,

    /// Generate server side RPC services
    gen_grpc_server: bool,

    /// External crates for Protobuf types
    ///
    /// See this[^1] method's documentation.
    ///
    /// [^1]: https://github.com/danburkert/prost/blob/c2c6feaee0eebd9eb71f6e9fb20e26fd0ef2a0c8/prost-build/src/lib.rs#L442
    extern_paths: Vec<(String, String)>,

    /// Generate proto files as top-level modules
    ///
    /// If `true` then generated code will be placed in a top-level module without respecting
    /// package hierarchy. For example, for package 'foo.bar' the code will be placed in
    /// 'foo.bar' rust module.
    ///
    /// If `false` then generated code will respect proto's package declaration and for
    /// each package a new rust module will be generated. For example, for package 'foo.bar'
    /// the code will be placed in module 'bar' which is submodule of 'foo' module.
    ///
    /// **NOTE:** Module hierarchy is inferred from proto files that has been provided to
    /// 'protoc' compiler. So, to be able to correctly infer module hierarchy, all proto files
    /// that corresponds to the same root package must be provided to 'protoc' at once.
    flat_modules: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            gen_grpc_client: false,
            gen_grpc_server: false,
            extern_paths: Vec::new(),
            flat_modules: true,
        }
    }
}

impl Config {
    /// Parses configuration from request parameters
    ///
    /// To pass custom parameters you need to specify `--rust_opt` argument
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
                "no-flat-modules" => conf.flat_modules = false,
                paths if paths.starts_with("extern-path=") => {
                    let mut parts = paths.split('=').skip(1);
                    let proto_path = parts.next().expect("proto path not found");
                    let rust_path = parts.next().expect("rust path not found");

                    let pair = (proto_path.to_string(), rust_path.to_string());
                    conf.extern_paths.push(pair);
                }
                _ => (),
            }
        }

        conf
    }
}
