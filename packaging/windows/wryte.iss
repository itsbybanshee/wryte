; Wryte installer (Inno Setup 6). Build with packaging\windows\build-wryte-installer.ps1, which
; compiles the release exe first and passes its folder as /DSourceDir=…
;
; Installs for the current user by default (no admin prompt); the first page offers "everyone".
; The Tasks page lets the user pick shortcuts and which document types Wryte opens. Chosen types
; get a Wryte ProgId, Wryte in "Open with", and Wryte as the type's handler; Windows may still
; ask once before switching a type another app already owns. Wryte is also listed in Settings ›
; Default apps. Uninstalling removes all of it, and offers to delete the Wryter settings folder.

#ifndef SourceDir
  #define SourceDir "..\..\target\release"
#endif
#ifndef AppVersion
  #define AppVersion "1.0.0"
#endif
#define AppName "Wryte"
#define AppExe "Wryte.exe"
#define RepoDir "..\.."

[Setup]
AppId={{6F0F7C1E-4B7A-4C55-9D2E-2B7E5A9C3F11}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisher=Wryte
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
PrivilegesRequired=admin
PrivilegesRequiredOverridesAllowed=dialog
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
OutputDir={#RepoDir}\target\installer
OutputBaseFilename=Wryte-Setup-{#AppVersion}
SetupIconFile={#RepoDir}\assets\app-icon\wryte.ico
UninstallDisplayIcon={app}\{#AppExe}
UninstallDisplayName={#AppName}
LicenseFile={#RepoDir}\LICENSE-MIT
WizardStyle=modern
Compression=lzma2/ultra64
SolidCompression=yes
ChangesAssociations=yes
CloseApplications=yes
RestartApplications=no

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Create a &desktop shortcut"; GroupDescription: "Shortcuts:"
Name: "assoc"; Description: "Open these document types with Wryte:"; GroupDescription: "File types:"
Name: "assoc\docx"; Description: "Word documents (.docx)"
Name: "assoc\doc"; Description: "Older Word documents (.doc)"
Name: "assoc\docm"; Description: "Macro-enabled Word documents (.docm)"
Name: "assoc\dotx"; Description: "Word templates (.dotx, .dot, .dotm)"
Name: "assoc\rtf"; Description: "Rich Text documents (.rtf)"
Name: "assoc\odt"; Description: "OpenDocument text (.odt)"
Name: "assoc\md"; Description: "Markdown (.md)"; Flags: unchecked
Name: "assoc\txt"; Description: "Plain text (.txt)"; Flags: unchecked

[Files]
Source: "{#SourceDir}\wryte.exe"; DestDir: "{app}"; DestName: "{#AppExe}"; Flags: ignoreversion
Source: "{#RepoDir}\LICENSE-MIT"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#RepoDir}\LICENSE-APACHE"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#RepoDir}\NOTICE"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#RepoDir}\ATTRIBUTION.md"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{autoprograms}\{#AppName}"; Filename: "{app}\{#AppExe}"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExe}"; Tasks: desktopicon

[Registry]
; The app itself: App Paths (so "Wryte" runs from Win+R), Open with, and Default apps.
Root: HKA; Subkey: "Software\Microsoft\Windows\CurrentVersion\App Paths\{#AppExe}"; ValueType: string; ValueName: ""; ValueData: "{app}\{#AppExe}"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExe}"; ValueType: string; ValueName: "FriendlyAppName"; ValueData: "{#AppName}"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\Applications\{#AppExe}\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\{#AppExe}"" ""%1"""
Root: HKA; Subkey: "Software\Wryte"; ValueType: string; ValueName: "InstallDir"; ValueData: "{app}"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Wryte\Capabilities"; ValueType: string; ValueName: "ApplicationName"; ValueData: "{#AppName}"
Root: HKA; Subkey: "Software\Wryte\Capabilities"; ValueType: string; ValueName: "ApplicationDescription"; ValueData: "Write and design documents; open and save Word files."
Root: HKA; Subkey: "Software\Wryte\Capabilities"; ValueType: string; ValueName: "ApplicationIcon"; ValueData: """{app}\{#AppExe}"",0"
Root: HKA; Subkey: "Software\RegisteredApplications"; ValueType: string; ValueName: "{#AppName}"; ValueData: "Software\Wryte\Capabilities"; Flags: uninsdeletevalue

; One ProgId per document type: its name, icon and open command.
#define ProgId(Id, Desc) \
  "Root: HKA; Subkey: ""Software\Classes\" + Id + """; ValueType: string; ValueName: """"; ValueData: """ + Desc + """; Flags: uninsdeletekey" + NewLine + \
  "Root: HKA; Subkey: ""Software\Classes\" + Id + "\DefaultIcon""; ValueType: string; ValueName: """"; ValueData: """"""{app}\" + AppExe + """"",0""" + NewLine + \
  "Root: HKA; Subkey: ""Software\Classes\" + Id + "\shell\open\command""; ValueType: string; ValueName: """"; ValueData: """"""{app}\" + AppExe + """"" """"%1""""""" + NewLine
#emit ProgId("Wryte.Document", "Word Document")
#emit ProgId("Wryte.LegacyDocument", "Word 97-2003 Document")
#emit ProgId("Wryte.DocumentMacroEnabled", "Word Macro-Enabled Document")
#emit ProgId("Wryte.Template", "Word Template")
#emit ProgId("Wryte.RichText", "Rich Text Document")
#emit ProgId("Wryte.OpenDocumentText", "OpenDocument Text")
#emit ProgId("Wryte.Markdown", "Markdown Document")
#emit ProgId("Wryte.PlainText", "Text Document")

; Every type lists Wryte under "Open with"; a chosen type also makes Wryte its handler.
#define Ext(Ext, Id, Task) \
  "Root: HKA; Subkey: ""Software\Classes\" + Ext + "\OpenWithProgids""; ValueType: string; ValueName: """ + Id + """; ValueData: """"; Flags: uninsdeletevalue" + NewLine + \
  "Root: HKA; Subkey: ""Software\Classes\Applications\" + AppExe + "\SupportedTypes""; ValueType: string; ValueName: """ + Ext + """; ValueData: """"" + NewLine + \
  "Root: HKA; Subkey: ""Software\Wryte\Capabilities\FileAssociations""; ValueType: string; ValueName: """ + Ext + """; ValueData: """ + Id + """" + NewLine + \
  "Root: HKA; Subkey: ""Software\Classes\" + Ext + """; ValueType: string; ValueName: """"; ValueData: """ + Id + """; Flags: uninsclearvalue; Tasks: " + Task + NewLine
#emit Ext(".docx", "Wryte.Document", "assoc\docx")
#emit Ext(".doc", "Wryte.LegacyDocument", "assoc\doc")
#emit Ext(".docm", "Wryte.DocumentMacroEnabled", "assoc\docm")
#emit Ext(".dotx", "Wryte.Template", "assoc\dotx")
#emit Ext(".dot", "Wryte.Template", "assoc\dotx")
#emit Ext(".dotm", "Wryte.Template", "assoc\dotx")
#emit Ext(".rtf", "Wryte.RichText", "assoc\rtf")
#emit Ext(".odt", "Wryte.OpenDocumentText", "assoc\odt")
#emit Ext(".md", "Wryte.Markdown", "assoc\md")
#emit Ext(".txt", "Wryte.PlainText", "assoc\txt")

[Run]
Filename: "{app}\{#AppExe}"; Description: "Launch Wryte"; Flags: nowait postinstall skipifsilent
Filename: "ms-settings:defaultapps?registeredAppUser=Wryte"; Description: "Open Default apps to finish choosing Wryte for documents"; Flags: shellexec nowait postinstall skipifsilent unchecked

[UninstallDelete]
Type: dirifempty; Name: "{app}"

[Code]
// Uninstall: offer to remove the settings, logs and recovery copies in %APPDATA%\Wryter.
procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  Dir: string;
begin
  if CurUninstallStep = usPostUninstall then
  begin
    Dir := ExpandConstant('{userappdata}\Wryter');
    if DirExists(Dir) and not UninstallSilent then
      if MsgBox('Also delete your Wryte settings, logs and recovery copies?' #13#10 + Dir,
                mbConfirmation, MB_YESNO or MB_DEFBUTTON2) = IDYES then
        DelTree(Dir, True, True, True);
  end;
end;
