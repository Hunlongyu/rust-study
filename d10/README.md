## 写入单元测试

Rust 中的单元测试是用 `#[test]` 属性标记的简单函数，可用于验证非测试代码是否按预期方式正常运行。系统仅会在测试代码时编译这些函数。

```rust
#[test]
fn add_fails() {
    assert_eq!(add(2, 2), 7);
}
```

### 预期的失败

在许多情况下，务必要测试某种条件是否会导致 `panic!`。

使用 `should_panic`，便可以检查 `panic!`。如果将此属性添加到测试函数，则当函数中的代码崩溃时，测试便会通过。当代码不崩溃时，测试便会失败

```rust
#[test]
#[should_panic]
fn add_fails() {
    assert_eq!(add(2, 2), 7);
}
```

### 忽略测试

我们还可以使用 `[ignore]` 属性对带有 `[test]` 属性批注的函数进行批注。 此属性会令系统在测试过程中跳过该测试函数。

你可出于忽略测试的原因，选择性地写入 `[ignore]` 属性。

```rust
#[test]
#[ignore = "not yet reviewed by the Q.A. team"]
fn add_negatives() {
    assert_eq!(add(-2, -2), -4)
}
```

### 测试模块

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod add_function_tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 12), 22);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[should_panic]
    fn add_fails() {
        assert_eq!(add(2, 2), 7);
    }

    #[test]
    #[ignore]
    fn add_negatives() {
        assert_eq!(add(-2, -2), -4)
    }
}
```