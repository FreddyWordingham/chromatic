use chromatic::Rgb;

#[test]
fn test_rgb_clone() {
    let original = Rgb::<f32>::new(0.5, 0.7, 0.3);
    let cloned = original.clone();

    // Verify the cloned value matches the original
    assert_eq!(original.red(), cloned.red());
    assert_eq!(original.green(), cloned.green());
    assert_eq!(original.blue(), cloned.blue());

    // Verify clone is a separate instance by modifying the original
    let mut original_mut = original;
    original_mut.set_red(0.2);
    original_mut.set_green(0.4);
    original_mut.set_blue(0.6);

    assert_eq!(original_mut.red(), 0.2);
    assert_eq!(original_mut.green(), 0.4);
    assert_eq!(original_mut.blue(), 0.6);

    assert_eq!(cloned.red(), 0.5);
    assert_eq!(cloned.green(), 0.7);
    assert_eq!(cloned.blue(), 0.3);
}

#[test]
fn test_rgb_copy() {
    let original = Rgb::<f32>::new(0.25, 0.50, 0.75);
    let copied = original;

    // Verify the copied value matches the original
    assert_eq!(original.red(), copied.red());
    assert_eq!(original.green(), copied.green());
    assert_eq!(original.blue(), copied.blue());

    // Original can still be used (Copy semantics)
    assert_eq!(original.red(), 0.25);
    assert_eq!(original.green(), 0.50);
    assert_eq!(original.blue(), 0.75);
}

#[test]
fn test_rgb_multiple_copies() {
    let original = Rgb::<f64>::new(0.333, 0.667, 0.5);

    let copy1 = original;
    let copy2 = original;
    let copy3 = copy1;

    assert_eq!(original.red(), copy1.red());
    assert_eq!(original.green(), copy1.green());
    assert_eq!(original.blue(), copy1.blue());

    assert_eq!(original.red(), copy2.red());
    assert_eq!(original.green(), copy2.green());
    assert_eq!(original.blue(), copy2.blue());

    assert_eq!(original.red(), copy3.red());
    assert_eq!(original.green(), copy3.green());
    assert_eq!(original.blue(), copy3.blue());
}
