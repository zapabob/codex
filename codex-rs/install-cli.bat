@echo off
setlocal

:: 設定
set BINARY_NAME=codex.exe
set SOURCE_PATH=target\dev-fast\%BINARY_NAME%
set INSTALL_PATH=%USERPROFILE%\.cargo\bin\%BINARY_NAME%

echo [1/3] 既存プロセスを終了...
taskkill /F /IM %BINARY_NAME% 2>nul
timeout /t 1 /nobreak >nul

echo [2/3] バイナリをコピー...
if exist %INSTALL_PATH% del /F %INSTALL_PATH%
copy /Y %SOURCE_PATH% %INSTALL_PATH%

echo [3/3] 完了！
echo インストール先: %INSTALL_PATH%

endlocal
