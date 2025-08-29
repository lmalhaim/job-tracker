use inquire::Select;
use std::vec;
#[derive(Debug, Clone)]

// TODO:
// [x] Implement CLI logic
// [x] Refactor nested loops
// [ ] Add logging and error handling (time out for errors then exit the loop)

enum Exit {
    True,
    False,
}




// MAIN MENU 
pub enum MainMenu {
    SignUp,
    SignIn,
    Quit,
}
impl MainMenu {
    fn all_options() -> Vec<Self> {
        vec![MainMenu::SignUp, MainMenu::SignIn, MainMenu::Quit]
    }
    pub fn show_menu() {
        loop {
            let user_choice =
                inquire::Select::new("What would you like to do?", MainMenu::all_options())
                    .prompt();
            match user_choice {
                Ok(choice) => match choice {
                    MainMenu::SignIn => {
                        println!("Sign in");
                        match SignInMenu::show_menu() {
                            Exit::True => break,
                            Exit::False => continue,
                        }
                    }
                    MainMenu::SignUp => {
                        println!("Sign up");
                        continue;
                    }
                    MainMenu::Quit => break,
                },
                Err(_) => {
                    //TODO: ERROR HANDLING AND LOGGING
                    println!("Error encountered, please try again");
                    continue;
                }
            }
        }
    }
}
impl std::fmt::Display for MainMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MainMenu::SignUp => write!(f, "Sign up"),
            MainMenu::SignIn => write!(f, "Sign in"),
            MainMenu::Quit => write!(f, "Quit"),
        }
    }
}

// SIGN-IN MENU 
#[derive(Debug, Clone)]
enum SignInMenu {
    ManageEmail,
    SyncEmail,
    ManageJobs,
    Back,
    Quit,
}
impl SignInMenu {
    fn all_options() -> Vec<Self> {
        vec![
            SignInMenu::ManageEmail,
            SignInMenu::SyncEmail,
            SignInMenu::ManageJobs,
            SignInMenu::Back,
            SignInMenu::Quit,
        ]
    }
    fn show_menu() -> Exit {
        loop {
            let user_choice =
                Select::new("What would you like to do?", SignInMenu::all_options()).prompt();
            match user_choice {
                Ok(choice) => {
                    return match choice {
                        SignInMenu::ManageEmail => {
                            // manage email
                            match EmailMenu::show_menu() {
                                Exit::True => return Exit::True,
                                Exit::False => continue,
                            }
                        }
                        SignInMenu::ManageJobs => {
                            // Manage jobs
                            match JobMenu::show_menu() {
                                Exit::True => return Exit::True,
                                Exit::False => continue,
                            }
                        }
                        SignInMenu::SyncEmail => {
                            println!("Syncing email");
                            continue;
                        }
                        SignInMenu::Back => break,
                        SignInMenu::Quit => Exit::True,
                    };
                }
                Err(_) => {
                    //TODO: ERROR HANDLING AND LOGGING
                    println!("Error encountered, please try again");
                    continue;
                }
            }
        }
        Exit::False
    }
}
impl std::fmt::Display for SignInMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SignInMenu::ManageEmail => write!(f, "Manage Email"),
            SignInMenu::SyncEmail => write!(f, "Sync email statuses"),
            SignInMenu::ManageJobs => write!(f, "Manage/Display Jobs"),
            SignInMenu::Back => write!(f, "Back"),
            SignInMenu::Quit => write!(f, "Quit"),
        }
    }
}

// EMAIL MENU
#[derive(Debug, Clone)]
enum EmailMenu {
    Connect,
    Remove,
    Back,
    Quit,
}
impl EmailMenu {
    fn all_options() -> Vec<Self> {
        vec![
            EmailMenu::Connect,
            EmailMenu::Remove,
            EmailMenu::Back,
            EmailMenu::Quit,
        ]
    }
    fn show_menu() -> Exit {
        loop {
            let user_choice = Select::new("Choose", EmailMenu::all_options()).prompt();
            match user_choice {
                Ok(choice) => match choice {
                    EmailMenu::Connect => println!("Connect to email"),
                    EmailMenu::Remove => println!("Remove Email"),
                    EmailMenu::Back => break,
                    EmailMenu::Quit => return Exit::True,
                },
                Err(_) => {
                    //TODO: ERROR HANDLING AND LOGGING
                    println!("Error encountered, please try again");
                    continue;
                }
            }
        }
        Exit::False
    }
}
impl std::fmt::Display for EmailMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EmailMenu::Connect => write!(f, "Connect to Email"),
            EmailMenu::Remove => write!(f, "Remove Email"),
            EmailMenu::Back => write!(f, "Back"),
            EmailMenu::Quit => write!(f, "Quit"),
        }
    }
}

// JOB MENU 
#[derive(Debug, Clone)]
enum JobMenu {
    Display,
    AddNew,
    EditStatus,
    Back,
    Quit,
}
impl JobMenu {
    fn all_options() -> Vec<Self> {
        vec![
            JobMenu::Display,
            JobMenu::AddNew,
            JobMenu::EditStatus,
            JobMenu::Back,
            JobMenu::Quit,
        ]
    }
    fn show_menu() -> Exit {
        loop {
            let user_choice = Select::new("Choose", JobMenu::all_options()).prompt();

            match user_choice {
                Ok(choice) => match choice {
                    JobMenu::Display => println!("Display jobs"),
                    JobMenu::AddNew => println!("Add new job"),
                    JobMenu::EditStatus => println!("Edit job status"),
                    JobMenu::Back => break,
                    JobMenu::Quit => {
                        return Exit::True;
                    }
                },
                Err(_) => {
                    //TODO: ERROR HANDLING AND LOGGING
                    println!("Error encountered, please try again");
                    continue;
                }
            }
        }
        Exit::False
    }
}
impl std::fmt::Display for JobMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JobMenu::Display => write!(f, "Display Jobs Table"),
            JobMenu::AddNew => write!(f, "Add new job"),
            JobMenu::EditStatus => write!(f, "Edit job status"),
            JobMenu::Back => write!(f, "Back"),
            JobMenu::Quit => write!(f, "Quit"),
        }
    }
}
