@echo off
rem Build snipsnap.exe with MSVC. Works from any prompt; sets up VS if needed.
setlocal
cd /d "%~dp0"

if defined INCLUDE goto build
set "VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"
for /f "usebackq delims=" %%i in (`call "%VSWHERE%" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do set "VSDIR=%%i"
if not defined VSDIR (
  echo Visual Studio C++ build tools not found.
  exit /b 1
)
call "%VSDIR%\VC\Auxiliary\Build\vcvars64.bat" >nul || exit /b 1

:build
rem Stop a running instance, otherwise the exe is locked and linking fails.
if exist snipsnap.exe start "" /wait snipsnap.exe quit

cl /nologo /W4 /WX /O1 /GS- /DSS_NOCRT snipsnap.c ^
   /link /NODEFAULTLIB /ENTRY:WinMainCRTStartup /SUBSYSTEM:WINDOWS ^
   /OPT:REF /OPT:ICF /MANIFEST:NO /OUT:snipsnap.exe ^
   kernel32.lib user32.lib gdi32.lib dwmapi.lib || exit /b 1
del /q snipsnap.obj 2>nul
echo Built %CD%\snipsnap.exe
