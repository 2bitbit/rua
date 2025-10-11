1. Open pwsh and run this command: `notepad $PROFILE`  

2. Paste the following content into the notepad and replace the value of rua_executable with the path to your rua executable.
```powershell
# ----------------------rua part--------------------------------
function rua-helper-function-Invoke-PasteAndRestore {
    param([string]$TextToPaste,$OriginalClipboardContent)

    $scriptBlock = {
        param($NewContent, $OriginalContent)
        try{
            Set-Clipboard -Value $NewContent
            Add-Type -AssemblyName System.Windows.Forms
            [System.Windows.Forms.SendKeys]::SendWait("^v")
            Start-Sleep -Milliseconds 200
        }
        finally{
            if ($null -ne $OriginalContent) {Set-Clipboard -Value $OriginalContent} 
            else {Clear-Clipboard}
        }
    }
    Start-ThreadJob -ScriptBlock $scriptBlock -ArgumentList $TextToPaste, $OriginalClipboardContent | Out-Null
}
function rua {
    $rua_executable = "<path/to/your/rua.exe e.g. D:\Workspace\Repos\rua\target\debug\rua.exe>"

    if ($args.Count -eq 0) {
        $selected_command = & $rua_executable
        if (-not [string]::IsNullOrEmpty($selected_command)) {
            $originalClipboard = Get-Clipboard -Raw -ErrorAction SilentlyContinue
            rua-helper-function-Invoke-PasteAndRestore -TextToPaste $selected_command -OriginalClipboardContent $originalClipboard
        }
    }
    else {& $rua_executable @args} 
}
# ----------------------rua part--------------------------------
```

3. Save the notepad and restart your shell, then it'll work. Enjoy rua!