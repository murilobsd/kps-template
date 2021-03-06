#![allow(dead_code)]
use std::{
    error::Error,
    fs::{create_dir_all, File},
    io::{prelude::*, BufReader},
    path::Path,
};

use handlebars::{handlebars_helper, Handlebars};
use serde_json::json;

// Policies:
// - rds
// - iam
// - eks
// - sns
// - sqs
// - ecr
// - dynamodb

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

struct Tpl<'a> {
    lang: &'a str,
    reg: Handlebars<'a>,
}

impl<'a> Tpl<'a> {
    pub fn new(lang: &'a str) -> Self {
        let mut reg = Handlebars::new();
        reg.register_helper("captialize", Box::new(captialize));
        Self { lang, reg }
    }

    pub fn insert(&mut self, name: &str) {
        let tpl_path =
            format!("./templates/{lang}/{name}", lang = self.lang, name = name);

        self.reg
            .register_template_file(name, tpl_path)
            .expect("Could not load index template");
    }

    pub fn render(&self, name: &str, data: &serde_json::Value) {
        println!("{}", self.reg.render(name, data).unwrap());
    }

    pub fn render_to_file(
        &self,
        name: &str,
        data: &serde_json::Value,
        out_path: &str,
    ) {
        println!("{} generated", out_path);
        let mut output = File::create(out_path).unwrap();
        self.reg.render_to_write(name, data, &mut output).unwrap();
    }
}

handlebars_helper!(captialize: |s: str| {
    let mut chars = s.chars();
    chars
        .next()
        .map(|first_letter| first_letter.to_uppercase())
        .into_iter()
        .flatten()
        .chain(chars)
        .collect::<String>()
});

fn main() -> Result<(), Box<dyn Error>> {
    let mut reg = Tpl::new("rust");
    let data = json!({"container_port": 8080,
        "docker_image_uri": "teste",
        "replicas": 2,
        "service_name": "app-mbsd",
        "squad_name": "platform-engineer",
        "squad_email": "platform-engineer@kps.com.br",
        "version": "0.0.1",
        "slack_channel": "#channel",
        "year": 2022
    });

    let rust_files = lines_from_file("./templates/rust_files.txt");
    let out_folder = "./output";
    create_dir_all(out_folder).unwrap();

    for name in &rust_files {
        let output_path =
            format!("{folder}/{path}", folder = out_folder, path = name);

        let path = Path::new(&output_path);
        let parent = path.parent().unwrap();
        // TODO: check if exist
        if !parent.exists() {
            create_dir_all(parent).unwrap();
        }
        reg.insert(name);
        // reg.render(name, &data);
        reg.render_to_file(name, &data, &output_path);
    }

    Ok(())
}
