use chromatic::Grey;

#[test]
fn test_grey_clone() {
    let original = Grey::<f32>::new(0.5);
    let cloned = original.clone();

    // Verify the cloned value matches the original
    assert_eq!(original.grey(), cloned.grey());

    // Verify clone is a separate instance by modifying the original
    let mut original_mut = original;
    original_mut.set_grey(0.75);

    assert_eq!(original_mut.grey(), 0.75);
    assert_eq!(cloned.grey(), 0.5);
}

#[test]
fn test_grey_copy() {
    let original = Grey::<f32>::new(0.25);
    let copied = original;

    // Verify the copied value matches the original
    assert_eq!(original.grey(), copied.grey());

    // Original can still be used (Copy semantics)
    assert_eq!(original.grey(), 0.25);
}

#[test]
fn test_grey_multiple_copies() {
    let original = Grey::<f64>::new(0.333);

    let copy1 = original;
    let copy2 = original;
    let copy3 = copy1;

    assert_eq!(original.grey(), copy1.grey());
    assert_eq!(original.grey(), copy2.grey());
    assert_eq!(original.grey(), copy3.grey());
}
