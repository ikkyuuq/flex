use std::error::Error;

use crate::{
    app::{self, App, AppError},
    args::{command::Command, positional_arg::PositionalArgument},
};

#[test]
fn test_new_app() {
    let app = App::new("test cli");

    assert_eq!(
        app,
        App {
            name: "test cli".to_string(),
            ..Default::default()
        }
    )
}

#[test]
fn test_new_app_failed() {
    let app = App::new("failed cli");

    assert_ne!(
        app,
        App {
            name: "test cli".to_string(),
            ..Default::default()
        }
    )
}

#[test]
fn test_add_command_with_new() {
    let app = App::new("test cli");

    let init_cmd = Command::new("init");

    // TODO: APP MUST NOT MOVE
    let app = app.add_command(init_cmd);

    assert!(app.command.contains(&Command {
        name: "init".to_string(),
        ..Default::default()
    }))
}

#[test]
fn test_add_command_with_from() {
    let app = App::new("test cli").add_command(Command::from(Command {
        name: "init".to_string(),
        short_description: Some("command to init cli".to_string()),
        ..Default::default()
    }));

    let another_command = Command {
        name: "another-command".to_string(),
        ..Default::default()
    };

    // TODO: APP MUST NOT MOVE
    let app = app.add_command(another_command);

    assert_eq!(
        app.command,
        vec![
            Command {
                name: "init".to_string(),
                short_description: Some("command to init cli".to_string()),
                ..Default::default()
            },
            Command {
                name: "another-command".to_string(),
                ..Default::default()
            }
        ]
    )
}

#[test]
fn test_add_properties_to_command() {
    let test = Command::from(Command {
        name: "test".to_string(),
        short_description: Some("test command description".to_string()),
        ..Default::default()
    })
    .sub_command(Command {
        name: "sub-test".to_string(),
        ..Default::default()
    });

    assert_eq!(
        test,
        Command {
            name: "test".to_string(),
            short_description: Some("test command description".to_string()),
            sub_command: vec![Command {
                name: "sub-test".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        }
    );

    let test = Command::new("test").argument("arg-test".to_string(), false);

    assert_eq!(
        test,
        Command {
            name: "test".to_string(),
            positional_arguments: vec![PositionalArgument {
                name: "arg-test".to_string(),
                required: false
            }],
            ..Default::default()
        }
    );

    let test = Command::new("test").description("long description", "short description");

    assert_eq!(
        test,
        Command {
            name: "test".to_string(),
            short_description: Some("short description".to_string()),
            long_description: Some("long description".to_string()),
            ..Default::default()
        }
    )
}

#[test]
fn test_err_unknown_command() {
    let args = vec!["not-test".to_string(), "something".to_string()];

    let app = App::new("test-app").add_command(Command::new("test"));

    // TODO: APP MUST NOT MOVE
    let binding = app.run(&args);
    let app = binding.as_ref();

    assert!(app.is_err());
    assert_eq!(
        app.expect_err("Expected an error, why not!").to_string(),
        "Unknown Command Provided: not-test"
    );
}

#[test]
fn test_err_not_provided_command() {
    let args = vec![];

    let app = App::new("test-app").add_command(Command::new("test"));

    // TODO: APP MUST NOT MOVE
    let binding = app.run(&args);
    let app = binding.as_ref();

    assert!(app.is_err());
    assert_eq!(
        app.expect_err("Expected an error, why not!").to_string(),
        "Command is not provided"
    );
}

#[test]
fn test_good_result() {
    // TODO: I don't even know yet what the command going to return from their action
}
