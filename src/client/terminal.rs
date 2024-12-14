use dialoguer::{console::Style, theme::Theme};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Post,
    Follow,
    Unfollow,
    Help,
    Quit,
    Delete, // Nip-09
    Get,
    Info, // Nip-11
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let command = match self {
            Command::Post => "post",
            Command::Follow => "follow",
            Command::Unfollow => "unfollow",
            Command::Help => "help",
            Command::Quit => "quit",
            Command::Delete => "delete",
            Command::Get => "get",
            Command::Info => "info",
        };

        write!(f, "{}", command)
    }
}

#[derive(Debug, Clone)]
pub struct TerminalInput {
    pub command: Command,
    pub argument: Option<String>,
}

impl fmt::Display for TerminalInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.argument {
            Some(argument) => write!(f, "{} {}", self.command, argument),
            None => write!(f, "{}", self.command),
        }
    }
}

impl FromStr for TerminalInput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, " ");
        let command = parts
            .next()
            .ok_or("no command was provided. enter `help` for a list of commands.")?;

        let command = match command {
            "post" => Command::Post,
            "follow" => Command::Follow,
            "unfollow" => Command::Unfollow,
            "delete" => Command::Delete,
            "get" => Command::Get,
            "help" => Command::Help,
            "quit" => Command::Quit,
            "info" => Command::Info,
            _ => return Err("invalid command. enter `help` for a list of commands.".to_string()),
        };

        let argument = parts.next().map(|s| s.trim().to_string());

        if (command != Command::Help
            && command != Command::Quit
            && command != Command::Delete
            && command != Command::Get
            && command != Command::Info)
            && argument.is_none()
        {
            return Err(
                "no argument was provided. enter `help` for a list of commands.".to_string(),
            );
        }

        Ok(Self { command, argument })
    }
}

pub struct SimplerTheme {
    prompt_color: Style,
    confirm_color: Style,
    error_color: Style,
}

impl Default for SimplerTheme {
    fn default() -> Self {
        Self {
            prompt_color: Style::new().for_stderr().green(),
            confirm_color: Style::new().for_stderr().dim(),
            error_color: Style::new().for_stderr().red(),
        }
    }
}

impl Theme for SimplerTheme {
    fn format_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
        write!(f, "{}", prompt)
    }

    fn format_error(&self, f: &mut dyn fmt::Write, err: &str) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.error_color.apply_to("> "),
            self.error_color.apply_to(err)
        )
    }

    fn format_input_prompt(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        default: Option<&str>,
    ) -> fmt::Result {
        match default {
            Some(default) if prompt.is_empty() => write!(f, "[{}]", default),
            Some(default) => write!(f, "{} [{}]", prompt, default),
            None => write!(f, "{}", self.prompt_color.apply_to(prompt)),
        }
    }

    fn format_input_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> fmt::Result {
        let parsed = TerminalInput::from_str(sel).map_err(|_| fmt::Error);
        let bold = Style::new().for_stderr().bold().dim();

        if let Ok(parsed) = parsed {
            if let Some(argument) = parsed.argument {
                write!(
                    f,
                    "{}{} {}",
                    self.prompt_color.apply_to(prompt),
                    bold.apply_to(parsed.command.to_string()),
                    self.confirm_color.apply_to(argument)
                )
            } else {
                write!(
                    f,
                    "{}{}",
                    self.prompt_color.apply_to(prompt),
                    bold.apply_to(parsed.command.to_string())
                )
            }
        } else {
            write!(
                f,
                "{}{}",
                self.prompt_color.apply_to(prompt),
                self.confirm_color.apply_to(sel)
            )
        }
    }
}
