/* ELP GIT HELPER
 * Elp is a command-line tool to abstract away many of the tedious commands
 * required to use git. Only one command is required to push to a repository
 * and only one command is required to link the local repository to a GitHub
 * repository.
*/

extern crate clap; // this crate is used to hand command-line arguments
use clap::{Arg, App, SubCommand};
use std::process::{Command, Stdio, exit}; // used to interact with command-line
use std::io::{stdin, stdout, Write}; // used to read user input


// runs a command and waits for it to finish
fn command(command: &str, args: &[&str], error: &str) {
	let mut command = Command::new(command)
		.args(args)
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.spawn()
		.expect(error);
	command.wait().unwrap();
}

// runs a command but doesn't wait for it to finish
fn command_spawn(command: &str, args: &[&str], error: &str) {
	Command::new(command)
		.args(args)
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.spawn()
		.expect(error);
}

// prompts the user for input
#[allow(unused_must_use)]
fn input(prompt: &str) -> String {
	let mut answer = String::new();
	print!("{} ", prompt);
	stdout().flush();
	stdin().read_line(&mut answer);
	answer = String::from(answer.trim_end());
	answer
}

// returns the output of a command
fn get_output(command: &str, args: &[&str], error: &str) -> String {
	let output = Command::new(command)
		.args(args)
		.output()
		.expect(error);
	let mut string = String::from_utf8(output.stdout).unwrap();
	string.pop();
	string
}

// a function to more easily call a git command
fn git(args: &[&str], error: &str) {
	command("git", args, error);
}

fn git_add(verbosity: usize, quiet: bool) {
	if quiet {git(&["add", "-A", "-q"], "failed to add files to the local repository");}
	else {
		match verbosity {
			0 => git(&["add", "-A"], "failed to add files to the local repository"),
			_ => git(&["add", "-A", "-v"], "failed to add files to the local repository")
		}
	}
}

fn git_commit(title: &str, message: Option<&str>, verbosity: usize, quiet: bool) {
	if quiet {
		match message {
			Some(m) => git(&["commit", "-a", "-q", "-m", title, "-m", m], "Failed to commit"),
			None => git(&["commit", "-a", "-q", "-m", title], "Failed to commit")
		};
	} else {
		match message {
			Some(m) => match verbosity {
				0 => git(&["commit", "-a", "-m", title, "-m", m], "Failed to commit"),
				1 => git(&["commit", "-a", "-v", "-m", title, "-m", m], "Failed to commit"),
				_ => git(&["commit", "-a", "-vv", "-m", title, "-m", m], "Failed to commit"),
			},
			None =>  match verbosity {
				0 => git(&["commit", "-a", "-m", title], "Failed to commit"),
				1 => git(&["commit", "-a", "-v", "-m", title], "Failed to commit"),
				_ => git(&["commit", "-a", "-vv", "-m", title], "Failed to commit"),
			}
		};
	}
}

fn add_origin(url: &str, verbosity: usize, quiet: bool) {
	if quiet {git(&["remote", "-q", "add", "origin", url], "Failed to add the origin");}
	else {
		match verbosity {
			0 => git(&["remote", "add", "origin", url], "Failed to add the origin"),
			_ => git(&["remote", "-v", "add", "origin", url], "Failed to add the origin"),
		};
	}
}

fn git_push(branch: Option<&str>, verbosity: usize, quiet: bool) {
	if quiet {git(&["push", "-q", "-u", "origin", get_branch(branch).as_str()], "Failed to push to the remote repo");}
	else {
		match verbosity {
			0 => git(&["push", "-u", "origin", get_branch(branch).as_str()], "Failed to push to the remote repo"),
			_ => git(&["push", "-v", "-u", "origin", get_branch(branch).as_str()], "Failed to push to the remote repo"),
		};
	}
}

fn git_pull(branch: Option<&str>, verbosity: usize, quiet: bool) {
	if quiet {git(&["pull", "-q", "origin", get_branch(branch).as_str()], "Failed to pull from the remote repository");}
	else {
		match verbosity {
			0 => git(&["pull", "origin", get_branch(branch).as_str()], "Failed to pull from the remote repo"),
			_ => git(&["pull", "-v", "origin", get_branch(branch).as_str()], "Failed to pull from the remote repo")
		};
	}
}

// takes an option and either uses the string given or the current branch
fn get_branch(branch: Option<&str>) -> String {
	match branch {
		Some(b) => String::from(b),
		None => get_output("git", &["branch", "--show-current"], "Could not get the current branch")
	}
}

// sets a git config option
fn set_config(prompt: &str, option: &str) {
	let value = input(prompt);
	if value == String::new() {return;} // stops if the string is empty
	let value_str = value.as_str();
	git(&["config", "--global", option, value_str], format!("Couldn't set {} properly", option).as_str());
}

// links a repository to github using a url
fn start(url: &str, branch: Option<&str>, verbosity: usize, quiet: bool) {
	git(&["init"], "failed to intialize the repository");
	git_add(verbosity, quiet);
	git_commit("First commit", None, verbosity, quiet);
	add_origin(url, verbosity, quiet);
	git_push(branch, verbosity, quiet);
}

// add files, commits them, and pushes (with a message)
fn push(title: Option<&str>, message: Option<&str>, branch: Option<&str>, commit: bool, verbosity: usize, quiet: bool) {
	let push_title : &str;
	if let Some(temp_title) = title {
		push_title = temp_title;
	} else {
		push_title = "Auto-commit"
	}
	git_add(verbosity, quiet);
	if commit {git_commit(push_title, message, verbosity, quiet);}
	git_push(branch, verbosity, quiet);
}

// pulls from remote repository
fn pull(branch: Option<&str>, verbosity: usize, quiet: bool) {
	git_pull(branch, verbosity, quiet)
}

// updates Elp
fn update(verbosity: usize) {
	if cfg!(target_family = "windows") {
		match verbosity {
			0 => command("update.cmd", &[], "Failed to update."),
			1 => command("vupdate.cmd", &[], "Failed to update."),
			_ => command("vvupdate.cmd", &[], "Failed to update."),
		};
	}
	else if cfg!(target_family = "unix") {
		git(&["clone", "https://github.com/Botahamec/elp.git"], "Unable to find latest version of Elp");
		match verbosity {
			0 => command_spawn("sh", &["elp/update.sh"], "Unable to run update script"),
			1 => command_spawn("sh", &["elp/vupdate.sh"], "Unable to run update script"),
			_ => command_spawn("sh", &["elp/vvupdate.sh"], "Unable to run update script"),
		};
		exit(0);
	}
}

// runs if no subcommand is supplied
fn elp_main(verbosity: usize, quiet: bool) {
	git_pull(None, verbosity, quiet);
	git_add(verbosity, quiet);
	git_commit("Auto-commit", None, verbosity, quiet);
	git_push(None, verbosity, quiet);
}

fn setup() {
	println!("Use Ctrl+C at any time to exit. Just press enter without typing to not set the option");
	set_config("Name:", "user.name");
	set_config("Email:", "user.email");
	set_config("Color (auto/false):", "color.ui");
	set_config("Editor for Commit Messages:", "core.editor");
}

fn main() {

	// creates the cli application
	let matches = App::new("Elp Git Helper")
		.version("1.1.2")
		.author("Mike White <botahamec@outlook.com>")
		.about("A helper for git to simplify many mundane tasks")

		// the verbose argument
		.arg(Arg::with_name("verbose")
			.short("v")
			.long("verbose")
			.multiple(true)
			.help("Prints out a lot of information"))

		// the quiet argument
		.arg(Arg::with_name("quiet")
			.short("q")
			.long("quiet")
			.help("Shows no output"))

		// the start command
		.subcommand(SubCommand::with_name("start")
			.about("Create a git repository at an optional specified url")
			.arg(Arg::with_name("url")
				.required(true)
				.help("A link to the remote repository on Github."))
			.arg(Arg::with_name("branch")
				.short("b")
				.long("branch")
				.value_name("BRANCH")
				.help("The branch to push to"))
			.arg(Arg::with_name("verbose")
				.short("v")
				.long("verbose")
				.help("Prints out a lot of information"))
			.arg(Arg::with_name("quiet")
				.short("q")
				.long("quiet")
				.help("Shows no output")))

		// the push command
		.subcommand(SubCommand::with_name("push")
			.about("Automatically add, commit, and push the repository")
			.arg(Arg::with_name("TITLE")
				.help("The title of the commit message. Simply, a description of what you did"))
			.arg(Arg::with_name("message")
				.short("m")
				.long("commit-message")
				.value_name("MESSAGE")
				.help("An optional description of what you did before pushing"))
			.arg(Arg::with_name("branch")
				.short("b")
				.long("branch")
				.value_name("BRANCH")
				.help("The branch to push to"))
			.arg(Arg::with_name("no-commit")
				.short("n")
				.long("no-commit")
				.help("Does not create a commit"))
			.arg(Arg::with_name("verbose")
				.short("v")
				.long("verbose")
				.help("Prints out a lot of information"))
			.arg(Arg::with_name("quiet")
				.short("q")
				.long("quiet")
				.help("Shows no output")))

		// the pull command
		.subcommand(SubCommand::with_name("pull")
			.about("Automatically pull from the repository")
			.arg(Arg::with_name("branch")
				.short("b")
				.long("branch")
				.value_name("BRANCH")
				.help("The branch to pull from"))
			.arg(Arg::with_name("verbose")
				.short("v")
				.long("verbose")
				.help("Prints out a lot of information"))
			.arg(Arg::with_name("quiet")
				.short("q")
				.long("quiet")
				.help("Shows no output")))

		// setup command
		.subcommand(SubCommand::with_name("setup")
			.about("Sets up Git options"))

		// the update command
		.subcommand(SubCommand::with_name("update"))
			.about("Clones from the master branch of the Elp repository and runs the make script")

		.get_matches();

	let verbosity = matches.occurrences_of("verbose") as usize;
	let quietness = matches.occurrences_of("quiet") as usize;
	if quietness > 0 && verbosity > 0 {panic!("Cannot be verbose and quiet at the same time!");}
	let quiet = quietness > 0;

	// runs the specified command
	if let Some(matches) = matches.subcommand_matches("start") {
		start(matches.value_of("url").unwrap(), matches.value_of("branch"), verbosity, quiet);
	} else if let Some(matches) = matches.subcommand_matches("push") {
		let commit = matches.occurrences_of("no-commit") == 0;
		push(matches.value_of("TITLE"), matches.value_of("message"), matches.value_of("branch"), commit, verbosity, quiet);
	} else if let Some(_matches) = matches.subcommand_matches("pull") {pull(matches.value_of("branch"), verbosity, quiet);}
	else if let Some(_matches) = matches.subcommand_matches("setup") {setup();}
	else if let Some(_matches) = matches.subcommand_matches("update") {update(verbosity);}
	else {elp_main(verbosity, quiet);}
}
