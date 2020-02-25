
pub fn help() -> Vec<String> {
    vec![
    String::from("Debug_args"), String::from("-help")
    ]
}

pub fn list() -> Vec<String> {
    vec![
    String::from("Debug_args"), String::from("-list")
    ]
}

pub fn add() -> Vec<String> {
    vec![
    String::from("Debug_args"), String::from("-add"), String::from("UserFoo")
    ]
}

pub fn remove() -> Vec<String> {
    vec![
    String::from("Debug_args"), String::from("-remove"), String::from("UserFoo")
    ]
}

pub fn modify_a() -> Vec<String> {
    vec![
    String::from("Debug_args"), String::from("-modify"), String::from("UserFoo"),
    String::from("-a") 
    ]
}

pub fn modify_o() -> Vec<String> {
    vec![
    String::from("Debug_args"), String::from("-modify"), String::from("UserFoo"),
    String::from("-a") 
    ]
}

pub fn modify_r() -> Vec<String> {
    vec![
    String::from("Debug_args"), String::from("-modify"), String::from("UserFoo"),
    String::from("-a"), String::from("<restriction1>"), String::from("<restriction2>")
    ]
}

pub fn bad() -> Vec<String> {
    vec![
    String::from("Debug_args"), String::from("-invalid_input")
    ]
}
