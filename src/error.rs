/*!
# 简单常见错误定义

- 统一错误定义。使用 anyhow::Error。
- 默认使用统一错误的Result重新封装。
- 简单错误携带文本错误信息，封装成 fail 宏。
- 进一步封装成为 Result::Err的err宏。
- 异常退出的宏 bail。

统一的错误处理模块，提供通用错误模式的封装，并定义了一系列常见的错误。

- AlreadyExists: 存在性检查冲突。
- Broken：channel, socket 等信息通道断开，或者某些机制损坏。
- Cancel：被撤销或者中止的事务及操作。
- Invalid：所有格式错误，及非正常操作。
- Nil：代表的意义比较广泛，包括
  - 存在性检查时，not found；
  - 值为空。
  - 无意义的操作。
- Refused：被拒绝或者权限错误造成的访问限制。
- Timeout：超时错误。
- Unexpected: 含混的错误，发生了非预期的异常。
- Unimplemented：未实现的方法及函数。
- Unknown：未知错误，与Unexpected类似（未来考虑合并）。

# 代码

- - -

```rust
//
*/

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Already exists.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Exception;
impl Display for Exception {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "unexpected")
    }
}
impl Error for Exception {}

/*
```
*/
