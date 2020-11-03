@echo off

REM 7zipで解凍する
REM このbatファイルに圧縮ファイルをドラッグ＆ドロップすればOK

REM カレントディレクトリをbatファイルのディレクトリに変更
cd /d %~dp0

temp_rust.exe %1
pause
