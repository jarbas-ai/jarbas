extern crate toml;

enum TaskIOType {
    int,
    float,
    string,
    boolean,
    mail,
    dir,
    file,
    telephone,
    url
}

fn str_to_iotype(iotype: String) -> TaskIOType {
    match &*iotype {
        "int"       => TaskIOType::int,
        "float"     => TaskIOType::float,
        "string"    => TaskIOType::string,
        "boolean"   => TaskIOType::boolean,
        "mail"      => TaskIOType::mail,
        "dir"       => TaskIOType::dir,
        "file"      => TaskIOType::file,
        "telephone" => TaskIOType::telephone,
        "url"       => TaskIOType::url,
        _           => TaskIOType::string
    }
}

pub struct TaskLanguage {
    name: String,
    version: String
}

impl TaskLanguage {
    pub fn new(toml: &toml::Value) -> TaskLanguage {
        TaskLanguage {
            name: toml.lookup("name").unwrap().as_str().unwrap().to_string(),
            version: toml.lookup("version").unwrap().as_str().unwrap().to_string()
        }
    }
}

pub struct TaskInput {
    pub name: String,
    pub iotype: TaskIOType,
    description: String,
    mandatory: bool
}

impl TaskInput {
    pub fn new(toml: &toml::Value) -> TaskInput {
        TaskInput {
            name: toml.lookup("name").unwrap().as_str().unwrap().to_string(),
            iotype: str_to_iotype(toml.lookup("type").unwrap().as_str().unwrap().to_string()),
            description: toml.lookup("description").unwrap().as_str().unwrap().to_string(),
            mandatory: toml.lookup("mandatory").unwrap().as_bool().unwrap()
        }
    }
}

pub struct TaskOutput {
    name: String,
    iotype: TaskIOType,
    description: String,
}

impl TaskOutput {
    pub fn new(toml: &toml::Value) -> TaskOutput {
        TaskOutput {
            name: toml.lookup("name").unwrap().as_str().unwrap().to_string(),
            iotype: str_to_iotype(toml.lookup("type").unwrap().as_str().unwrap().to_string()),
            description: toml.lookup("description").unwrap().as_str().unwrap().to_string()
        }
    }
}

struct TaskConfig {
    name: String,
    iotype: TaskIOType,
    description: String
}

pub struct Task {
    pub title: String,
    pub version: String,
    pub authors: Vec<String>,
    pub date: String,
    pub category: String,
    pub tags: Vec<String>,
    pub min_jarbas_version: String,
    pub languages: Vec<TaskLanguage>,
    pub inputs: Vec<TaskInput>,
    pub outputs: Vec<TaskOutput>
}

impl Task {
    pub fn new(tomlInput: String) -> Task {
        let mut parser = toml::Parser::new(&tomlInput);
        let toml = parser.parse().unwrap();
        Task {
            title: toml["title"].as_str().unwrap().to_string(),
            version: toml["version"].as_str().unwrap().to_string(),
            authors: toml["authors"].as_slice().unwrap().into_iter().map( |tv| tv.as_str().unwrap().to_string() ).collect(),
            date: toml["date"].as_datetime().unwrap().to_string(),
            category: toml["category"].as_str().unwrap().to_string(),
            tags: toml["tags"].as_slice().unwrap().into_iter().map( |tv| tv.as_str().unwrap().to_string() ).collect(),
            min_jarbas_version: toml["min_jarbas_version"].as_str().unwrap().to_string(),
            languages: toml["languages"].as_slice().unwrap().into_iter().map( |tv| TaskLanguage::new(tv) ).collect(),
            inputs: toml["inputs"].as_slice().unwrap().into_iter().map( |tv| TaskInput::new(tv) ).collect(),
            outputs: toml["outputs"].as_slice().unwrap().into_iter().map( |tv| TaskOutput::new(tv) ).collect()
        }
    }
}
