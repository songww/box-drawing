use box_drawing_derive::Parameters;

#[derive(Parameters)]
struct Param<F> {
    b: F,
    a: i32,
    c: Option<F>,
    d: Option<i32>,
}

#[test]
fn test_basic() {
    let p: Param<f32> = Param::new(1., 2).c(-3.).d(-4);
    assert_eq!(p.b, 1.);
    assert_eq!(p.a, 2);
    assert_eq!(p.c, Some(-3.));
    assert_eq!(p.d, Some(-4));
}
