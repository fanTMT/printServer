' start_axum.vbs
Set WshShell = CreateObject("WScript.Shell")
WshShell.Run "D:\Doc\web_rust_print.exe -i 0.0.0.0 -p 3000", 0, False
