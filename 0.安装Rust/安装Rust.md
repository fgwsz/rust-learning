# 安装`Rust`
## `Windows`操作系统安装方式
下载`rustup-init.exe`进行安装.

`rustup-init.exe`下载地址:<https://rust-lang.org/zh-CN/tools/install/>

`rustup-init.exe`是`rustup`的安装程序.

运行`rustup-init.exe`会看到如下的安装提示:

注意:下图中的`15935`是用户名.
```
Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  C:\Users\15935\.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  C:\Users\15935\.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  C:\Users\15935\.cargo\bin

This path will then be added to your PATH environment variable by
modifying the PATH registry key at HKEY_CURRENT_USER\Environment.

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
```
输入`1`,回车选择默认安装方式,安装完成之后会提示如下信息:
```
info: profile set to default
info: default host triple is x86_64-pc-windows-msvc
info: syncing channel updates for stable-x86_64-pc-windows-msvc
info: latest update on 2026-04-16 for version 1.95.0 (59807616e 2026-04-14)
info: downloading 6 components
        cargo installed                        9.54 MiB
       clippy installed                        3.80 MiB
    rust-docs installed                       21.19 MiB
     rust-std installed                       21.05 MiB
        rustc installed                       68.26 MiB
      rustfmt installed                        2.47 MiB
info: default toolchain set to stable-x86_64-pc-windows-msvc

  stable-x86_64-pc-windows-msvc installed - rustc 1.95.0 (59807616e 2026-04-14)


Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload its PATH environment variable to include
Cargo's bin directory (%USERPROFILE%\.cargo\bin).

Press the Enter key to continue.
```
输入回车,完成安装.

注意:上述安装过程中定义了一个用户环境变量:`~/.cargo/bin`.

目录`~/.cargo/bin`的文件结构是这样的:
```
~/
│ .cargo/
│ │ bin/
│ │ │ cargo-clippy.exe
│ │ │ cargo-fmt.exe
│ │ │ cargo-miri.exe
│ │ │ cargo.exe
│ │ │ clippy-driver.exe
│ │ │ rls.exe
│ │ │ rust-analyzer.exe
│ │ │ rust-gdb.exe
│ │ │ rust-gdbgui.exe
│ │ │ rust-lldb.exe
│ │ │ rustc.exe
│ │ │ rustdoc.exe
│ │ │ rustfmt.exe
│ │ │ rustup.exe
```

## `Rust`工具链
从上面的安装信息完成提示中可以发现`rustup-init`安装了6个组件,
这些组件构成了`Rust`工具链.

这里简要介绍一下这6个组件:

`cargo`:包管理器

`clippy`:静态代码分析工具

`rust-docs`:说明文档

`rust-std`:标准库

`rustc`:编译器

`rustfmt`:代码格式化工具

可以检查一下各个组件的版本信息:
```
rustup --version
cargo --version
rustc --version
rustfmt --version
```
可能的显示:
```
rustup 1.29.0 (28d1352db 2026-03-05)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: the currently active `rustc` version is `rustc 1.95.0 (59807616e 2026-04-14)`

cargo 1.95.0 (f2d3ce0bd 2026-03-21)

rustc 1.95.0 (59807616e 2026-04-14)

rustfmt 1.9.0-stable (59807616e1 2026-04-14)
```

## `Rust`工具链命令行管理工具`rustup`
`rustup`的命令行用法:

`rustup --version`:显示`rustup`版本信息

`rustup update`:更新`rustup`

`rustup self uninstall`:卸载`Rust`及`Rust`工具链
