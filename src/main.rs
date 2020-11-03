#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::path::Path;
use std::process::Command;
use std::{env, fs};

/// Settings構造体
#[derive(Debug, Serialize, Deserialize)]
struct Setting {
    /// 7-zipのインストールパス（7z.exeのパス）
    seven_zip_path: String,
    /// 解凍パスワード（複数可能）
    extract_passwords: Vec<String>,
}

fn main() {
    // コマンドライン引数
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("コマンドライン引数に、ファイル名を指定して下さい。")
    }

    // 解凍対象のファイル（フルパス）
    let mut target_file = Path::new(args[1].as_str());
    if !target_file.exists() {
        panic!("解凍対象のファイルが存在しません。")
    }

    // 設定ファイル
    let settings = read_settings();
    if !Path::new(&settings.seven_zip_path).exists() {
        panic!("7zipのdllが存在しません。")
    }

    let rename_file = change_extension(target_file);
    fs::rename(target_file, &rename_file).expect("解凍ファイルの拡張子変更に失敗しました。");
    target_file = Path::new(&rename_file);

    let result = extract(&settings, target_file);

    if result {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

/// 設定ファイルを読み込みます。
fn read_settings() -> Setting {
    const FILE_PATH: &str = "settings.toml";

    // 設定ファイル読み込み
    let read_line = fs::read_to_string(FILE_PATH).expect("設定ファイルの読み込みに失敗しました。");
    toml::from_str(&read_line).expect("設定ファイルのデシリアライズに失敗しました。")
}

/// 拡張子を変更した文字列を返します。
fn change_extension(file_path: &Path) -> String {
    let extension = file_path
        .extension()
        .expect("解凍ファイルの拡張子変更に失敗しました。");
    let file = file_path.to_str().unwrap();

    let path = match extension.to_str().unwrap() {
        "ex_" => format!("{}.{}", file.replace(".ex_", ""), "exe"),
        "zi_" => format!("{}.{}", file.replace(".zi_", ""), "zip"),
        _ => file_path.display().to_string(),
    };

    path
}

/// 7zipで解凍します。
///
/// 7zipの引数: http://fla-moo.blogspot.com/2013/05/7-zip.html
fn extract(setting: &Setting, target_file: &Path) -> bool {
    for pass in &setting.extract_passwords {
        let mut command = Command::new(&setting.seven_zip_path);
        command.arg("x"); // x: 解凍
        command.arg("-y"); // -y: 強制的に処理を続行
        command.arg(format!("-p{}", pass)); // -p: パスワードを設定
        command.arg(format!("-o{}", target_file.parent().unwrap().display())); // -o: 出力先を指定
        command.arg(target_file.display().to_string()); // 解凍対象のファイル

        let result = command
            .status()
            .expect("7z.exeの実行でエラーが発生しました。");
        if result.success() {
            return true;
        } else {
            continue;
        }
    }

    false
}
