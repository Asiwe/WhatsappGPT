Set objShell = CreateObject("WScript.Shell")
Set objFSO = CreateObject("Scripting.FileSystemObject")

' Get the directory where this script is located
strScriptPath = objFSO.GetParentFolderName(WScript.ScriptFullName)
strExePath = strScriptPath & "\whatsappgpt.exe"

' Check if the exe exists
If objFSO.FileExists(strExePath) Then
    ' Run the exe with hidden window
    objShell.Run """" & strExePath & """", 0, False
Else
    ' Show error if exe not found
    MsgBox "whatsappgpt.exe not found in:" & vbCrLf & strScriptPath, vbCritical, "Error"
End If
