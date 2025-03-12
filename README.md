# QuickMail

## English Version

A simple command-line email sending tool written in Rust.

### Features

- Receive email title and content from the command line
- Read sender, recipient, and SMTP server information from a configuration file
- Securely retrieve SMTP password from the system keychain

### Installation

```bash
cargo install quickmail
```

### Compile from Source

```bash
cargo build --release
```

The compiled binary will be located at `target/release/quickmail`.

### Configuration

1. Create a configuration file. You can either:
   - Create `~/.quickmail.yml` in your home directory (recommended)
   - Create `config.yml` in the current directory
   - Use a custom named configuration file (which you'll need to specify with `--config`)

```bash
# Option 1: Create in home directory (recommended)
cp config.yml.example ~/.quickmail.yml

# Option 2: Create in current directory
cp config.yml.example config.yml
```

2. Edit the `config.yml` file, fill in your email and SMTP server information:

```yaml
# Email configuration
sender: "your.email@example.com"
receiver: "recipient@example.com"
smtp_server: "smtp.example.com"
smtp_port: 587

# Keychain access parameters
keychain_service: "quickmail"
keychain_account: "your.email@example.com"
```

3. Store your SMTP password in the system keychain:

On macOS, you can use the following command:

```bash
security add-generic-password -s "quickmail" -a "your.email@example.com" -w "your-password"
```

Make sure that `keychain_service` and `keychain_account` match the values you used when storing the password.

On platforms other than Mac, Rust's Keyring library should also provide other password services, but the author hasn't tested them. Please check the [Keyring library documentation](https://docs.rs/keyring/latest/keyring/) for more information.

### Usage

```bash
# Use the default configuration search path (first config.yml in current directory, then ~/.quickmail.yml)
quickmail --title "Email Title" --message "Email Content"

# Specify a configuration file
quickmail --title "Email Title" --message "Email Content" --config "my-config.yml"

# Use short parameters
quickmail -t "Email Title" -m "Email Content" -c "my-config.yml"
```

### Error Handling

If you encounter problems, the program will provide detailed error information to help you diagnose the issue. Common problems include:

- Configuration file does not exist or has incorrect format
- Password not found in Keychain
- SMTP server connection failure
- Email sending failure

---

## 中文版

一个简单的命令行邮件发送工具，使用 Rust 编写。

### 功能

- 从命令行接收邮件标题和内容
- 从配置文件读取发件人、收件人和 SMTP 服务器信息
- 从系统 keychain 安全读取 SMTP 密码

### 安装

```bash
cargo install quickmail
```

### 从源码编译

```bash
cargo build --release
```

编译后的二进制文件将位于 `target/release/quickmail`。

### 配置

1. 创建配置文件。您可以选择：
   - 在您的主目录中创建 `~/.quickmail.yml`（推荐）
   - 在当前目录中创建 `config.yml`
   - 使用自定义命名的配置文件（需要使用 `--config` 指定）

```bash
# 选项1：在主目录中创建（推荐）
cp config.yml.example ~/.quickmail.yml

# 选项2：在当前目录中创建
cp config.yml.example config.yml
```

2. 编辑 `config.yml` 文件，填入您的邮箱和 SMTP 服务器信息：

```yaml
# Email configuration
sender: "your.email@example.com"
receiver: "recipient@example.com"
smtp_server: "smtp.example.com"
smtp_port: 587

# Keychain access parameters
keychain_service: "quickmail"
keychain_account: "your.email@example.com"
```

3. 将您的 SMTP 密码存储到系统 keychain 中：

在 macOS 上，可以使用以下命令：

```bash
security add-generic-password -s "quickmail" -a "your.email@example.com" -w "your-password"
```

请确保 `keychain_service` 和 `keychain_account` 与您存储密码时使用的值一致。

在 Mac 以外的平台上，Rust 的 Keyring 库应该也提供其他密码服务，但作者没有测试过。请查看 [Keyring 库的文档](https://docs.rs/keyring/latest/keyring/) 以获取更多信息。

### 使用方法

```bash
# 使用默认配置文件搜索路径（首先是当前目录下的 config.yml，然后是 ~/.quickmail.yml）
quickmail --title "邮件标题" --message "邮件内容"

# 指定配置文件
quickmail --title "邮件标题" --message "邮件内容" --config "my-config.yml"

# 使用短参数
quickmail -t "邮件标题" -m "邮件内容" -c "my-config.yml"
```

### 错误处理

如果遇到问题，程序会提供详细的错误信息，帮助您诊断问题所在。常见问题包括：

- 配置文件不存在或格式错误
- Keychain 中找不到密码
- SMTP 服务器连接失败
- 邮件发送失败
