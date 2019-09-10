@echo off
MKDIR "C:\Program Files\elp"
XCOPY /S . "C:\Program Files\elp\"
SET "PATH:=C:\Program Files\elp;%PATH%"