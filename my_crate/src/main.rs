//crate 是一个二进制项或者库
//一个包中至多 只能 包含一个库 crate(library crate)
//包中可以包含任意多个二进制 crate(binary crate)
//包中至少包含一个 crate，无论是库的还是二进制的。
//约定：src/main.rs 就是一个与包同名的二进制 crate 的 crate 根
//Cargo 知道如果包目录中包含 src/lib.rs，则包带有与其同名的库 crate，且 src/lib.rs 是 crate 根
//如果一个包同时含有 src/main.rs 和 src/lib.rs，则它有两个 crate：一个库和一个二进制项，且名字都与包相同
fn main() {}