fn main() {
    Some(2 + 4).unwrap();
    Some("abc").expect("not warned");

    let x = Ok::<f32, &str>(3.14);
    x.unwrap();

    let y = None;
    y.unwrap_or("こんにちは");
}
