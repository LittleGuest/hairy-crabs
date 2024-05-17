# 代码生成 CLI 工具

根据指定的数据库和表名等信息生成`Rust`对应的`struct`。

# 安装

```shell
cargo install --path .
```

# 参数说明

暂时只支持`MySQL`驱动

```shell
$ generator --help
generator 0.1.0

2190975784@qq.com

生成器配置

USAGE:
    generator [OPTIONS] --database <DATABASE>

OPTIONS:
    -d, --driver <DRIVER>              数据库驱动 [default: mysql]                                                                                       
    -D, --database <DATABASE>          指定的数据库名称
    -h, --help                         Print help information
    -p, --password <PASSWORD>          数据库密码 [default: root]
    -P, --port <PORT>                  数据库端口号 [default: 3306]
        --path <PATH>                  代码生成的路径 [default: tps/]
    -t, --table-names <TABLE_NAMES>    指定要生成代码的表名，多个用英文逗号拼接，为空表示全部
                                       [default: ]
    -u, --username <USERNAME>          数据库账号 [default: root]
        --url <URL>                    数据库地址 [default: localhost]
    -V, --version                      Print version information
```

# 生成的代码

```rust
use rbatis::{crud_table, Timestamp};
use serde::{Deserialize, Serialize};

/// 参数配置表
#[crud_table]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysConfig { 
    /// 参数名称
    pub config_name: Option<String>,
    /// 参数键名
    pub config_key: Option<String>,
    /// 参数键值
    pub config_value: Option<String>,
}
```

# 卸载

```shell
cargo uninstall generator
```