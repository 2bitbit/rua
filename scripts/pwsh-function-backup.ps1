# - My initial idea, to have a function insert a new command onto the next line after pressing Enter,
# couldn't be realized in my environment. 
# - However, we were able to solve it by thinking outside the box -- using clipboard.

# Helper function to handle the background clipboard and paste operations
function rua-helper-function-Invoke-PasteAndRestore {
    param([string]$TextToPaste,$OriginalClipboardContent) # OriginalClipboardContent can be null

    $scriptBlock = {
        param($NewContent, $OriginalContent)
        try{
            # This entire block runs in the background: 
            # 1. Place the new command onto the clipboard
            Set-Clipboard -Value $NewContent
            # 2. Simulate the paste command (Ctrl+V)
            Add-Type -AssemblyName System.Windows.Forms
            [System.Windows.Forms.SendKeys]::SendWait("^v")
            # 3. Restore the original clipboard content
            # A tiny delay helps ensure the paste command has been processed by the system.
            Start-Sleep -Milliseconds 200
        }
        finally{Set-Clipboard -Value $OriginalContent}
    }
    # Start a lightweight thread job to run the script block without delay.
    Start-ThreadJob -ScriptBlock $scriptBlock -ArgumentList $TextToPaste, $OriginalClipboardContent | Out-Null
}

# Main function
function rua {
    $rua_executable = "D:\Workspace\Repos\rua\target\debug\rua.exe" # "<path/to/your/rua.exe e.g. D:\Workspace\Repos\rua\target\debug\rua.exe>"

    if ($args.Count -eq 0) {
        $selected_command = & $rua_executable
        if (-not [string]::IsNullOrEmpty($selected_command)) {
            # Step A: Save the original clipboard content before doing anything else.
            # The -Raw switch gets text, and -ErrorAction SilentlyContinue handles an empty clipboard.
            $originalClipboard = Get-Clipboard -Raw -ErrorAction SilentlyContinue
            # Step B: Call the helper to perform the paste-and-restore operation in the background.
            rua-helper-function-Invoke-PasteAndRestore -TextToPaste $selected_command -OriginalClipboardContent $originalClipboard
        }
    }
    else {& $rua_executable @args}  # For subcommands like 'rua add', just run the command normally.
}