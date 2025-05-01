use chromatic::Rgba;

#[test]
fn test_rgba_clone() {
    let original = Rgba::<f32>::new(0.5, 0.7, 0.3, 0.8);
    let cloned = original.clone();

    // Verify the cloned value matches the original
    assert_eq!(original.red(), cloned.red());
    assert_eq!(original.green(), cloned.green());
    assert_eq!(original.blue(), cloned.blue());
    assert_eq!(original.alpha(), cloned.alpha());

    // Verify clone is a separate instance by modifying the original
    let mut original_mut = original;
    original_mut.set_red(0.2);
    original_mut.set_green(0.4);
    original_mut.set_blue(0.6);
    original_mut.set_alpha(0.9);

    assert_eq!(original_mut.red(), 0.2);
    assert_eq!(original_mut.green(), 0.4);
    assert_eq!(original_mut.blue(), 0.6);
    assert_eq!(original_mut.alpha(), 0.9);

    assert_eq!(cloned.red(), 0.5);
    assert_eq!(cloned.green(), 0.7);
    assert_eq!(cloned.blue(), 0.3);
    assert_eq!(cloned.alpha(), 0.8);
}

#[test]
fn test_rgba_copy() {
    let original = Rgba::<f32>::new(0.25, 0.50, 0.75, 0.80);
    let copied = original;

    // Verify the copied value matches the original
    assert_eq!(original.red(), copied.red());
    assert_eq!(original.green(), copied.green());
    assert_eq!(original.blue(), copied.blue());
    assert_eq!(original.alpha(), copied.alpha());

    // Original can still be used (Copy semantics)
    assert_eq!(original.red(), 0.25);
    assert_eq!(original.green(), 0.50);
    assert_eq!(original.blue(), 0.75);
    assert_eq!(original.alpha(), 0.80);
}

#[test]
fn test_rgba_multiple_copies() {
    let original = Rgba::<f64>::new(0.333, 0.667, 0.5, 0.25);

    let copy1 = original;
    let copy2 = original;
    let copy3 = copy1;

    assert_eq!(original.red(), copy1.red());
    assert_eq!(original.green(), copy1.green());
    assert_eq!(original.blue(), copy1.blue());
    assert_eq!(original.alpha(), copy1.alpha());

    assert_eq!(original.red(), copy2.red());
    assert_eq!(original.green(), copy2.green());
    assert_eq!(original.blue(), copy2.blue());
    assert_eq!(original.alpha(), copy2.alpha());

    assert_eq!(original.red(), copy3.red());
    assert_eq!(original.green(), copy3.green());
    assert_eq!(original.blue(), copy3.blue());
    assert_eq!(original.alpha(), copy3.alpha());
}
