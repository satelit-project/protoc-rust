use prost_build::CodeGeneratorResponse;
use prost_build::code_generator_response::File;

#[derive(Debug)]
struct ModuleTree<'a> {
    name: &'a str,
    content: Option<&'a str>,
    children: Vec<ModuleTree<'a>>,
}

fn build_tree(response: &CodeGeneratorResponse) -> ModuleTree {
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
