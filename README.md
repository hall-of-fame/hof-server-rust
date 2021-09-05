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
pub const AVATARS: [(&str, &str); 1] = [
    ("NAME", "1145141919"),
];
```

### `/src/service/popular.rs` (FILE)

```rust
use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct PopSticker<'a> {
    author: &'a str,
    desc: &'a str,
    url: &'a str,
}

pub fn get_popular<'a>() -> Vec<PopSticker<'a>> {
    vec![
        /* 以下注释代码可循环多次 */
        /* PopSticker {
            author: "AUTHOR",
            desc: "DESCRIPTION",
            url: "/static/DEPARTMENT/20/AUTHOR/DESCRIPTION.jpg",
        }, */
    ]
}
```






