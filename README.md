# generate-strong-password

generate-strong-password is a command line tool for generating strong passwords. This tool allows you to create a secure password based on a specified length or combination of characters.

## Installations

```shell
sudo mkdir -p /usr/local/generate-strong-password/bin
curl -L https://github.com/kyuki3rain/generate-strong-password/releases/download/v0.1.0/generate-strong-password_v0.1.0_x86_64-unknown-linux-musl.tar.gz -o ./generate-strong-password.tar.gz
tar -zxvf generate-strong-password.tar.gz
rm generate-strong-password.tar.gz
sudo mv ./generate-strong-password /usr/local/generate-strong-password/bin
echo 'export PATH=$PATH:/usr/local/generate-strong-password/bin' >> ~/.bashrc
source ~/.bashrc
```

## Usage

```shell
generate-strong-password [OPTIONS]
```

Available options are:

- -l, --length <LENGTH>           : Sets the length of the password. Default is `16`.
- -C, --uppercase <UPPERCASE>     : Sets the weight of uppercase alphabets. Default is `4`.
- -c, --lowercase <LOWERCASE>     : set weight of lowercase alphabet. Default is `4`.
- -n, --numbers <NUMBERS>         : set weights of numbers. Default is `4`.
- -s, --symbols <SYMBOLS>         : set weights of symbols. Default is `1`.
-   , --symbol-sets <SYMBOL-SETS> : Sets symbol sets to be included in the password. Default is `!$%&'()*+,/;<=>? []^{}~`
- -h, --help                      : Display help.
- -V, --version                   : Display version.


## Examples

```shell
generate-strong-password
```

Generates a password with default settings.

```shell
generate-strong-password -l 12 -C 1 -c 0 -n 1 -s 0
```

Generates a 12-character password with equal parts uppercase letters and numbers.

It must contain at least one character of any kind with a non-zero weight.


```shell
generate-strong-password --symbol-sets "@%&"
```

The password will be generated with one of the three types of symbols that appear as `@%&`.

```shell
generate-strong-password -l 3
```

A 3-character password cannot be generated because by default, at least one of each of the four types of characters will be in the password. This command will result in an error.

## License

This project is released under the MIT License. See the [LICENSE](https://github.com/kyuki3rain/generate-strong-password/blob/master/LICENSE) file for more information.


## Developer
- [kyuki3rain](https://github.com/kyuki3rain)

Please report any questions or bugs to the Issues page.
