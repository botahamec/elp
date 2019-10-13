@echo off
MKDIR "C:\Program Files\elp"
cargo build --release
XCOPY /E /I /Y target\release "C:\Program Files\elp\"
SETX /M PATH "C:\Program Files\elp;%PATH%"