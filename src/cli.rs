use inquire::Select;
use std::vec;
#[derive(Debug, Clone)]

// TODO: 
// [x] Implement CLI logic 
// [ ] Refactor nested loops 
// [ ] Add logging and error handling 

enum MainMenu {
    SignUp,
    SignIn,
    Quit,
}
impl MainMenu {
    fn all_options() -> Vec<Self> {
        vec![MainMenu::SignUp, MainMenu::SignIn, MainMenu::Quit]
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

pub fn show_main_menu() {
    'main_menu: loop {
        let first_prompt =
            inquire::Select::new("What would you like to do?", MainMenu::all_options()).prompt();
        if let Ok(first_choice) = first_prompt {
            match first_choice {
                MainMenu::SignIn => {
                    'sign_in_menu: loop {
                        let second_prompt =
                            Select::new("What would you like to do?", SignInMenu::all_options())
                                .prompt();

                        if let Ok(second_choice) = second_prompt {
                            match second_choice {
                                SignInMenu::ManageEmail => {
                                    // manage email
                                    'manage_email_menu: loop {
                                        let third_prompt =
                                            Select::new("Choose", EmailMenu::all_options())
                                                .prompt();
                                        if let Ok(third_choice) = third_prompt {
                                            match third_choice {
                                                EmailMenu::Connect => println!("Connect to email"),
                                                EmailMenu::Remove => println!("Remove Email"),
                                                EmailMenu::Back => break 'manage_email_menu,
                                                EmailMenu::Quit => break 'main_menu,
                                            }
                                        }
                                    }
                                }
                                SignInMenu::ManageJobs => {
                                    // Manage jobs
                                    'manage_jobs_menu: loop {
                                        let third_prompt =
                                            Select::new("Choose", JobMenu::all_options()).prompt();
                                        if let Ok(third_choice) = third_prompt {
                                            match third_choice {
                                                JobMenu::Display => println!("Display jobs"),
                                                JobMenu::AddNew => println!("Add new job"),
                                                JobMenu::EditStatus => println!("Edit job status"),
                                                JobMenu::Back => break 'manage_jobs_menu,
                                                JobMenu::Quit => break 'main_menu,
                                            }
                                        }
                                    }
                                }
                                SignInMenu::SyncEmail => println!("Syncing email"),
                                SignInMenu::Back => break 'sign_in_menu,
                                SignInMenu::Quit => break 'main_menu,
                            }
                        }
                    }
                }
                MainMenu::SignUp => println!("Sign up"),
                MainMenu::Quit => break 'main_menu,
            }
        }
    }
}
