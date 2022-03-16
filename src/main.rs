use std::error::Error;

use handlebars::Handlebars;
use serde_json::json;

// Policies:
// - rds
// - iam
// - eks
// - sns
// - sqs
// - ecr
// - dynamodb
// service_name (readme)
// squad_name (license)
// squad_email (license)
// channel slack (readme)
// url do serviço (readme)
//
fn main() -> Result<(), Box<dyn Error>> {
    let mut reg = Handlebars::new();

    reg.register_template_file("index", "./templates/index.hbs")
        .expect("Could not load index template");

    println!("{}", reg.render("index", &json!({"name": "Murilo"}))?);

    Ok(())
}
