# You can open your profile for editing by running the command $PROFILE in PowerShell.


# - My initial idea, to have a function insert a new command onto the next line after pressing Enter,
# couldn't be realized in my environment. 
# - However, we were able to solve it by thinking outside the box and
# using a function that simulates automatically typing the characters.

# Helper function that types text into the active window
function rua-helper-function-Invoke-KeyPress {
    param([string]$Text)

    # This is the code that will run in a background process
    $scriptBlock = {
        param($KeysToSend)
        Add-Type -AssemblyName System.Windows.Forms  # Load the required .NET assembly for sending keystrokes
        [System.Windows.Forms.SendKeys]::SendWait($KeysToSend) # Send the keystrokes to whatever window is currently active
    }

    # Start a background job to run the script block.
    # This lets our main function finish instantly.
    Start-Job -ScriptBlock $scriptBlock -ArgumentList $Text | Out-Null
}


function rua {
    $rubus_executable = "D:\Workspace\Repos\rubus\target\debug\rubus.exe"
        # 检查 $args 数组的长度。如果长度为 0，说明用户只输入了 `rua`，没有带任何参数。
    if ($args.Count -eq 0) {
            # --- 模式一：交互式填充 ---
            # 1. 执行 rubus 并捕获其标准输出 (stdout)
            $selected_command = & $rubus_executable
            # 2. 检查是否有命令被输出（即用户是否在 TUI 中选择了命令）
            if (-not [string]::IsNullOrEmpty($selected_command)) {
                rua-helper-function-Invoke-KeyPress -Text $selected_command
            }
    }
    else {
        # --- 模式二：普通命令执行 ---
        # 如果 $args 不为空，说明用户执行的是 rua add, rua ls 等子命令。
        # 在这种情况下，我们直接执行命令，并让它的输出自然地显示在控制台中，而不进行任何捕获或填充操作。
        # 使用 & (调用操作符) 和 @args (参数) 
         & $rubus_executable @args
    }
}