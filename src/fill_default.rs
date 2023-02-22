//! 使用 ` ..Default::default() `快速填充结构体

#[derive(Debug)]
struct Example {
    p0: i32,
    p1: i32,
    p2: String,
    p3: Option<i32>,
}

/// 实现Default
impl Default for Example {
    fn default() -> Self {
        Example {
            p0: -1,
            p1: 100,
            p2: String::from("nice try"),
            p3: Some(200),
        }
    }
}

#[test]
fn test_fill_default() {
    let example = Example {
        p0: -200,
        ..Default::default() // 这里Default自动建立了一个默认的Example,填充了剩余未初始化的字段
    };
    dbg!(&example);
    assert_eq!(example.p0, -200);
    assert_eq!(example.p1, 100);
    assert_eq!(example.p2, String::from("nice try"));
    assert_eq!(example.p3, Some(200));
}