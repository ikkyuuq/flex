use app::App;
use args::{
    arg::Arg,
    command::{Command, CommandError, Flex},
    flag::Flag,
};
use std::env::args;

fn main() {
    let init = Command::default("init").action(|args| -> Result<String, CommandError> {Ok(format!("init {:?}", args))});
    let start = Command::default("start");
    let stop = Command::default("stop").description("Stop command");
    let validate = Command::default("validate")
        .description("Validate command description")
        .subcommand(
            Command::default("email")
                .description("Validate email whether is connected to github")
                .help(),
        )
        .help();

    let path = Arg::new("path")
        .description("local repository path")
        .required();
    let f_all = Flag::new("all")
        .short("a")
        .description("include all repositories");
    let add_repo = Command::default("repo")
        .subcommand(
            Command::default("add")
                .description("add repository path to track")
                .arg(path.clone())
                .flag(f_all.clone())
                .help(),
        )
        .subcommand(
            Command::default("remove")
                .description("add repository path to track")
                .arg(path.clone())
                .flag(f_all.clone())
                .help(),
        )
        .help();

    let app = App::new("flex")
        .about(
            "Flexing CLI tool to tracking your git comit
even on the local project that didn't create repository on github
and keep getting contribution/streak and summarized the commit messages 
keep you able what have you done on multiple projects",
        )
        .add_commands(vec![init, start, stop, validate, add_repo])
        .help();

    match app.run() {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("Error: {}", e),
    }
}

mod app {
    use std::{
        env::args, error::Error, fmt::{Debug, Display}
    };

    use crate::args::{command::{Command, Flex, FlexCommand}};

    #[derive(Debug)]
    pub enum AppError {
        MissingCommand,
        InvalidCommand(String),
        InvalidConfiguration(String),
    }

    impl Display for AppError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                AppError::MissingCommand => write!(f, "No command provided"),
                AppError::InvalidCommand(e) => write!(f, "Unknown command: {}", e),
                AppError::InvalidConfiguration(e) => write!(f, "Invalid configuration: {}", e),
            }
        }
    }

    impl Error for AppError {}

    #[derive(Default, Debug)]
    pub struct App {
        pub name: String,
        pub about: String,
        pub commands: Vec<Command>,
    }

    impl App {
        pub fn new(name: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                ..Default::default()
            }
        }

        pub fn about(mut self, about: impl Into<String>) -> Self {
            self.about = about.into();
            self
        }

        pub fn add_commands(mut self, commands: Vec<Command>) -> Self {
            self.commands = commands;
            self
        }

        pub fn add_command(mut self, cmd: Command) -> Self {
            self.commands.push(cmd);
            self
        }

        pub fn help(mut self) -> Self {
            let app_name = self.name.clone();
            let app_about = self.about.clone();
            let commands = self.commands.clone();
            self.commands.push(Command::flex(FlexCommand {
                name: "help".to_string(),
                action: Some(Box::new(move |_: &[String]| {
                    let mut output = format!("Usage: {} <command> [<args>]\n", app_name);
                    if !app_about.is_empty() {
                        output.push_str(&format!("{}\n\n", app_about));
                    }
                    if !commands.is_empty() {
                        output.push_str("Available Commands:\n");
                        for cmd in &commands {
                            let cmd_name = cmd.get_cmd_name();
                            let cmd_desc = cmd.get_cmd_description();
                            output.push_str(&format!("  {:<16} {}\n", cmd_name, cmd_desc));
                        }
                    }
                    Ok(output.trim().to_string())
                })),
                ..Default::default()
            }));
            self
        }

        pub fn run(&self) -> Result<String, Box<dyn Error>> {
            let args: Vec<String> = args().skip(1).collect();
            if args.is_empty() || args.first().is_some_and(|arg| arg == "help") {
                for cmd in &self.commands {
                    if cmd.get_cmd_name() == "help" {
                        return cmd.run(&[]);
                    }
                }
                return Err(Box::new(AppError::InvalidConfiguration(
                    "No help command defined. Add with `.help()`".to_string(),
                )));
            }

            let command_name = &args[0];
            let command_args = &args[1..];

            for cmd in &self.commands {
                if cmd.get_cmd_name() == *command_name {
                    return cmd.run(command_args);
                }
            }

            Err(Box::new(AppError::InvalidCommand(format!(
                "Unknown command: {}",
                command_name
            ))))
        }
    }
}

mod args {
    pub mod command {
        use std::{
            error::Error,
            fmt::{Debug, Display},
        };

        use super::{arg::Arg, flag::Flag};

        type Action = Box<dyn Fn(&[String]) -> Result<String, Box<dyn Error>> + 'static>;

        #[derive(Default)]
        pub struct FlexCommand {
            pub name: String,
            pub desc: String,
            pub action: Option<Action>,
            pub sub_commands: Vec<Command>,
            pub args: Vec<Arg>,
            pub flags: Vec<Flag>,
        }

        impl Debug for FlexCommand {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("FlexCommand")
                    .field("name", &self.name)
                    .field("desc", &self.desc)
                    .field("action", &"<function>")
                    .field("sub_commands", &self.sub_commands)
                    .field("args", &self.args)
                    .field("flags", &self.flags)
                    .finish()
            }
        }

        impl Clone for FlexCommand {
            fn clone(&self) -> Self {
                Self {
                    name: self.name.clone(),
                    desc: self.desc.clone(),
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
                    && self.desc == other.desc
                    && self.sub_commands == other.sub_commands
                    && self.args == other.args
                    && self.flags == other.flags
            }
        }

        #[derive(Debug)]
        pub enum CommandError {
            InvalidCommand(String),
            InvalidArgument(String),
            MissingSubcommand(String),
            InvalidConfiguration(String),
        }

        impl Display for CommandError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    CommandError::InvalidCommand(e) => write!(f, "Unknown subcommand: {}", e),
                    CommandError::InvalidArgument(e) => write!(f, "Invalid argument: {}", e),
                    CommandError::MissingSubcommand(e) => write!(f, "Missing subcommand: {}", e),
                    CommandError::InvalidConfiguration(e) => {
                        write!(f, "Invalid configuration: {}", e)
                    }
                }
            }
        }

        impl Error for CommandError {}

        #[derive(Debug, Clone, PartialEq)]
        pub enum Command {
            Default { name: String },
            Flex(FlexCommand),
        }

        impl Default for Command {
            fn default() -> Self {
                Command::Default {
                    name: String::new(),
                }
            }
        }

        pub trait Flex {
            fn description(self, desc: impl Into<String>) -> Self;
            fn subcommand(self, subcmd: Command) -> Self;
            fn action<F, E>(self, action: F) -> Self
            where
                F: Fn(&[String]) -> Result<String, E> + 'static,
                E: Error + 'static;
            fn arg(self, arg: Arg) -> Self;
            fn flag(self, flag: Flag) -> Self;
            fn help(self) -> Self;
            fn run(&self, args: &[String]) -> Result<String, Box<dyn Error>>;
        }

        impl Flex for Command {
            fn description(self, desc: impl Into<String>) -> Self {
                self.flex_with(|cmd| {
                    cmd.desc = desc.into();
                })
            }

            fn subcommand(self, subcmd: Command) -> Self {
                self.flex_with(|cmd| {
                    cmd.sub_commands.push(subcmd);
                })
            }

            fn action<F, E>(self, action: F) -> Self
            where
                F: Fn(&[String]) -> Result<String, E> + 'static,
                E: Error + 'static,
            {
                let new_action = Box::new(move |args: &[String]| {
                    action(args).map_err(|e| Box::new(e) as Box<dyn Error>)
                });
                self.flex_with(|cmd| {
                    cmd.action = Some(new_action);
                })
            }

            fn arg(self, arg: Arg) -> Self {
                self.flex_with(|cmd| {
                    cmd.args.push(arg);
                })
            }

            fn flag(self, flag: Flag) -> Self {
                self.flex_with(|cmd| {
                    cmd.flags.push(flag);
                })
            }

            fn help(self) -> Self {
                let help_default = self.help_default();
                self.flex_with(|cmd| {
                    cmd.sub_commands.push(help_default);
                })
            }

            fn run(&self, args: &[String]) -> Result<String, Box<dyn Error>> {
                run_command(self, args)
            }
        }

        impl Flex for &mut Command {
            fn description(self, desc: impl Into<String>) -> Self {
                self.flex_mut_with(|cmd| {
                    cmd.desc = desc.into();
                });
                self
            }

            fn subcommand(self, subcmd: Command) -> Self {
                self.flex_mut_with(|cmd| {
                    cmd.sub_commands.push(subcmd);
                });
                self
            }

            fn action<F, E>(self, action: F) -> Self
            where
                F: Fn(&[String]) -> Result<String, E> + 'static,
                E: Error + 'static,
            {
                let new_action = Box::new(move |args: &[String]| {
                    action(args).map_err(|e| Box::new(e) as Box<dyn Error>)
                });
                self.flex_mut_with(|cmd| {
                    cmd.action = Some(new_action);
                });
                self
            }

            fn arg(self, arg: Arg) -> Self {
                self.flex_mut_with(|cmd| {
                    cmd.args.push(arg);
                });
                self
            }

            fn flag(self, flag: Flag) -> Self {
                self.flex_mut_with(|cmd| {
                    cmd.flags.push(flag);
                });
                self
            }

            fn help(self) -> Self {
                let help_default = self.help_default();
                self.flex_mut_with(|cmd| {
                    cmd.sub_commands.push(help_default);
                });
                self
            }

            fn run(&self, args: &[String]) -> Result<String, Box<dyn Error>> {
                run_command(self, args)
            }
        }

        impl Command {
            pub fn default(name: impl Into<String>) -> Self {
                Command::Default { name: name.into() }
            }

            pub fn flex(cmd: FlexCommand) -> Self {
                Command::Flex(cmd)
            }

            fn flex_with<F>(self, f: F) -> Self
            where
                F: FnOnce(&mut FlexCommand),
            {
                match self {
                    Command::Default { name } => {
                        let mut cmd = FlexCommand {
                            name: name.to_string(),
                            ..Default::default()
                        };
                        f(&mut cmd);
                        Command::Flex(cmd)
                    }
                    Command::Flex(mut cmd) => {
                        f(&mut cmd);
                        Command::Flex(cmd)
                    }
                }
            }

            fn flex_mut_with<F>(&mut self, f: F)
            where
                F: FnOnce(&mut FlexCommand),
            {
                *self = match std::mem::take(self) {
                    Command::Default { name } => {
                        let mut cmd = FlexCommand {
                            name: name.to_string(),
                            ..Default::default()
                        };
                        f(&mut cmd);
                        Command::Flex(cmd)
                    }
                    Command::Flex(mut cmd) => {
                        f(&mut cmd);
                        Command::Flex(cmd)
                    }
                };
            }

            pub fn get_cmd_name(&self) -> String {
                match self {
                    Command::Default { name } => name.clone(),
                    Command::Flex(cmd) => cmd.name.clone(),
                }
            }

            pub fn get_cmd_description(&self) -> String {
                match self {
                    Command::Default { .. } => "".to_string(),
                    Command::Flex(cmd) => cmd.desc.clone(),
                }
            }

            pub fn get_available_cmds(&self) -> Vec<Command> {
                match self {
                    Command::Default { .. } => Vec::new(),
                    Command::Flex(cmd) => cmd.sub_commands.clone(),
                }
            }

            pub fn get_args(&self) -> Vec<Arg> {
                match self {
                    Command::Default { .. } => Vec::new(),
                    Command::Flex(cmd) => cmd.args.clone(),
                }
            }

            pub fn get_flags(&self) -> Vec<Flag> {
                match self {
                    Command::Default { .. } => Vec::new(),
                    Command::Flex(cmd) => cmd.flags.clone(),
                }
            }

            fn help_default(&self) -> Command {
                let cmd_name = self.get_cmd_name();
                let cmd_description = self.get_cmd_description();
                let available_commands = self.get_available_cmds();
                let args = self.get_args();
                let flags = self.get_flags();

                Command::flex(FlexCommand {
                    name: "help".to_string(),
                    action: Some(Box::new(move |_: &[String]| {
                        let mut output =
                            format!("Usage: {} {} <subcommand> [<args>]\n", "flex", cmd_name);
                        if !cmd_description.is_empty() {
                            output.push_str(&format!("{}\n\n", cmd_description));
                        }
                        output.push_str("Available Subcommands:\n");
                        for cmd in &available_commands {
                            let cmd_name = cmd.get_cmd_name();
                            let cmd_desc = cmd.get_cmd_description();
                            output.push_str(&format!("  {:<16} {}\n", cmd_name, cmd_desc));
                        }
                        if !args.is_empty() {
                            output.push_str("\nArguments:\n");
                            for arg in &args {
                                output.push_str(&format!(
                                    "  {:<16} {}\n",
                                    arg.name,
                                    if arg.required {
                                        "(required)"
                                    } else {
                                        "(optional)"
                                    }
                                ));
                            }
                        }
                        if !flags.is_empty() {
                            output.push_str("\nFlags:\n");
                            for flag in &flags {
                                let flag_desc = flag.desc.clone();
                                output.push_str(&format!(
                                    "  --{:<14} -{:<7} {}\n",
                                    flag.name, flag.short, flag_desc
                                ));
                            }
                        }
                        Ok(output.trim().to_string())
                    })),
                    ..Default::default()
                })
            }
        }

        fn run_command(cmd: &Command, args: &[String]) -> Result<String, Box<dyn Error>> {
            let parent_cmd = cmd.get_cmd_name();
            let default_action = Box::new(move |_: &[String]| {
                Ok(format!(
                    "Command '{}' called (default). Use `action()` to customize or `help()` to add a help subcommand.",
                    parent_cmd
                ))
            });

            match cmd {
                Command::Default { name } => {
                    let flex_cmd = FlexCommand {
                        name: name.clone(),
                        action: Some(default_action),
                        ..Default::default()
                    };
                    Command::Flex(flex_cmd).run(args)
                }
                Command::Flex(flex_cmd) => {
                    if flex_cmd.action.is_some() {
                        let required_args = flex_cmd.args.iter().filter(|arg| arg.required).count();
                        if args.len() < required_args {
                            return Err(Box::new(CommandError::InvalidArgument(format!(
                                "Command '{}' requires {} arguments, got {}",
                                flex_cmd.name,
                                required_args,
                                args.len()
                            ))));
                        }
                    }

                    if !flex_cmd.sub_commands.is_empty() {
                        if args.is_empty() {
                            for sub_cmd in &flex_cmd.sub_commands {
                                if sub_cmd.get_cmd_name() == "help" {
                                    return sub_cmd.run(args);
                                }
                            }
                            return Err(Box::new(CommandError::MissingSubcommand(format!(
                                "Command '{}' requires a subcommand",
                                flex_cmd.name
                            ))));
                        }

                        let sub_command = &args[0];
                        let sub_command_args = &args[1..];
                        for sub_cmd in &flex_cmd.sub_commands {
                            if sub_cmd.get_cmd_name() == *sub_command {
                                return sub_cmd.run(sub_command_args);
                            }
                        }

                        if flex_cmd.action.is_none() {
                            return Err(Box::new(CommandError::InvalidCommand(format!(
                                "Unknown subcommand: {}",
                                sub_command
                            ))));
                        }
                    }

                    if let Some(action) = &flex_cmd.action {
                        return action(args);
                    }

                    for sub_cmd in &flex_cmd.sub_commands {
                        if sub_cmd.get_cmd_name() == "help" {
                            return sub_cmd.run(args);
                        }
                    }

                    default_action(args)
                }
            }
        }
    }

    pub mod arg {
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct Arg {
            pub name: String,
            pub desc: String,
            pub required: bool,
        }

        impl Arg {
            pub fn new(name: impl Into<String>) -> Self {
                Arg {
                    name: name.into(),
                    ..Default::default()
                }
            }

            pub fn flex(flag: Arg) -> Self {
                Self {
                    name: flag.name,
                    desc: flag.desc,
                    required: flag.required,
                }
            }

            pub fn description(mut self, content: impl Into<String>) -> Self {
                self.desc = content.into();
                Self {
                    name: self.name,
                    desc: self.desc,
                    required: self.required,
                }
            }

            pub fn required(mut self) -> Self {
                self.required = true;
                Self {
                    name: self.name,
                    desc: self.desc,
                    required: self.required,
                }
            }
        }
    }

    pub mod flag {
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct Flag {
            pub name: String,
            pub short: String,
            pub desc: String,
        }

        impl Flag {
            pub fn new(name: impl Into<String>) -> Self {
                Flag {
                    name: name.into(),
                    ..Default::default()
                }
            }

            pub fn flex(flag: Flag) -> Self {
                Self {
                    name: flag.name,
                    short: flag.short,
                    desc: flag.desc,
                }
            }

            pub fn description(mut self, content: impl Into<String>) -> Self {
                self.desc = content.into();
                Self {
                    name: self.name,
                    short: self.short,
                    desc: self.desc,
                }
            }

            pub fn short(mut self, short_name: impl Into<String>) -> Self {
                self.short = short_name.into();
                Self {
                    name: self.name,
                    short: self.short,
                    desc: self.desc,
                }
            }
        }
    }
}
