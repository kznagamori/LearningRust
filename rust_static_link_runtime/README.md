# Runtimeの静的リンクについて

Rustでは、コンパイルオプション `target-feature=+crt-static` により、静的リンクを指定する事が出来ます。

## configファイルで設定


コンパイルオプションを `プロジェクトのフォルダ/.cargo/config.toml` に記述しておけば、ビルド時にコンパイルオプションが反映されます。

```
[build]
rustflags = ["-C", "target-feature=+crt-static"]
```

ターゲットごとにコンパイルオプションを設定したい場合は、以下のように記述できます。
```
# for MSVC 32bit
[target.i686-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]

# for MSVC 64bit
[target.x86_64-pc-windows-msvc]    
rustflags = ["-C", "target-feature=+crt-static"]
```

## ビルド時に設定

コマンドラインから環境変数 RUSTFLAGS を設定し、cargo build --release を実行する方法は以下の通りです。

**Windows (コマンドプロンプト)**
```
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release
```
**Windows (PowerShell)**
```
$env:RUSTFLAGS="-C target-feature=+crt-static"
cargo build --release
```
**Unix系OS (bash, zshなど)**
```
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release
```
