use crate::Todo;
use super::user_input::input_string;

pub fn create_todo(len_actually: i32) -> Todo {
    println!("Len actually: {}", len_actually);
    println!("-- Insert new todo --");
    println!("enter title of todo:");
    let title = input_string();
    println!("enter description of todo:");
    let description = input_string();
    let new_todo: Todo = Todo { id: len_actually + 1 , title: title, content: description };
    return new_todo;
}

pub fn display_todo_list(todos: Vec<Todo>) {

    if todos.len() <=0 {
        println!("Vous n'avez pas encore de todos");
    } else {
        println!("Your todo list:\n{:#?}", todos);
    }
}