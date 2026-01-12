# generate-strong-password

[![Crates.io](https://img.shields.io/crates/v/generate-strong-password.svg)](https://crates.io/crates/generate-strong-password)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English README](README.md)

強力なパスワードを生成するためのコマンドラインツールです。指定した長さや文字の組み合わせに基づいてセキュアなパスワードを作成できます。

## インストール方法

### cargo install を使用

```shell
cargo install generate-strong-password
```

### cargo binstall を使用（推奨）

[cargo-binstall](https://github.com/cargo-bins/cargo-binstall) がインストールされている場合、ビルド済みバイナリをインストールできます：

```shell
cargo binstall generate-strong-password
```

### GitHub Releases から直接ダウンロード

[GitHub Releases](https://github.com/kyuki3rain/generate-strong-password/releases) からお使いのプラットフォーム用のバイナリをダウンロードできます。

#### Linux

```shell
curl -L https://github.com/kyuki3rain/generate-strong-password/releases/latest/download/generate-strong-password_x86_64-unknown-linux-musl.tar.gz | tar xz
sudo mv generate-strong-password /usr/local/bin/
```

#### macOS

```shell
curl -L https://github.com/kyuki3rain/generate-strong-password/releases/latest/download/generate-strong-password_x86_64-apple-darwin.zip -o gsp.zip
unzip gsp.zip
sudo mv generate-strong-password /usr/local/bin/
rm gsp.zip
```

#### Windows

[Releases](https://github.com/kyuki3rain/generate-strong-password/releases) から `generate-strong-password_x86_64-pc-windows-gnu.zip` をダウンロードして解凍してください。

## 使用法

```shell
generate-strong-password [OPTIONS]
```

### オプション

| オプション | 説明 | デフォルト |
|-----------|------|-----------|
| `-l, --length <LENGTH>` | パスワードの長さ | `16` |
| `-C, --uppercase <UPPERCASE>` | 大文字の重み | `4` |
| `-c, --lowercase <LOWERCASE>` | 小文字の重み | `4` |
| `-n, --numbers <NUMBERS>` | 数字の重み | `4` |
| `-s, --symbols <SYMBOLS>` | 記号の重み | `1` |
| `--symbol-sets <SYMBOL-SETS>` | 使用する記号文字 | ``!$%&'()*+,/;<=>?[]^{}~`` |
| `-h, --help` | ヘルプを表示 | |
| `-V, --version` | バージョンを表示 | |

## 使用例

デフォルト設定でパスワードを生成：

```shell
generate-strong-password
```

大文字と数字のみで12文字のパスワードを生成：

```shell
generate-strong-password -l 12 -C 1 -c 0 -n 1 -s 0
```

特定の記号（`@%&`）のみを使用してパスワードを生成：

```shell
generate-strong-password --symbol-sets "@%&"
```

**注意:** 重みが0以外の文字種は必ず1文字以上含まれます。例えばデフォルト設定（4種類）では、最小長は4文字です。

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。詳細は [LICENSE](LICENSE) ファイルを参照してください。

## 開発者

- [kyuki3rain](https://github.com/kyuki3rain)

ご質問やバグの報告は [Issues](https://github.com/kyuki3rain/generate-strong-password/issues) ページにお願いします。
