fn main() {}

mod args {

    mod command {
        use std::error::Error;

        use super::positional_arg::PositionalArgument;

        #[derive(Debug)]
        pub enum CommandError {
            MissingArgument(String),
            InvalidCommand(String),
            ExecutionError(String),
        }

        pub struct Command {
            pub name: String,
            pub short_description: Option<String>,
            pub long_description: Option<String>,
            pub positional_arguments: Vec<PositionalArgument>,
            pub action: Option<Box<dyn Fn(&[String]) -> Result<(), Box<dyn Error>>>>,
            pub sub_command: Vec<Command>
        }

        impl Command {
            pub fn new(name: impl Into<String>) -> Self {
                Self {
                    name: name.into(),
                    short_description: None,
                    long_description: None,
                    positional_arguments: Vec::new(),
                    action: None,
                    sub_command: Vec::new()
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
                self.action = Some(Box::new(move |args| action(args).map_err(|e| Box::new(e) as Box<dyn Error>)));
                self
            }

            pub fn sub_command(mut self, cmd: Command) -> Self {
                self.sub_command.push(cmd);
                self
            }
        }
    }

    mod positional_arg {
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
            pub long_description: Option<String>
        }
    }
}
