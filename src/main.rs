use todo::{create_todo, display_todo_list, edit_todo_by_id};
use user_asked::{user_asked_operation, user_asked_to_continue_proram, Choice};

mod todo;
mod user_input;
mod user_asked;

#[derive(Debug, Clone)]
struct Todo {
    id: i32,
    title: String,
    content: String
}

fn main() {
    println!("-- Todo CLI Application --");
    let mut continue_program: i32 = 1;

    let mut todos: Vec<Todo> = Vec::new();

    while continue_program == 1 {
       
        let choice_operation: Option<user_asked::Choice> = user_asked_operation();
        match choice_operation.unwrap() {
            Choice::DisplayAll =>{
                display_todo_list(todos.clone());
            },
            Choice::Create => {
                todos.push(create_todo(todos.clone().len() as i32));
                println!("Todo créée avec success!")
            },
            Choice::Update => {
                todos = edit_todo_by_id(&mut todos);
            },
            Choice::Delete => println!("You choice Delete")
        }
        continue_program = user_asked_to_continue_proram();
    }
}
