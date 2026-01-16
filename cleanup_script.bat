@echo off
echo ========================================
echo Cドライブ容量確保スクリプト
echo ========================================
echo.

echo 現在の容量状況を確認...
dir /-c C:\ | find "bytes free"
echo.

echo ---------------------------------------
echo 1. 一時ファイルの削除中...
echo ---------------------------------------

REM Windows一時ファイル削除
echo Windows一時ファイル削除...
del /f /s /q "C:\Windows\Temp\*" 2>nul
rd /s /q "C:\Windows\Temp" 2>nul
md "C:\Windows\Temp" 2>nul

REM ユーザー一時ファイル削除
echo ユーザー一時ファイル削除...
for /d %%i in ("C:\Users\*") do (
    if exist "%%i\AppData\Local\Temp" (
        echo 処理中: %%i
        del /f /s /q "%%i\AppData\Local\Temp\*" 2>nul
        rd /s /q "%%i\AppData\Local\Temp" 2>nul
        md "%%i\AppData\Local\Temp" 2>nul
    )
)

REM システム一時ファイル削除
echo システム一時ファイル削除...
del /f /s /q "%TEMP%\*" 2>nul
rd /s /q "%TEMP%" 2>nul
md "%TEMP%" 2>nul

echo.

echo ---------------------------------------
echo 2. ごみ箱の削除中...
echo ---------------------------------------

REM ごみ箱削除
echo ごみ箱削除...
rd /s /q "C:\$Recycle.Bin" 2>nul
md "C:\$Recycle.Bin" 2>nul

echo.

echo ---------------------------------------
echo 3. ダウンロードフォルダのクリーンアップ
echo ---------------------------------------

echo ダウンロードフォルダを確認中...
for /d %%i in ("C:\Users\*") do (
    if exist "%%i\Downloads" (
        echo 処理中: %%i\Downloads
        REM 古いファイルを削除（7日以上前のファイル）
        forfiles /p "%%i\Downloads" /s /m *.* /d -7 /c "cmd /c del @path" 2>nul
    )
)

echo.

echo ---------------------------------------
echo 4. ブラウザキャッシュの削除中...
echo ---------------------------------------

echo Chromeキャッシュ削除...
for /d %%i in ("C:\Users\*") do (
    if exist "%%i\AppData\Local\Google\Chrome\User Data\Default\Cache" (
        echo Chromeキャッシュ削除: %%i
        del /f /s /q "%%i\AppData\Local\Google\Chrome\User Data\Default\Cache\*" 2>nul
        rd /s /q "%%i\AppData\Local\Google\Chrome\User Data\Default\Cache" 2>nul
    )
)

echo Edgeキャッシュ削除...
for /d %%i in ("C:\Users\*") do (
    if exist "%%i\AppData\Local\Microsoft\Edge\User Data\Default\Cache" (
        echo Edgeキャッシュ削除: %%i
        del /f /s /q "%%i\AppData\Local\Microsoft\Edge\User Data\Default\Cache\*" 2>nul
        rd /s /q "%%i\AppData\Local\Microsoft\Edge\User Data\Default\Cache" 2>nul
    )
)

echo Firefoxキャッシュ削除...
for /d %%i in ("C:\Users\*") do (
    if exist "%%i\AppData\Local\Mozilla\Firefox\Profiles" (
        for /d %%p in ("%%i\AppData\Local\Mozilla\Firefox\Profiles\*") do (
            if exist "%%p\cache2" (
                echo Firefoxキャッシュ削除: %%p
                rd /s /q "%%p\cache2" 2>nul
            )
        )
    )
)

echo.

echo ---------------------------------------
echo 5. 追加クリーンアップ
echo ---------------------------------------

echo サムネイルキャッシュ削除...
del /f /s /q "C:\Users\*\AppData\Local\Microsoft\Windows\Explorer\thumbcache_*.db" 2>nul

echo ログファイル削除...
del /f /s /q "C:\Windows\Logs\*" 2>nul

echo 配信最適化ファイル削除...
del /f /s /q "C:\Windows\SoftwareDistribution\Download\*" 2>nul

echo.

echo ========================================
echo 容量確保後の状況
echo ========================================

echo 最終容量確認:
dir /-c C:\ | find "bytes free"

echo.
echo ========================================
echo 処理完了！
echo ========================================
echo.
echo 推奨される次のステップ:
echo 1. PCを再起動
echo 2. 容量が十分に確保されたか確認
echo 3. 必要に応じて追加のクリーンアップを実行
echo.
pause