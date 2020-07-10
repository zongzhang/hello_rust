//另一个没有所有权的数据类型是 slice。slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice类型引用 &str
fn slice()
{
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[..2];

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    let slice = &s[0..len];
    let slice = &s[..];
}