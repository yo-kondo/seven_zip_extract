# seven_zip_extract

7zipで解凍します。

## 使い方

`bin/7zipで解凍する.bat`に解凍したいファイルをドラッグ＆ドロップします。  
拡張子が`.ex_`、`.zi_`も自動で変換して解凍します。

## 前提条件

実行するPCに`7-Zip`がインストールされていること。

## 設定

``` toml
# 7-zipのインストールパス（7z.exeのパス）
seven_zip_path = "C:\\Program Files\\7-Zip\\7z.exe"

# 解凍パスワード（複数可能）
extract_passwords = [
    "password1",
    "password2",
]
```

設定ファイルは実行ファイルと同じディレクトリに配置して下さい。

## バージョン

* 7-Zip 19.00 (x64)
