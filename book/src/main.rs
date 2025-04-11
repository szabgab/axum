fn main() {
    read_toml_files();
}

fn read_toml_files() {
    let path = std::path::PathBuf::from("../examples");
    let mut deps = std::collections::HashSet::new();
    for thing in path.read_dir().unwrap() {
        let thing = thing.unwrap().path();
        //println!("{:?}", thing);
        let toml_file = thing.join("Cargo.toml");
        if !toml_file.exists() {
            //println!("No Cargo.toml found in {:?}", thing);
            continue;
        }

        let content = std::fs::read_to_string(toml_file).unwrap();
        let toml: toml::Value = toml::from_str(&content).unwrap();
        for section in ["dependencies", "dev-dependencies"] {
            if let Some(dependencies) = toml.get(section) {
                for (thing, _) in dependencies.as_table().unwrap() {
                    deps.insert(thing.to_string());
                }
    
            }
        }
    }
    let mut md = String::from("# Crates in use\n\n");
    md += "The following crates are used in the examples.\n\n";
    let mut names = deps.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    names.sort();
    for name in names {
        md += &format!("- [{}](https://crates.io/crates/{})\n", name, name);
    }

    std::fs::write("pages/crates.md", md).unwrap()

}

