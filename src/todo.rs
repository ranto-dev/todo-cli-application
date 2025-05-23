use crate::{user_input::input_i32, Todo};
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

pub fn edit_todo_by_id(todos: &mut Vec<Todo>) -> Vec<Todo> {
    let len_todos: i32 = todos.len() as i32;
    let mut todo_id_to_edit: i32 = 0;

    println!("{}", len_todos);

    if len_todos < 1 {
        println!("Vous n'avez pas encore de todos");
    }

    while todo_id_to_edit < 1 || todo_id_to_edit > len_todos {
        println!("You want to update todo at id?");
        todo_id_to_edit = input_i32();
    }

    for todo in &mut * todos {
        if todo.id == todo_id_to_edit {
            println!("-- Update todo id: {} --", todo.id);
            println!("enter title of todo:");
            todo.title = input_string();
            println!("enter description of todo:");
            todo.content = input_string();
        }
    }

    let new_todo: Vec<Todo> = (*todos.clone()).to_vec();
    return new_todo.clone();
}

pub fn remove_todo_by_id(todos: &mut Vec<Todo>) {
    let len_todos: i32 = todos.len() as i32;
    let mut todo_id_to_remove: i32 = 0;

    println!("{}", len_todos);

    if len_todos < 1 {
        println!("Vous n'avez pas encore de todos");
    }

    while todo_id_to_remove < 1 || todo_id_to_remove > len_todos {
        println!("You want to remove todo at id?");
        todo_id_to_remove = input_i32();
    }

    println!("{}", todo_id_to_remove);

    todos.remove((todo_id_to_remove as usize) - 1);
}