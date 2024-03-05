# Runtimeの静的リンクについて

Rustでは、コンパイルオプション `target-feature=+crt-static` により、静的リンクを指定する事が出来ます。


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
