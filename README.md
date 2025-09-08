# RGBLang 🎨

An esoteric programming language for generating RGB patterns, written in Rust.

## Features
- Simple syntax: `0`=Red, `1`=Green, `2`=Blue
- Pattern repetition with `R(n)`
- Real-time visual output

## Examples
```bash
# Input: "01,2R(2)"
# Output:
🔴🟢
🔵🔵🔵
```

also try: 1000,.0R(1)22,.0R(3),.1010 you won't regret it trust me 🗿

## Getting Started
```bash
git clone https://github.com/islamfazliyev/RGBLang.git
cd RGBLang
cargo run
```

## Syntax
- `0` = Red
- `1` = Green  
- `2` = Blue
- `,` = New line
- `R(n)` = Repeat previous pattern `n` times
- `.` = Breakpoint (start repetition from here)

## 🔍 See Also

For a more advanced implementation with additional features like compound expressions and professional error handling, check out:
- [rgblang2](https://github.com/cbarrick/rgblang2) by [@cbarrick](https://github.com/cbarrick)
---

*Created with passion for esoteric languages and Rust!* 🦀

---

