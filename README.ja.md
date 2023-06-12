# generate-strong-password

generate-strong-passwordは、強力なパスワードを生成するためのコマンドラインツールです。このツールを使用すると、指定した長さや文字の組み合わせに基づいてセキュアなパスワードを作成することができます。

## インストール方法

```shell
sudo mkdir -p /usr/local/generate-strong-password/bin
curl -L https://github.com/kyuki3rain/generate-strong-password/releases/download/v0.1.0/generate-strong-password_v0.1.0_x86_64-unknown-linux-musl.tar.gz -o ./generate-strong-password.tar.gz
tar -zxvf generate-strong-password.tar.gz
rm generate-strong-password.tar.gz
sudo mv ./generate-strong-password /usr/local/generate-strong-password/bin
echo 'export PATH=$PATH:/usr/local/generate-strong-password/bin' >> ~/.bashrc
source ~/.bashrc
```

## 使用法

```shell
generate-strong-password [OPTIONS]
```

利用可能なオプションは以下の通りです。

- -l, --length <LENGTH>           : パスワードの長さを設定します。デフォルトは`16`です。
- -C, --uppercase <UPPERCASE>     : 大文字のアルファベットの重みを設定します。デフォルトは`4`です。
- -c, --lowercase <LOWERCASE>     : 小文字のアルファベットの重みを設定します。デフォルトは`4`です。
- -n, --numbers <NUMBERS>         : 数字の重みを設定します。デフォルトは`4`です。
- -s, --symbols <SYMBOLS>         : 記号の重みを設定します。デフォルトは`1`です。
-     --symbol-sets <SYMBOL_SETS> : パスワードに含める記号セットを設定します。デフォルトは!$%&'()*+,/;<=>?[]^{}~です。
- -h, --help                      : ヘルプを表示します。
- -V, --version                   : バージョンを表示します。


## 使用例

```shell
generate-strong-password
```

デフォルトの設定でパスワードが生成されます。

```
generate-strong-password -l 12 -C 1 -c 0 -n 1 -s 0
```

英大文字と数字が均等に現れる12文字のパスワードが生成されます。

重みが0以外の種類の文字は必ず1つ以上含まれます。


```
generate-strong-password --symbol-sets "@%&"
```

登場する記号が`@%&`の３種類の中から選ばれてパスワードが生成されます。

```
generate-strong-password -l 3
```

デフォルトでは4種類の文字が最低1文字ずつパスワードに入るため、3文字のパスワードは生成できません。このコマンドはエラーになります。

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。詳細については、[LICENSE](https://github.com/kyuki3rain/generate-strong-password/blob/master/LICENSE)ファイルを参照してください。


## 開発者
- [kyuki3rain](https://github.com/kyuki3rain)

ご不明点やバグの報告は、Issuesページにお願いします。