# Quickmail

一个简单的命令行邮件发送工具，使用 Rust 编写。

## 功能

- 从命令行接收邮件标题和内容
- 从配置文件读取发件人、收件人和 SMTP 服务器信息
- 从系统 keychain 安全读取 SMTP 密码

## 安装

```bash
cargo build --release
```

编译后的二进制文件将位于 `target/release/quickmail`。

## 配置

1. 复制示例配置文件并进行修改：

```bash
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

## 使用方法

```bash
# 使用默认配置文件 (config.yml)
quickmail --title "邮件标题" --message "邮件内容"

# 指定配置文件
quickmail --title "邮件标题" --message "邮件内容" --config "my-config.yml"

# 使用短参数
quickmail -t "邮件标题" -m "邮件内容" -c "my-config.yml"
```

## 错误处理

如果遇到问题，程序会提供详细的错误信息，帮助您诊断问题所在。常见问题包括：

- 配置文件不存在或格式错误
- Keychain 中找不到密码
- SMTP 服务器连接失败
- 邮件发送失败
