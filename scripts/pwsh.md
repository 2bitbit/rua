1. Open pwsh and run this command: `notepad $PROFILE`  

2. Paste the following content into the notepad and replace the value of rubus_executable with the path to your rubus executable.
```powershell
function rua-helper-function-Invoke-KeyPress {
    param([string]$Text)
    $scriptBlock = {
        param($KeysToSend)
        Add-Type -AssemblyName System.Windows.Forms 
        [System.Windows.Forms.SendKeys]::SendWait($KeysToSend)active
    }
    Start-Job -ScriptBlock $scriptBlock -ArgumentList $Text | Out-Null
}


function rua {
    $rubus_executable = "<path/to/your/rubus.exe e.g. D:\Workspace\Repos\rubus\target\debug\rubus.exe>"
    if ($args.Count -eq 0) {
            $selected_command = & $rubus_executable
            if (-not [string]::IsNullOrEmpty($selected_command)) {
                rua-helper-function-Invoke-KeyPress -Text $selected_command
            }
    }
    else {& $rubus_executable @args}
}
```

3. Save the notepad and restart your shell, then it'll work. Enjoy rua!