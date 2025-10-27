# DIRLINK

使用TUI界面管理目录链接

## 安装方式

使用如下命令编译：

```shell
cargo build --release
```

使用如下命令安装：
```shell
cargo install --path .
```

将`shell`文件夹中的对应脚本添加到SHELL的配置文件中。
例如，如果你使用的是`bash`，可以将`bash.sh`中的内容加入到`.bashrc`中。

在SHELL中输入：

```shell
dlk
```

即可使用。

## TODO

- [ ] 提供便捷的安装方式
- [ ] 出现问题时的警告框
- [ ] 使用`?`打开帮助对话框
- [ ] 鼠标支持
