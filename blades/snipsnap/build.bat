@echo off
rem Build snipsnap.exe with cargo (the Rust workspace at the repo root). Works from any prompt:
rem cargo finds the MSVC linker itself.
rem   build.bat         builds snipsnap.exe here, stopping a running SnipSnap first
rem   build.bat <dir>   builds <dir>\snipsnap.exe (an absolute path); stops nothing
setlocal
set "OUT=%~1"
set "TARGET=%~dp0..\..\target"
cd /d "%~dp0"

if defined OUT goto build
set "OUT=%CD%"
rem Stop a running instance, otherwise the exe is locked and the copy fails. Any running SnipSnap
rem is found by its window class, so this stops the user's installed copy too.
if exist snipsnap.exe start "" /wait snipsnap.exe quit

:build
cargo build --release -p snipsnap --target-dir "%TARGET%" || exit /b 1
copy /y "%TARGET%\release\snipsnap.exe" "%OUT%\snipsnap.exe" >nul || exit /b 1
echo Built %OUT%\snipsnap.exe
