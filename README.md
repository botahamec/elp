# elp
A helper for git to make some mundane tasks simpler

## Building
I plan to make this available with commonly used package managers at some point, and provide an installer. For now you need to clone, the repository, cargo build and put in in your.local/bin folder. You get the gist this isn't your first day here.

I will mention that you need git to already be installed in order to use this. If you're building, you obviously need Cargo and you also need the clap crate, which helps with creating CLI tools.

## Usage
Elp has a grand total of three commands, all of which require that you have a blank Github repository to start.

```bash
elp start -u [remote link]
```
Creates a local repository and pushes it to the remote repository linked. For example: https://www.github.com/Botahamec/elp.git

```bash
elp push [TITLE] -m [MESSAGE]
```
Creates a commit and pushes it using the specififed title and an optional message.

```bash
elp pull
```
Pulls from the origin. Simple enough.
