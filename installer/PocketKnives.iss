; PocketKnives installer (Inno Setup 6.7+): the runner and blades in one per-user install.
; installer\build.ps1 builds them into AppSource and passes it on the ISCC command line.

#define AppVersion "0.1.0"
#ifndef AppSource
  #error Pass /DAppSource=<staged app folder> (installer\build.ps1 does this).
#endif

#define AppName "PocketKnives"
#define AppUrl "https://github.com/benjkr/PocketKnives"

[Setup]
AppId={{FA06E5A1-B6DB-4BFF-97A2-37E8ECFFB33D}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher=benjkr
AppPublisherURL={#AppUrl}
AppSupportURL={#AppUrl}/issues
AppUpdatesURL={#AppUrl}/releases
VersionInfoVersion={#AppVersion}
VersionInfoDescription={#AppName} Setup

PrivilegesRequired=lowest
DefaultDirName={autopf}\{#AppName}
DisableDirPage=yes
DisableProgramGroupPage=yes
DisableReadyPage=yes
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
; QuickTooth's minimum.
MinVersion=10.0.19041

; The [Code] section closes the runner and blades through their commands.
CloseApplications=no
RestartApplications=no

SetupIconFile=..\assets\pocketknives.ico
UninstallDisplayIcon={uninstallexe}
UninstallDisplayName={#AppName}
WizardStyle=modern dynamic windows11
SetupLogging=yes

OutputDir=..\dist
OutputBaseFilename={#AppName}-Setup-{#AppVersion}
Compression=lzma2/ultra64
SolidCompression=yes
LZMAUseSeparateProcess=yes

[Tasks]
; Offered (ticked) only when the blade isn't installed yet. An update leaves each blade's
; "start when I sign in" as the user last set it.
Name: snipsnap; Description: "SnipSnap"; GroupDescription: "Start when I sign in to Windows:"; Check: IsFresh('snipsnap.exe')
Name: quicktooth; Description: "QuickTooth"; GroupDescription: "Start when I sign in to Windows:"; Check: IsFresh('QuickTooth.exe')

[Files]
Source: "{#AppSource}\snipsnap.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#AppSource}\QuickTooth.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#AppSource}\pk.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#AppSource}\icons\*.ico"; DestDir: "{app}\icons"; Flags: ignoreversion
Source: "{#AppSource}\winui\*"; DestDir: "{app}\winui"; Excludes: "*.pdb"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{autoprograms}\{#AppName}\PocketKnives Settings"; Filename: "{app}\pk.exe"; Parameters: "settings"
Name: "{autoprograms}\{#AppName}\SnipSnap"; Filename: "{app}\snipsnap.exe"; Comment: "Copy a rectangle, a window or the screen to the clipboard"
Name: "{autoprograms}\{#AppName}\QuickTooth"; Filename: "{app}\QuickTooth.exe"; Comment: "Connect or disconnect your AirPods with one click"
Name: "{userstartup}\{#AppName}"; Filename: "{app}\pk.exe"
; SnipSnap starts at sign-in through a Startup shortcut, as it always has.
Name: "{userstartup}\SnipSnap"; Filename: "{app}\snipsnap.exe"; Tasks: snipsnap

[Run]
Filename: "{app}\pk.exe"; Flags: nowait
; QuickTooth writes its own Run value (its tray menu turns it off again).
Filename: "{app}\QuickTooth.exe"; Parameters: "--autostart on"; Flags: runhidden waituntilterminated; Tasks: quicktooth
; No skipifsilent: a silent update should leave the blades running, as they were before.
Filename: "{app}\snipsnap.exe"; Description: "Start SnipSnap now"; Flags: nowait postinstall
Filename: "{app}\QuickTooth.exe"; Description: "Start QuickTooth now"; Flags: nowait postinstall

[UninstallDelete]
; Settings can create SnipSnap's Startup shortcut after Setup, so Setup never recorded it.
Type: files; Name: "{userstartup}\SnipSnap.lnk"
Type: dirifempty; Name: "{app}"

[Code]
const
  RunKey = 'Software\Microsoft\Windows\CurrentVersion\Run';

{ The blade's exe isn't in the install folder yet. }
function IsFresh(const Exe: String): Boolean;
begin
  Result := not FileExists(AddBackslash(WizardDirValue) + Exe);
end;

{ The running copies of this exe (WMI), or null if WMI is unavailable. }
function RunningCopies(const ExePath: String): Variant;
var
  Locator, Service: Variant;
  Path: String;
begin
  Result := Null;
  try
    Locator := CreateOleObject('WbemScripting.SWbemLocator');
    Service := Locator.ConnectServer('.', 'root\CIMV2');
    Path := ExePath;
    StringChangeEx(Path, '\', '\\', True);
    StringChangeEx(Path, '''', '\''', True);
    Result := Service.ExecQuery('SELECT ProcessId FROM Win32_Process WHERE ExecutablePath = ''' + Path + '''');
  except
    Result := Null;
  end;
end;

{ Without WMI, the blade's single-instance mutex says whether any copy runs. }
function IsRunning(const ExePath, Mutex: String): Boolean;
var
  Procs: Variant;
begin
  Procs := RunningCopies(ExePath);
  if VarIsNull(Procs) then
    Result := (Mutex <> '') and CheckForMutexes(Mutex)
  else
    Result := Procs.Count > 0;
end;

function WaitUntilGone(const ExePath, Mutex: String; TimeoutMs: Integer): Boolean;
var
  Waited: Integer;
begin
  Waited := 0;
  while IsRunning(ExePath, Mutex) and (Waited < TimeoutMs) do
  begin
    Sleep(200);
    Waited := Waited + 200;
  end;
  Result := not IsRunning(ExePath, Mutex);
end;

{ Asks the blade to stop with its own command, then force-closes it if it hasn't gone within
  10 s. A running exe can't be overwritten or deleted. }
function CloseBlade(const Exe, StopArgs, Mutex: String): Boolean;
var
  ExePath: String;
  Procs: Variant;
  I, Code: Integer;
begin
  Result := True;
  ExePath := ExpandConstant('{app}\') + Exe;
  if not FileExists(ExePath) or not IsRunning(ExePath, Mutex) then Exit;
  if StopArgs <> '' then
  begin
    Log('Asking the running ' + Exe + ' to stop');
    Exec(ExePath, StopArgs, '', SW_HIDE, ewWaitUntilTerminated, Code);
    if WaitUntilGone(ExePath, Mutex, 10000) then Exit;
  end;
  Log(Exe + ' did not stop in time; terminating it');
  Procs := RunningCopies(ExePath);
  if not VarIsNull(Procs) then
    for I := 0 to Procs.Count - 1 do
      try
        Procs.ItemIndex(I).Terminate();
      except
      end;
  Result := WaitUntilGone(ExePath, Mutex, 5000);
end;

function CloseSnipSnap: Boolean;
begin
  Result := CloseBlade('snipsnap.exe', 'quit', 'Local\SnipSnap.Daemon');
end;

function CloseRunner: Boolean;
begin
  Result := CloseBlade('pk.exe', '--exit', 'Local\PocketKnives.Runner');
  { A hung Windows 11 settings window can outlive a terminated runner. Close only the
    executable in this installation before Setup replaces its files. }
  if Result then
    Result := CloseBlade('winui\PocketKnives.Settings.exe', '', '');
end;

function CloseQuickTooth: Boolean;
begin
  Result := CloseBlade('QuickTooth.exe', '--exit', 'Local\QuickTooth.Tray.6F0C2B1E');
end;

function PrepareToInstall(var NeedsRestart: Boolean): String;
begin
  Result := '';
  if not CloseRunner then
    Result := 'PocketKnives Settings is still running and could not be closed. Close it, then run Setup again.'
  else if not CloseSnipSnap then
    Result := 'SnipSnap is still running and could not be closed. Run "snipsnap quit", then run Setup again.'
  else if not CloseQuickTooth then
    Result := 'QuickTooth is still running and could not be closed. Choose Exit in its tray menu, then run Setup again.';
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  Value: String;
begin
  if CurUninstallStep = usUninstall then
  begin
    CloseRunner;
    CloseSnipSnap;
    CloseQuickTooth;
    { Remove QuickTooth's "start when I sign in", but only if it points at this install. Its
      settings in %APPDATA%\QuickTooth are kept, so a reinstall remembers the chosen headset.
      SnipSnap's Startup shortcut is removed by [UninstallDelete]. }
    if RegQueryStringValue(HKEY_CURRENT_USER, RunKey, 'QuickTooth', Value) and
       (Pos(Lowercase(ExpandConstant('{app}')), Lowercase(Value)) > 0) then
      RegDeleteValue(HKEY_CURRENT_USER, RunKey, 'QuickTooth');
  end;
end;
