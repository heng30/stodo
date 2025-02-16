<div style="display: flex, margin: 20px, gap: 20px">
    <img src="./screenshot/1-cn.png" width="200"/>
    <img src="./screenshot/2-cn.png" width="200"/>
</div>

[English Documentation](./README.md)

### 简介
这是一款极其简单的`Todo`程序。基于`Rust`和`Slint-UI` GUI框架。该项目能够编译到`Windows`，`Linux`和`Macos`。

### 如何构建?
- 安装 `Rust` 和 `Cargo`
- 运行 `make desktop-debug` 调试桌面平台程序
- 运行 `make desktop-build-release` 编译桌面平台程序
- 参考 [Makefile](./Makefile) 了解更多信息

### 排错
因为`Slint`不支持在`wayland`上设置窗口位置。所以没法通过移动标题栏来移动程序。解决方法是通过编辑`~/.config/stodo/stodo.toml`文件，将`no_frame = true`修改为`no_frame = false`。这会使用系统的标题栏。

### 参考
- [Slint Language Documentation](https://slint-ui.com/releases/1.0.0/docs/slint/)
- [slint::android](https://snapshots.slint.dev/master/docs/rust/slint/android/#building-and-deploying)
- [Running In A Browser Using WebAssembly](https://releases.slint.dev/1.7.0/docs/slint/src/quickstart/running_in_a_browser)
- [github/slint-ui](https://github.com/slint-ui/slint)
- [Viewer for Slint](https://github.com/slint-ui/slint/tree/master/tools/viewer)
- [LSP (Language Server Protocol) Server for Slint](https://github.com/slint-ui/slint/tree/master/tools/lsp)
- [developer.android.com](https://developer.android.com/guide)
- [color4bg](https://www.color4bg.com/zh-hans/)
- [How to Deploy Rust Binaries with GitHub Actions](https://dzfrias.dev/blog/deploy-rust-cross-platform-github-actions/)
