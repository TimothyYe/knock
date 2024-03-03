# knock: 一个用Rust实现的端口敲门程序

<img src="https://raw.githubusercontent.com/TimothyYe/knock/master/images/knock.png" width="600">

## 什么是端口敲门？

端口敲门是一种通过在预设的一组关闭端口上生成连接尝试来从外部打开防火墙端口的方法。一旦收到正确的连接尝试序列，防火墙规则就会动态修改，允许发送连接尝试的主机通过特定端口连接。

`knock` __只检测SYN包，不占用监听打开的端口__，它使用[pnet](https://docs.rs/pnet/latest/pnet/) crate来捕获原始网络数据包。

这种技术的一个常见用途是通过只有在成功执行端口敲门序列后才允许访问SSH端口，从而保护对SSH服务器的连接。

此项目的灵感来自于另一个用C语言编写的[knock](https://github.com/jvinet/knock)项目，但它是由Rust编写的，并且具有不同的配置格式。

## 为什么使用端口敲门？

端口敲门是一种简单而有效的方式来保护你的服务器免受未授权访问。它是一种轻量级且安全的方法来保护你的服务器免受未授权访问。

## 常见用例

- 保护你的SSH服务器免受暴力破解攻击
- 根据你的需求动态地打开和关闭防火墙上的任何端口

## Download

You can download the pre-built binaries from the [releases](https://github.com/TimothyYe/knock/releases) page.

## 构建

```bash
cargo build --release
```

## 配置

### 服务端配置

在与 `knockd` 二进制文件相同的目录中创建一个名为 `config.yaml` 的配置文件。

```yaml
interface: "eth0"
timeout: 5
rules:
  - name: "enable_ssh"
    command: "/usr/sbin/iptables -I INPUT -s %IP% -p tcp --dport 22 -j ACCEPT"
    sequence:
      - 15523
      - 17767
      - 32768
      - 28977
      - 51234
  - name: "disable_ssh"
    command: "/usr/sbin/iptables -D INPUT -s %IP% -p tcp --dport 22 -j ACCEPT"
    sequence:
      - 51234
      - 28977
      - 32768
      - 17767
      - 15523
```

- `interface`：要监听的网络接口
- `timeout`：等待客户端发送完整序列的超时时间（以秒为单位）
- `rules`：收到正确序列时要应用的规则
	- `name`：规则的名称
	- `command`：收到正确序列时要执行的命令。`%IP%` 将被替换为客户端的IP地址
	- `sequence`：客户端敲门的端口序列

### 客户端配置

在与 `knock-cli` 二进制文件相同的目录中创建一个名为 `config.yaml` 的配置文件。

__Do make sure that the client has the same sequence as the server.__

```yaml
rules:
  - name: "enable_ssh"
    host: "example.com"
    sequence:
      - 12345
      - 54321
      - 32768
      - 18933
  - name: "disable_ssh"
    host: "example.com"
    sequence:
      - 18933
      - 32768
      - 54321
      - 12345
```

- `rules`: 当发送正确的序列时应用的规则
	- `name`：规则的名称，名称不需要与服务器的规则名称匹配，但序列需要匹配。此外，名称在客户端的配置文件中应该是唯一的
	- `host`：服务端的地址
	- `sequence`：客户端敲门的端口序列

## 使用方法

### 服务端

```bash
./knockd -c config.yaml
```

默认的配置文件路径是 `config.yaml`，你也可以通过使用 `-c` 选项来指定配置文件的路径。

### 客户端

```bash
./knock-cli -c config.yaml -r enable_ssh
```

默认的配置文件路径是 `config.yaml`，你也可以通过使用 `-c` 选项来指定配置文件的路径。

`-r` 选项用于指定敲门时运行的规则名称。

## 作为docker容器运行服务端

```bash
docker run --network host --cap-add=NET_RAW --cap-add=NET_BIND_SERVICE --cap-add=NET_ADMIN -d --restart=always --name=knockd -v ./config.yaml:/config.yaml:ro ghcr.io/timothyye/knockd:latest
```

由于服务器需要监听原始数据包，因此需要将 `NET_RAW`、`NET_BIND_SERVICE` 和 `NET_ADMIN` 权限添加到容器中。

## 示例

假设你已经添加了一个防火墙规则来阻止所有对SSH端口的连接。例如：

```bash
iptables -A INPUT -p tcp --dport 22 -j DROP
```

使用以下命令在服务器上启用SSH端口：

```bash
./knock-cli -r enable_ssh
```

发送正确的序列后，SSH端口将会对客户端的IP地址打开。现在你可以连接到SSH服务器了。

要关闭SSH端口，使用以下命令：

```bash
./knock-cli -r disable_ssh
```
