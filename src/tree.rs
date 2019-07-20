use prost_build::code_generator_response::File;
use prost_build::CodeGeneratorResponse;

#[derive(Debug)]
pub struct ModuleTree<'a> {
    name: &'a str,
    content: Option<&'a str>,
    children: Vec<ModuleTree<'a>>,
}

impl<'a> From<&'a CodeGeneratorResponse> for ModuleTree<'a> {
    fn from(response: &'a CodeGeneratorResponse) -> Self {
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
                put_content(content, path, &mut root);
            }
        }

        root
    }
}

impl<'a> From<ModuleTree<'a>> for CodeGeneratorResponse {
    fn from(root: ModuleTree<'a>) -> Self {
        let mut response = CodeGeneratorResponse {
            error: None,
            file: vec![],
        };

        for child in &root.children {
            let path = vec![child.name];
            put_node(&mut response, path, child);
        }

        response
    }
}

fn put_content<'a, 'b>(
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
        put_content(content, path, child);
    } else {
        let mut child = ModuleTree {
            name,
            content: None,
            children: vec![],
        };

        put_content(content, path, &mut child);
        root.children.push(child);
    }
}

fn put_node<'a, 'b>(
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
        put_node(response, path_cpy, child);
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
