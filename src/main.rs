mod templating_engine;
mod utils;

use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let supported_projects: Vec<String> = vec!["nestjs".to_string(), "nextjs".to_string()];

    let project: String = std::env::args().nth(1).expect("no project specified");

    if !supported_projects.contains(&project) {
        println!("{} is not a supported project", project);
        std::process::exit(1);
    }

    let dockerfile_content: String = templating_engine::get_template_content(&project);

    fs::write(Path::new(".").join("Dockerfile"), dockerfile_content)?;

    Ok(())
}
