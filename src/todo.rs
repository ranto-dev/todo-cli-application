use crate::Todo;
use super::user_input::input_string;

pub fn create_todo() -> Todo {
    println!("-- Insert new todo --");
    println!("enter title of todo:");
    let title = input_string();
    println!("enter description of todo:");
    let description = input_string();
    let new_todo: Todo = Todo { title: title, content: description };
    return new_todo;
}

pub fn display_todo_list(todos: Vec<Todo>) {
    println!("Your todo list: {:#?}", todos);
}