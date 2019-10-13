/* ELP GIT HELPER
 * Elp is a command-line tool to abstract away many of the tedious commands
 * required to use git. Only one command is required to push to a repository
 * and only one command is required to link the local repository to a GitHub
 * repository.
*/

extern crate clap; // this crate is used to hand command-line arguments
use clap::{Arg, App, SubCommand};
use std::process::{Command, Stdio}; // used to run git commands


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

// returns the output of a command
fn get_output(command: &str, args: &[&str], error: &str) -> String {
	let output = Command::new(command)
		.args(args)
		.output()
		.expect(error);
	String::from_utf8(output.stdout).unwrap()
}

// a function to more easily call a git command
fn git(args: &[&str], error: &str) {
	command("git", args, error);
}

// takes an option and either uses the string given or the current branch
fn get_branch(branch: Option<&str>) -> String {
	match branch {
		Some(b) => String::from(b),
		None => get_output("git", &["branch", "--show-current"], "Could not get the current branch")
	}
}

// links a repository to github using a url
fn start(url: &str, branch: Option<&str>, verbosity: usize) {
	git(&["init"], "failed to intialize the repository");
	match verbosity {
		0 => git(&["add", "-A"], "failed to add files to the local repository"),
		_ => git(&["add", "-A", "-v"], "failed to add files to the local repository")
	}
	match verbosity {
		0 => git(&["commit", "-a", "-m", "First commit"], "Failed to commit"),
		1 => git(&["commit", "-a", "-v", "-m", "First commit"], "Failed to commit"),
		_ => git(&["commit", "-a", "-vv", "-m", "First commit"], "Failed to commit"),
	};
	match verbosity {
		0 => git(&["remote", "add", "origin", url], "Failed to add the origin"),
		_ => git(&["remote", "-v", "add", "origin", url], "Failed to add the origin"),
	};
	git(&["push", "-u", "origin", get_branch(branch).as_str()], "Failed to push to the remote repo");
}

// add files, commits them, and pushes (with a message)
fn push(title: &str, message: Option<&str>, branch: Option<&str>, verbosity: usize) {
	match verbosity {
		0 => git(&["add", "-A"], "failed to add files to the local repository"),
		_ => git(&["add", "-A", "-v"], "failed to add files to the local repository")
	}
	match message {
		Some(m) => match verbosity {
			0 => git(&["commit", "-a", "-m", title, "-m", m], "Failed to commit"),
			1 => git(&["commit", "-a", "-v", "-m", title, "-m", m], "Failed to commit"),
			_ => git(&["commit", "-a", "-vv", "-m", title, "-m", m], "Failed to commit"),
		},
		None => git(&["commit", "-a", "-m", title], "Failed to commit")
	};
	match verbosity {
		0 => git(&["push", "origin", get_branch(branch).as_str()], "Failed to push to the remote repo"),
		_ => git(&["push", "-v", "origin", get_branch(branch).as_str()], "Failed to push to the remote repo"),
	};
}

// pulls from remote repository
fn pull(branch: Option<&str>, verbosity: usize) {
	match verbosity {
		0 => git(&["pull", "origin", get_branch(branch).as_str()], "Failed to pull from the remote repo"),
		_ => git(&["pull", "-v", "origin", get_branch(branch).as_str()], "Failed to pull from the remote repo")
	};
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
		match verbosity {
			0 => command("sh", &["update.sh"], "Failed to update."),
			1 => command("sh", &["vupdate.sh"], "Failed to update."),
			_ => command("sh", &["vvupdate.sh"], "Failed to update."),
		};
	}
}

fn main() {

	// creates the cli application
	let matches = App::new("Elp Git Helper")
		.version("1.0.4")
		.author("Mike White <botahamec@outlook.com>")
		.about("A helper for git to simplify many mundane tasks")

		// the verbose argument
		.arg(Arg::with_name("verbose")
			.short("v")
			.long("verbose")
			.multiple(true)
			.help("Prints out a lot of information"))

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
				.help("The branch to push to")))
		
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
				.help("An optional description of what you did before pushing"))
			.arg(Arg::with_name("branch")
				.short("b")
				.long("branch")
				.value_name("BRANCH")
				.help("The branch to push to")))
		
		// the pull command
		.subcommand(SubCommand::with_name("pull")
			.about("Automatically pull from the repository")
			.arg(Arg::with_name("branch")
				.short("b")
				.long("branch")
				.value_name("BRANCH")
				.help("The branch to pull from")))

		// the update command
		.subcommand(SubCommand::with_name("update"))
			.about("Clones from the master branch of the Elp repository and runs the make script")
			
		.get_matches();
	
	let verbosity = matches.occurrences_of("verbose") as usize;

	// runs the specified command
	if let Some(matches) = matches.subcommand_matches("start") {
		start(matches.value_of("url").unwrap(), matches.value_of("branch"), verbosity);
	}
	if let Some(matches) = matches.subcommand_matches("push") {
		push(matches.value_of("TITLE").unwrap(), matches.value_of("message"), matches.value_of("branch"), verbosity);
	}
	if let Some(_matches) = matches.subcommand_matches("pull") {pull(matches.value_of("branch"), verbosity);}
	if let Some(_matches) = matches.subcommand_matches("update") {update(verbosity);}
}
