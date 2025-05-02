use chromatic::GreyAlpha;

#[test]
fn test_grey_alpha_clone() {
    let original = GreyAlpha::<f32>::new(0.5, 0.7);
    let cloned = original.clone();

    // Verify the cloned value matches the original
    assert_eq!(original.grey(), cloned.grey());
    assert_eq!(original.alpha(), cloned.alpha());

    // Verify clone is a separate instance by modifying the original
    let mut original_mut = original;
    original_mut.set_grey(0.75);
    original_mut.set_alpha(0.25);

    assert_eq!(original_mut.grey(), 0.75);
    assert_eq!(original_mut.alpha(), 0.25);
    assert_eq!(cloned.grey(), 0.5);
    assert_eq!(cloned.alpha(), 0.7);
}

#[test]
fn test_grey_alpha_copy() {
    let original = GreyAlpha::<f32>::new(0.25, 0.75);
    let copied = original;

    // Verify the copied value matches the original
    assert_eq!(original.grey(), copied.grey());
    assert_eq!(original.alpha(), copied.alpha());

    // Original can still be used (Copy semantics)
    assert_eq!(original.grey(), 0.25);
    assert_eq!(original.alpha(), 0.75);
}

#[test]
fn test_grey_alpha_multiple_copies() {
    let original = GreyAlpha::<f64>::new(0.333, 0.667);

    let copy1 = original;
    let copy2 = original;
    let copy3 = copy1;

    assert_eq!(original.grey(), copy1.grey());
    assert_eq!(original.alpha(), copy1.alpha());
    assert_eq!(original.grey(), copy2.grey());
    assert_eq!(original.alpha(), copy2.alpha());
    assert_eq!(original.grey(), copy3.grey());
    assert_eq!(original.alpha(), copy3.alpha());
}
