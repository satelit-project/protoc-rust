use prost_build::code_generator_response::File;
use prost_build::CodeGeneratorResponse;

/// Tree structure that represents rust's module hierarchy
#[derive(Debug)]
pub struct ModuleTree<'a> {
    /// Module name
    name: &'a str,

    /// Content of the module
    content: Option<&'a str>,

    /// Module's submodules
    children: Vec<ModuleTree<'a>>,
}

impl<'a> From<&'a CodeGeneratorResponse> for ModuleTree<'a> {
    /// Creates module tree from 'protoc' response with flat modules
    ///
    /// A file for a flat module should be named like 'parent.child.file.rs'. This way
    /// we can infer actual module hierarchy.
    fn from(response: &'a CodeGeneratorResponse) -> Self {
        // mod.rs for a code output directory (top-level module)
        let mut root = ModuleTree {
            name: "",
            content: None,
            children: vec![],
        };

        let files = match response {
            CodeGeneratorResponse {
                error: None,
                file: files,
            } => files,
            _ => return root,
        };

        for file in files {
            if let Some(name) = file.name.as_ref() {
                let path = name.trim_end_matches(".rs").split('.').collect();
                let content = file.content.as_ref().map(|s| s.as_str());
                create_subtree(content, path, &mut root);
            }
        }

        root
    }
}

impl<'a> From<ModuleTree<'a>> for CodeGeneratorResponse {
    /// Creates `CodeGeneratorResponse` with modules which respects proto's package definition
    fn from(root: ModuleTree<'a>) -> Self {
        let mut response = CodeGeneratorResponse {
            error: None,
            file: vec![],
        };

        for child in &root.children {
            let path = vec![child.name];
            write_subtree(&mut response, path, child);
        }

        response
    }
}

/// Puts module content to the right node in modules tree
///
/// # Arguments
///
/// * `content` – content of the module (generated source code)
/// * `path` – path to the module related to a top-level module (like ['parent', 'child', 'file'])
/// * `root` – root node of a top-level module
fn create_subtree<'a, 'b>(
    content: Option<&'a str>,
    mut path: Vec<&'a str>,
    root: &'b mut ModuleTree<'a>,
) {
    if path.is_empty() {
        root.content = content;
        return;
    }

    let name = path.remove(0);
    let existing = root.children.iter_mut().find(|e| e.name.eq(name));

    if let Some(child) = existing {
        create_subtree(content, path, child);
    } else {
        let mut child = ModuleTree {
            name,
            content: None,
            children: vec![],
        };

        create_subtree(content, path, &mut child);
        root.children.push(child);
    }
}

/// Translates a module hierarchy to rust files
///
/// # Arguments
///
/// * `response` – 'protoc' response to place files
/// * `path` – module path relative to a top-level module (like ['parent'])
/// * `root` – root node of a top-level module
fn write_subtree<'a, 'b>(
    response: &'b mut CodeGeneratorResponse,
    path: Vec<&'a str>,
    root: &'b ModuleTree<'a>,
) {
    let mut content = String::new();
    let mut name = path.join("/");
    name.push_str(".rs");

    for child in &root.children {
        content.push_str("pub mod ");
        content.push_str(child.name);
        content.push_str(";\n");

        let mut path_cpy = path.clone();
        path_cpy.push(child.name);
        write_subtree(response, path_cpy, child);
    }

    if let Some(node_content) = root.content {
        if !content.is_empty() {
            content.push('\n');
        }

        content.push_str(node_content)
    }

    response.file.push(File {
        name: Some(name),
        insertion_point: None,
        content: Some(content),
    })
}
