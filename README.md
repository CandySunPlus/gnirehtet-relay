# Gnirehtet Relay

这个项目 Fork 自开源项目[Genirehtet](https://github.com/Genymobile/gnirehtet) 的 Rust Relay 端。

项目结合 `adb` 的 `reverse tethering` ，将 Host 的端口映射到 Android 中，Android 结合 VPN 能力，将接管的所有手机流量转发到此端口上。转发服务端连接此端口，并开启基础 `socket`，对 [OSI 模型](https://en.wikipedia.org/wiki/OSI_model)的 3 层（设备端）和 5层（主机端）进行转发，从而实现设备上网。

设备的联网行为非常类似 NAT，不过只是通过 `TCP` 连接对一些基础的协议进行了转发，当前项目已支持基于 `IPv4` 的 `TCP`、 `UDP` 和 `ICMP` 协议包的转发功能，但不支持 `IPv6` 的数据转发。

# 启动依赖

转发端启动需要安装 `adb` ，版本 `>= 1.0.36` ，此版本后 `adb` 才实现了 `reverse` 能力。

`adb` 目前在 [Android SDK platform tools](https://developer.android.com/studio/releases/platform-tools.html) 中提供，MacOS 可以通过 [Homebrew](https://brew.sh/) 安装 `android-platform-tools` 。

最后，你需要确定连接的 Android 设备已经开启了 `调试模式` 。

# 启动

> 转发服务全局只用启动一个，每个新设备接入时开启对应的 `reverse tunnel` 即可。更多用法，可直接执行 `./gnirehtet` ，会显示所有的命令用法。

1. 启动转发服务，默认端口 `31416` :

    ```bash
    ./gnirehtet relay
    ```

2. 启动转发通道 `reverse tunnel` ，其中 `serial` 是  `adb devices` 中设备的连接标识 :

    ```bash
    ./gnirehtet tunnel [serial]
    ```

3. 启动转发服务，并自动 `track` 新连接的设备，启动 `reverse tunnel` :

    ```bash
    ./gnirehtet autorun
    ```

# 开发构建

1. MacOS 可使用 [Homebrew](https://brew.sh/) 直接安装 [Rustup](https://rustup.rs/)，然后通过 `rustup-init` 安装 [Rust](https://www.rust-lang.org/) :

    ```bash
    $ brew install rustup
    $ rustup-init
    ```

2. 安装完成后，在项目目录下执行 `cargo build` 完成构建，构建结果产出在 target 目录中：

    ```bash
    $ cargo build --release
    ```

3. 若要交叉编译其它系统下的版本，需要通过 `rustup` 安装对应平台的[交叉编译](https://rust-lang.github.io/rustup/cross-compilation.html)工具，最后指定编译目标平台进行编译，构建结果产出在 `target` 目录的对应平台文件夹内：

    ```bash
    $ cargo build --release --target <target> # 例如：x86_64-unknown-linux-musl
    ```

# 启动脚本

此脚本会循环退出所有连接设备的转发通道，并重启 `adb reverse tunnel` 。

```bash
#!/bin/sh

dns=$(cat /etc/resolv.conf | awk 'NR>1{print $2}' |paste -sd,)
series=$(adb devices -l | sed -n '1!P;N;$q;D' | awk '{print $1}')
for i in $series
do
        ./gnirehtet stop $i
        echo $i
done
./gnirehtet autorun -d $dns
```
