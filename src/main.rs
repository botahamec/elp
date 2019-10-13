/* ELP GIT HELPER
 * Elp is a command-line tool to abstract away many of the tedious commands
 * required to use git. Only one command is required to push to a repository
 * and only one command is required to link the local repository to a GitHub
 * repository.
*/

extern crate clap; // this crate is used to hand command-line arguments
use clap::{Arg, App, SubCommand};
use std::process::{Command, Stdio}; // used to run git commands
 // used to get output from the commands

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

// runs the command in the given directory
fn command_in_dir(command: &str, args: &[&str], dir: &str, error: &str) {
	let mut command = Command::new(command)
		.args(args)
		.current_dir(dir)
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.spawn()
		.expect(error);
	command.wait().unwrap();
}

// a function to more easily call a git command
fn git(args: &[&str], error: &str) {
	command("git", args, error);
}

// links a repository to github using a url
fn start(url: Option<&str>) {
	git(&["init"], "failed to intialize the repository");
	git(&["add", "-A"], "failed to add files to the local repository");
	match url {
		Some(u) => {
			git(&["commit", "-a", "-m", "First commit"], "Failed to commit");
			git(&["remote", "add", "origin", u], "Failed to add the origin");
			git(&["push", "-u", "origin", "master"], "Failed to push to the remote repo");
		},
		None => return
	};
}

// add files, commits them, and pushes (with a message)
fn push(title: &str, message: Option<&str>) {
	git(&["add", "-A"], "failed to add files to the local repository");
	match message {
		Some(m) => git(&["commit", "-a", "-m", title, "-m", m], "Failed to commit"),
		None => git(&["commit", "-a", "-m", title], "Failed to commit")
	};
	git(&["push", "origin", "master"], "Failed to push to the remote repo");
}

// pulls from remote repository
fn pull() {
	git(&["pull", "origin", "master"], "Failed to pull from the remote repo");
}

// updates Elp
#[cfg(target_family = "windows")]
fn update() {
	command("update.cmd", &[], "Failed to update.")
}

// updates Elp
#[cfg(target_family = "unix")]
fn update() {
	command("sh", &["make_unix.sh"], "Failed to update.")
}

fn main() {

	// creates the cli application
	let matches = App::new("Elp Git Helper")
		.version("1.0.4")
		.author("Mike White <botahamec@outlook.com>")
		.about("A helper for git to simplify many mundane tasks")

		// the start command
		.subcommand(SubCommand::with_name("start")
			.about("Create a git repository at an optional specified url")
			.arg(Arg::with_name("url")
				.short("u")
				.long("remote-repository-url")
				.value_name("URL")
				.help("A link to the remote repository on Github.")))
		
		// the push command
		.subcommand(SubCommand::with_name("push")
			.about("Automatically add, commit, and push the repository")
			.arg(Arg::with_name("TITLE")
				.required(true)
				.help("The title of the commit message. Simply, a description of what you did"))
			.arg(Arg::with_name("message")
				.short("m")
				.long("commit-message")
				.value_name("MESSAGE")
				.help("An optional description of what you did before pushing")))
		
		// the pull command
		.subcommand(SubCommand::with_name("pull")
			.about("Automatically pull from the repository"))

		// the update command
		.subcommand(SubCommand::with_name("update"))
			.about("Clones from the master branch of the Elp repository and runs the make script")
			
		.get_matches();
	
	// runs the specified command
	if let Some(matches) = matches.subcommand_matches("start") {
		start(matches.value_of("url"));
	}
	if let Some(matches) = matches.subcommand_matches("push") {
		push(matches.value_of("TITLE").unwrap(), matches.value_of("message"));
	}
	if let Some(_matches) = matches.subcommand_matches("pull") {pull();}
	if let Some(_matches) = matches.subcommand_matches("update") {update();}
}
