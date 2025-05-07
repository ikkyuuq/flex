fn main() {}

mod app {
    use std::{
        error::Error,
        fmt::{Debug, Display},
    };

    use crate::args::command::Command;

    #[derive(Debug, PartialEq)]
    pub enum AppError {
        MissingArgument(String),
        InvalidCommand(String),
        ExecutionError(String),
    }

    impl Display for AppError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::MissingArgument(e) => {
                    write!(f, "{}", e)
                }
                Self::InvalidCommand(e) => {
                    write!(f, "{}", e)
                }
                Self::ExecutionError(e) => {
                    write!(f, "{}", e)
                }
            }
        }
    }

    impl Error for AppError {}

    #[derive(Default, Debug, PartialEq)]
    pub struct App {
        pub name: String,
        pub short_description: Option<String>,
        pub long_description: Option<String>,
        pub command: Vec<Command>,
    }

    impl App {
        pub fn new(name: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                ..Default::default()
            }
        }

        pub fn add_command(mut self, cmd: Command) -> Self {
            self.command.push(cmd);
            self
        }

        pub fn run(&self, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
            if args.is_empty() {
                // TODO: display help messages
                return Err(Box::new(AppError::InvalidCommand(
                    "Command is not provided".to_string(),
                )));
            }

            let command_name = &args[0];
            let command_args = &args[1..];

            for cmd in &self.command {
                if cmd.name == *command_name {
                    // TODO: execute the command
                    // TODO: Write the test for good result
                    return Ok(());
                }
            }

            Err(Box::new(AppError::InvalidCommand(format!(
                "Unknown Command Provided: {}",
                command_name
            ))))
        }
    }
}

mod args {
    pub mod command {
        use std::{error::Error, fmt::Debug};

        use super::positional_arg::PositionalArgument;

        pub struct Command {
            pub name: String,
            pub short_description: Option<String>,
            pub long_description: Option<String>,
            pub positional_arguments: Vec<PositionalArgument>,
            pub action: Option<Box<dyn Fn(&[String]) -> Result<(), Box<dyn Error>>>>,
            pub sub_command: Vec<Command>,
        }

        impl Default for Command {
            fn default() -> Self {
                Self {
                    name: Default::default(),
                    short_description: Default::default(),
                    long_description: Default::default(),
                    positional_arguments: Default::default(),
                    action: Default::default(),
                    sub_command: Default::default(),
                }
            }
        }

        impl Debug for Command {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Command")
                    .field("name", &self.name)
                    .field("short_description", &self.short_description)
                    .field("long_description", &self.long_description)
                    .field("positional_arguments", &self.positional_arguments)
                    .field("action", &"<function>")
                    .field("sub_command", &self.sub_command)
                    .finish()
            }
        }

        impl PartialEq for Command {
            fn eq(&self, other: &Self) -> bool {
                self.name == other.name
                    && self.short_description == other.short_description
                    && self.long_description == other.long_description
                    && self.positional_arguments == other.positional_arguments
                    && self.sub_command == other.sub_command
            }
        }

        impl Command {
            pub fn new(name: impl Into<String>) -> Self {
                Self {
                    name: name.into(),
                    ..Default::default()
                }
            }

            pub fn from(cmd: Command) -> Self {
                Self {
                    name: cmd.name,
                    short_description: cmd.short_description,
                    long_description: cmd.long_description,
                    positional_arguments: cmd.positional_arguments,
                    action: cmd.action,
                    sub_command: cmd.sub_command,
                }
            }

            pub fn short_description(mut self, content: impl Into<String>) -> Self {
                self.short_description = Some(content.into());
                self
            }

            pub fn long_description(mut self, content: impl Into<String>) -> Self {
                self.long_description = Some(content.into());
                self
            }

            pub fn description(
                mut self,
                long_description: impl Into<String>,
                short_description: impl Into<String>,
            ) -> Self {
                self.long_description = Some(long_description.into());
                self.short_description = Some(short_description.into());
                self
            }

            pub fn argument(mut self, name: impl Into<String>, required: bool) -> Self {
                self.positional_arguments.push(PositionalArgument {
                    name: name.into(),
                    required,
                });
                self
            }

            pub fn with_action<F, E>(mut self, action: F) -> Self
            where
                F: Fn(&[String]) -> Result<(), E> + 'static,
                E: Error + 'static,
            {
                self.action = Some(Box::new(move |args| {
                    action(args).map_err(|e| Box::new(e) as Box<dyn Error>)
                }));
                self
            }

            pub fn sub_command(mut self, cmd: Command) -> Self {
                self.sub_command.push(cmd);
                self
            }
        }
    }

    pub mod positional_arg {
        #[derive(Debug, PartialEq)]
        pub struct PositionalArgument {
            pub name: String,
            pub required: bool,
        }
    }

    mod flag {
        pub struct Flag {
            pub name: String,
            pub short_name: Option<String>,
            pub short_description: Option<String>,
            pub long_description: Option<String>,
        }
    }
}

#[cfg(test)]
mod tests;
