# elp
A helper for git to make some mundane tasks simpler. When you create an empty Github repository normally, you need to do five commands to get your local folder on the remote repository. This application simplifies things dramatically by giving commands which do these things for you automatically.

## Building
In order to build this, you'll need to have Cargo (the Rust compiler) and Git pre-installed.

 1. Clone the repository `git clone https://github.com/Botahamec/elp.git`
 2. Run the installer script. On Unix systems, it's make_linux.sh and on Windows it's make_win.cmd
 3. Verify that it installed correctly by typing "elp -V" in a terminal

## Usage
Elp has a grand total of three commands, all of which require that you have a blank Github repository to start.

```bash
elp start -u [remote link]
```
Creates a local repository and pushes it to the remote repository linked. For example: `https://www.github.com/Botahamec/elp.git`

```bash
elp push [TITLE] -m [MESSAGE]
```
Creates a commit and pushes it using the specififed title and an optional message.

```bash
elp pull
```
Pulls from the origin. Simple enough.
