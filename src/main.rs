mod task;

fn main() {
    let toml = r#"
    title = "Task Title"
    version = "0.1.0"
    authors = [ "Jarbas" ]
    date = 2016-05-27T13:30:00Z
    category = "action"
    tags = [ "notification" ]
    min_jarbas_version = "0.1.0"

    [[languages]]
    name = "ruby"
    version = "2.3.0"

    [[inputs]]
    name = "input 1"
    type = "string"
    description = "Lorem Ipsum"
    mandatory = true

    [[configs]]
    name = "name"
    type = [ "dir" ]    # When the type is between square brackets an list with the type is expected
    description = "Lorem Ipsum"
    mandatory = true

    [[outputs]]
    name = "status"
    type = "boolean"
    description = "desc"
    "#;


    let test_task = task::Task::new(toml.to_string());
    println!("{:?}", test_task.inputs[0].iotype);
}
