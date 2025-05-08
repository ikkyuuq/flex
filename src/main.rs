use args::command::Command;

fn main() {
    let args = vec!["hello".to_string()];
    let mut init = Command::default("init");

    let mut start = Command::default("start");
    let mut stop = Command::default("stop");

    let mut validate = Command::default("validate");
    let mut validate_repo = Command::default("repo");
    validate_repo.description(
        Some("Validate the repository that registered to flex".to_string()),
        None,
    );
    let mut validate_email = Command::default("email");
    validate_email.description(
        Some("Validate email wheater is connected to github".to_string()),
        None,
    );
    validate
        .description(Some("validate command description".to_string()), None)
        .subcommand(validate_repo)
        .subcommand(validate_email)
        .help();

    let mut add_repo = Command::default("add-repo");

    init.help();

    println!("{}", validate.run(&args).unwrap())
}

mod app {}

mod args {
    pub mod command {
        use std::{
            error::Error,
            fmt::{Debug, Display},
        };

        use super::{flag::Flag, positional_arg::PositionalArgument};

        type Action = Box<dyn Fn(&[String]) -> Result<String, Box<dyn Error>> + 'static>;

        #[derive(Default)]
        pub struct FlexCommand {
            pub name: String,
            pub s_desc: Option<String>,
            pub l_desc: Option<String>,
            pub action: Option<Action>,
            pub sub_commands: Vec<Command>,
            pub args: Vec<PositionalArgument>,
            pub flags: Vec<Flag>,
        }

        impl Clone for FlexCommand {
            fn clone(&self) -> Self {
                Self {
                    name: self.name.clone(),
                    s_desc: self.s_desc.clone(),
                    l_desc: self.l_desc.clone(),
                    action: None,
                    sub_commands: self.sub_commands.clone(),
                    args: self.args.clone(),
                    flags: self.flags.clone(),
                }
            }
        }

        impl PartialEq for FlexCommand {
            fn eq(&self, other: &Self) -> bool {
                self.name == other.name
                    && self.s_desc == other.s_desc
                    && self.l_desc == other.l_desc
                    && self.sub_commands == other.sub_commands
                    && self.args == other.args
                    && self.flags == other.flags
            }
        }

        impl Debug for FlexCommand {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("FlexCommand")
                    .field("name", &self.name)
                    .field("s_desc", &self.s_desc)
                    .field("l_desc", &self.l_desc)
                    .field("action", &"<function>")
                    .field("sub_commands", &self.sub_commands)
                    .field("args", &self.args)
                    .field("flags", &self.flags)
                    .finish()
            }
        }

        #[derive(Debug)]
        pub enum CommandError {
            Error(String),
        }

        impl Display for CommandError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    CommandError::Error(e) => {
                        write!(f, "{}", e)
                    }
                }
            }
        }

        impl Error for CommandError {}

        #[derive(Debug, Clone, PartialEq)]
        pub enum Command {
            /// Create a command with Default
            ///
            /// Including action by default and modify later with method
            /// `with_action` to perform custom behavior of the default command
            /// also able to add others properties like descriptions, sub_commands,
            /// positional_args, and flags
            ///
            /// If added sub_commands, and also implemented `with_action` it's
            /// need to be manual provided `help` method to the command
            /// to add help as a sub_command and perform an action with
            /// args(if exists and args doesn't match with the sub_commands)
            ///
            /// # Example
            /// ```
            /// // You can just create a Command with default pass in
            /// // the name of the command and it's ready to use
            /// app::default().add_command(Command::Default{ name: "greet".into() });
            ///
            /// let args = vec!["greet"]
            /// assert_eq!(app.run(&args), "greet command is called (default)".to_string())
            /// ````
            Default {
                name: String,
            },
            Flex(FlexCommand),
        }

        impl Command {
            pub fn default(name: impl Into<String>) -> Self {
                Command::Default { name: name.into() }
            }

            pub fn flex(cmd: FlexCommand) -> Self {
                Command::Flex(FlexCommand {
                    name: cmd.name,
                    s_desc: cmd.s_desc,
                    l_desc: cmd.l_desc,
                    action: cmd.action,
                    sub_commands: cmd.sub_commands,
                    args: cmd.args,
                    flags: cmd.flags,
                })
            }

            pub fn subcommand(&mut self, subcmd: Command) -> &mut Self {
                match self {
                    Command::Default { name } => {
                        *self = Command::Flex(FlexCommand {
                            name: name.to_string(),
                            sub_commands: vec![subcmd],
                            ..Default::default()
                        });
                    }
                    Command::Flex(cmd) => {
                        cmd.sub_commands.push(subcmd);
                    }
                }
                self
            }

            pub fn arg(&mut self, arg: PositionalArgument) -> &mut Self {
                match self {
                    Command::Default { name } => {
                        *self = Command::Flex(FlexCommand {
                            name: name.to_string(),
                            args: vec![arg],
                            ..Default::default()
                        });
                    }
                    Command::Flex(cmd) => {
                        cmd.args.push(arg);
                    }
                }
                self
            }

            pub fn flag(&mut self, flag: Flag) -> &mut Self {
                match self {
                    Command::Default { name } => {
                        *self = Command::Flex(FlexCommand {
                            name: name.to_string(),
                            flags: vec![flag],
                            ..Default::default()
                        });
                    }
                    Command::Flex(cmd) => {
                        cmd.flags.push(flag);
                    }
                }
                self
            }

            pub fn description(
                &mut self,
                s_desc: Option<String>,
                l_desc: Option<String>,
            ) -> &mut Self {
                match self {
                    Command::Default { name } => {
                        *self = Command::Flex(FlexCommand {
                            name: name.to_string(),
                            s_desc,
                            l_desc,
                            ..Default::default()
                        });
                    }
                    Command::Flex(cmd) => {
                        cmd.s_desc = s_desc;
                        cmd.l_desc = l_desc;
                    }
                }
                self
            }

            fn get_command_name(&self) -> String {
                match &self {
                    Command::Default { name } => name.clone().to_string(),
                    Command::Flex(cmd) => cmd.name.clone().to_string(),
                }
            }

            fn get_short_description(&self) -> String {
                match &self {
                    Command::Default { name } => "".to_string(),
                    Command::Flex(cmd) => cmd.s_desc.clone().unwrap_or("".to_string()),
                }
            }

            fn get_long_description(&self) -> String {
                match &self {
                    Command::Default { name } => "".to_string(),
                    Command::Flex(cmd) => cmd.l_desc.clone().unwrap_or("".to_string()),
                }
            }

            fn get_available_commands(&self) -> Vec<Command> {
                match &self {
                    Command::Default { name } => Vec::new(),
                    Command::Flex(cmd) => cmd.sub_commands.clone(),
                }
            }

            fn help_default(&self) -> Command {
                let cmd = self.get_command_name().to_string();
                let cmd_short_description = self.get_short_description();
                let available_commands = self.get_available_commands();
                Command::Flex(FlexCommand {
                    name: "help".to_string(),
                    action: Some(Box::new(
                        move |_: &[String]| -> Result<String, Box<dyn Error>> {
                            let mut commands_list = String::new();
                            commands_list.push_str("Available Commands:\n");
                            commands_list.push_str("  help             Display this message\n");
                            for cmd in &available_commands {
                                let cmd_name = cmd.get_command_name();
                                let cmd_desc = cmd.get_short_description();
                                commands_list
                                    .push_str(&format!("  {:<16} {}\n", cmd_name, cmd_desc))
                            }
                            Ok(format!(
                                r#"
usage: {} <command> [<agrs>]
    {cmd_short_description}

{}"#,
                                cmd,
                                commands_list.trim(),
                            ))
                        },
                    )),
                    ..Default::default()
                })
            }

            pub fn help(&mut self) -> &mut Self {
                let help_default = self.help_default();
                match self {
                    Command::Default { name } => {
                        *self = Command::Flex(FlexCommand {
                            name: name.to_string(),
                            sub_commands: vec![help_default],
                            ..Default::default()
                        });
                    }
                    Command::Flex(cmd) => {
                        cmd.sub_commands.push(help_default);
                    }
                }
                self
            }

            pub fn action<F, E>(&mut self, action: F) -> &mut Self
            where
                F: Fn(&[String]) -> Result<String, E> + 'static,
                E: Error + 'static,
            {
                let new_action = Box::new(move |args: &[String]| {
                    action(args).map_err(|e| Box::new(e) as Box<dyn Error>)
                });
                match self {
                    Command::Default { name } => {
                        *self = Command::Flex(FlexCommand {
                            name: name.to_string(),
                            action: Some(new_action),
                            ..Default::default()
                        });
                    }
                    Command::Flex(cmd) => {
                        cmd.action = Some(new_action);
                    }
                }
                self
            }

            pub fn run(&self, args: &[String]) -> Result<String, Box<dyn Error>> {
                let parent_cmd = self.get_command_name().to_string();
                let default_action =
                    Box::new(move |_: &[String]| -> Result<String, Box<dyn Error>> {
                        Ok(format!(
                            r#"
`{}` command is called (default)

You can modify your command behavior with `action()`
to customize, and replace this messages,
or using `help()` to generate `help` as sub command

If your command have `no action` to perform it going to
display this mesages, however If this command have 
`help` as sub command it will going to display 
help messages instead of (default)
                            "#,
                            parent_cmd
                        )
                        .trim()
                        .to_string())
                    });

                match self {
                    Command::Default { name } => {
                        let cmd = Command::Flex(FlexCommand {
                            name: name.to_string(),
                            action: Some(default_action),
                            ..Default::default()
                        });

                        cmd.run(args)
                    }
                    Command::Flex(cmd) => match &cmd.action {
                        Some(act) => act(args),
                        None => {
                            if !cmd.sub_commands.is_empty() {
                                for subcmd in &cmd.sub_commands {
                                    if subcmd.get_command_name().to_string() == "help".to_string() {
                                        return subcmd.run(args);
                                    }
                                }
                            }

                            let new_cmd = Command::Flex(FlexCommand {
                                name: cmd.name.to_string(),
                                action: Some(default_action),
                                s_desc: cmd.s_desc.clone(),
                                l_desc: cmd.l_desc.clone(),
                                sub_commands: cmd.sub_commands.clone(),
                                args: cmd.args.clone(),
                                flags: cmd.flags.clone(),
                            });

                            new_cmd.run(args)
                        }
                    },
                }
            }
        }
    }

    pub mod positional_arg {
        #[derive(Debug, Clone, PartialEq)]
        pub struct PositionalArgument {
            pub name: String,
            pub required: bool,
        }
    }

    mod flag {
        #[derive(Debug, Clone, PartialEq)]
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
