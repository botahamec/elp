/* ELP GIT HELPER
 * Elp is a command-line tool to abstract away many of the tedious commands
 * required to use git. Only one command is required to push to a repository
 * and only one command is required to link the local repository to a GitHub
 * repository.
*/

extern crate clap; // this crate is used to hand command-line arguments
use clap::{Arg, App, SubCommand};
use std::process::Command; // used to run git commands

// a function to more easily call a git command
fn git(args: &[&str], error: &str) {
	Command::new("git").args(args).output().expect(error);
}

// creates a local repository
fn init() {
	git(&["init"], "failed to intialize the repository");
	git(&["add", "."], "failed to add files to the local repository");
}

// links a repository to github using a url
fn start(url: &str) {
	git(&["commit", "-a", "-m", "\"First commit\""], "Failed to commit");
	git(&["remote", "add", "origin", url], "Failed to add the origin");
	git(&["push", "origin", "master"], "Failed to push to the remote repo");
}

// add files, commits them, and pushes (with a message)
fn push_message(title: &str, message: &str) {
	git(&["add", "."], "failed to add files to the local repository");
	git(&["commit", "-a", "-m", title, "-m", message], "Failed to commit");
	git(&["push", "origin", "master"], "Failed to push to the remote repo");
}

// add files, commits them, and pushes (without a message)
fn push(title: &str) {
	git(&["add", "."], "failed to add files to the local repository");
	git(&["commit", "-a", "-m", title], "Failed to commit");
	git(&["push", "origin", "master"], "Failed to push to the remote repo");
}

// pulls from remote repository
fn pull() {
	git(&["pull", "origin", "master"], "Failed to pull from the remote repo");
}

fn main() {

	// creates the cli application
	let matches = App::new("Elp Git Helper")
		.version("1.0.0")
		.author("Mike White <botahamec@outlook.com>")
		.about("A helper for git to simplify many mundane tasks")

		// the start command
		.subcommand(SubCommand::with_name("start")
			.about("Create a git repository at an optional specified url")
			.arg(Arg::with_name("url")
				.short("u")
				.long("remote repository url")
				.value_name("URL")
				.help("A link to the remote repository on Github.")))
		
		// the push command
		.subcommand(SubCommand::with_name("push")
			.about("Automatically add, commit, and push the repository")
			.arg(Arg::with_name("title")
				.short("t")
				.long("commit title")
				.value_name("TITLE")
				.required(true)
				.help("The title of the commit message. Simply, a description of what you did"))
			.arg(Arg::with_name("message")
				.short("m")
				.long("commit message")
				.value_name("MESS")
				.help("An optional description of the what you did before pushing")))
		
		// the pull command
		.subcommand(SubCommand::with_name("pull")
			.about("Automatically pull from the repository"))
			
		.get_matches();
	
	// runs the specified command
	if let Some(matches) = matches.subcommand_matches("start") {
		init();
		if matches.is_present("url") {start(matches.value_of("URL").unwrap());}
	}
	if let Some(matches) = matches.subcommand_matches("push") {
		if matches.is_present("message") {
			push_message(matches.value_of("TITLE").unwrap(), matches.value_of("MESS").unwrap());
		} else {push(matches.value_of("TITLE").unwrap());}
	}
	if let Some(_matches) = matches.subcommand_matches("pull") {pull();}
}
