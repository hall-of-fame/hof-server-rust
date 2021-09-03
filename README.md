## Hall Of Fame Server

Hall Of Fame 项目的后端服务器，采用 Rust + Rocket 🚀 开发。

接口文档[见此链接](https://github.com/Hall-of-Fame/hall-of-fame-server/blob/master/docs/api.md)。

## Ignored Files

### `/static` (DIR)

目录结构如下：

```
.
├── 0xfa
├── Android
├── Backend
├── Design
├── Frontend
├── iOS
├── PM
└── SRE
    └── 20
        └── 李小明
            ├── some_category
            │   └── a.jpg
            └── b.jpg
```

### `/src/utils/table.rs` (FILE)

文件内容结构如下：

```rust
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref AVATARS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        /* 以下注释代码可循环多次，首参数为人名，次参数为此人所对应的 QQ 号，若无对应则填写为空 */
        /* map.insert("NAME", "1145141919810"); */
        map
    };
}
```






