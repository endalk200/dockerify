fn main() {
    let supported_projects: Vec<String> = vec!["node".to_string()];

    let project = std::env::args().nth(1).expect("no project specified");

    if !supported_projects.contains(&project) {
        println!("{} is not a supported project", project);
        std::process::exit(1);
    }
}
