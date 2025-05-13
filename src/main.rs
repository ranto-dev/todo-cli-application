#![allow(unused)]
mod todo;
mod user_input;
mod user_asked;

#[derive(Debug, Clone)]
struct Todo {
    title: String,
    content: String
}

fn main() {
    println!("-- Todo CLI Application --");
    let mut continue_choice: i32 = 0;
    let mut operation_choice: i32 = 0;

    let mut todos: Vec<Todo> = Vec::new();

    while operation_choice==1 {
        todos = todo::create_todo(&mut todos).to_vec();
        todo::display_todo_list(todos.clone())
    }
    todo::display_todo_list(todos);

    let continue_choice = user_asked::user_asked_to_continue_proram();
}
