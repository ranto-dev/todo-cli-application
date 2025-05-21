use super::user_input::input_i32;

#[derive(Debug)]
pub enum Choice {
    DisplayAll,
    Create,
    Update,
    Delete
}

pub fn user_asked_operation() -> Option<Choice> {
    println!("Menu\n1. Lister tous les todos\n2. Créer un todo\n3. Modifier un todo\n4. Supprimer un todo");
    let choice_operation: i32 = input_i32();
    let mut option: Choice = Choice::DisplayAll;

    match choice_operation {
        1 => {
            option = Choice::DisplayAll
        },
        2 => {
            option = Choice::Create;
        },
        3 => {
            option = Choice::Update;
        },
        4 => {
            option = Choice::Delete;
        },
        _ => {
            println!("Votre choix ne figure pas dans la liste de menu! Veuillez choisir une autre opération");
        }
    }

    return  Some(option);
}

pub fn user_asked_to_continue_proram() -> i32 {
    println!("Whould you want to continue? If you want press 1 but else 0");
    println!("Your choice: ");
    let continue_choice: i32 = input_i32();
    if continue_choice != 1 {
        println!("Merci d'avoir utiliser ce programme!")
    }
    return continue_choice;
}