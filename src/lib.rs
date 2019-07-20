mod tree;

use prost_build::Config as ProtoConfig;
use prost_build::{CodeGeneratorRequest, CodeGeneratorResponse};
use tower_grpc_build::ServiceGenerator;

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
            flat_modules: true,
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
                "no-flat-modules" => conf.flat_modules = false,
                _ => (),
            }
        }

        conf
    }
}
