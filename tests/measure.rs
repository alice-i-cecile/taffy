#[cfg(test)]
mod measure {
    use taffy::node::MeasureFunc;
    use taffy::number::OrElse;

    #[test]
    fn measure_root() {
        let mut taffy = taffy::node::Taffy::new();
        let node = taffy
            .new_leaf(
                taffy::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| taffy::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child() {
        let mut taffy = taffy::node::Taffy::new();

        let child = taffy
            .new_leaf(
                taffy::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| taffy::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = taffy.new_node(taffy::style::Style { ..Default::default() }, &[child]).unwrap();
        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint() {
        let mut taffy = taffy::node::Taffy::new();
        let child = taffy
            .new_leaf(
                taffy::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| taffy::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = taffy
            .new_node(
                taffy::style::Style {
                    size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50.0), ..Default::default() },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 100.0);

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_constraint_padding_parent() {
        let mut taffy = taffy::node::Taffy::new();
        let child = taffy
            .new_leaf(
                taffy::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| taffy::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = taffy
            .new_node(
                taffy::style::Style {
                    size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50.0), ..Default::default() },
                    padding: taffy::geometry::Rect {
                        start: taffy::style::Dimension::Points(10.0),
                        end: taffy::style::Dimension::Points(10.0),
                        top: taffy::style::Dimension::Points(10.0),
                        bottom: taffy::style::Dimension::Points(10.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();
        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(node).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(node).unwrap().size.height, 120.0);

        assert_eq!(taffy.layout(child).unwrap().size.width, 30.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_child_with_flex_grow() {
        let mut taffy = taffy::node::Taffy::new();
        let child0 = taffy
            .new_node(
                taffy::style::Style {
                    size: taffy::geometry::Size {
                        width: taffy::style::Dimension::Points(50.0),
                        height: taffy::style::Dimension::Points(50.0),
                    },
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        let child1 = taffy
            .new_leaf(
                taffy::style::Style { flex_grow: 1.0, ..Default::default() },
                MeasureFunc::Raw(|constraint| taffy::geometry::Size {
                    width: constraint.width.or_else(10.0),
                    height: constraint.height.or_else(50.0),
                }),
            )
            .unwrap();

        let node = taffy
            .new_node(
                taffy::style::Style {
                    size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100.0), ..Default::default() },
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn measure_child_with_flex_shrink() {
        let mut taffy = taffy::node::Taffy::new();
        let child0 = taffy
            .new_node(
                taffy::style::Style {
                    size: taffy::geometry::Size {
                        width: taffy::style::Dimension::Points(50.0),
                        height: taffy::style::Dimension::Points(50.0),
                    },
                    flex_shrink: 0.0,
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        let child1 = taffy
            .new_leaf(
                taffy::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| taffy::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(50.0),
                }),
            )
            .unwrap();

        let node = taffy
            .new_node(
                taffy::style::Style {
                    size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100.0), ..Default::default() },
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 50.0);
    }

    #[test]
    fn remeasure_child_after_growing() {
        let mut taffy = taffy::node::Taffy::new();
        let child0 = taffy
            .new_node(
                taffy::style::Style {
                    size: taffy::geometry::Size {
                        width: taffy::style::Dimension::Points(50.0),
                        height: taffy::style::Dimension::Points(50.0),
                    },
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        let child1 = taffy
            .new_leaf(
                taffy::style::Style { flex_grow: 1.0, ..Default::default() },
                MeasureFunc::Raw(|constraint| {
                    let width = constraint.width.or_else(10.0);
                    let height = constraint.height.or_else(width * 2.0);
                    taffy::geometry::Size { width, height }
                }),
            )
            .unwrap();

        let node = taffy
            .new_node(
                taffy::style::Style {
                    size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100.0), ..Default::default() },
                    align_items: taffy::style::AlignItems::FlexStart,
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn remeasure_child_after_shrinking() {
        let mut taffy = taffy::node::Taffy::new();

        let child0 = taffy
            .new_node(
                taffy::style::Style {
                    size: taffy::geometry::Size {
                        width: taffy::style::Dimension::Points(50.0),
                        height: taffy::style::Dimension::Points(50.0),
                    },
                    flex_shrink: 0.0,
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        let child1 = taffy
            .new_leaf(
                taffy::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| {
                    let width = constraint.width.or_else(100.0);
                    let height = constraint.height.or_else(width * 2.0);
                    taffy::geometry::Size { width, height }
                }),
            )
            .unwrap();

        let node = taffy
            .new_node(
                taffy::style::Style {
                    size: taffy::geometry::Size { width: taffy::style::Dimension::Points(100.0), ..Default::default() },
                    align_items: taffy::style::AlignItems::FlexStart,
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child1).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn remeasure_child_after_stretching() {
        let mut taffy = taffy::node::Taffy::new();

        let child = taffy
            .new_leaf(
                taffy::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| {
                    let height = constraint.height.or_else(50.0);
                    let width = constraint.width.or_else(height);
                    taffy::geometry::Size { width, height }
                }),
            )
            .unwrap();

        let node = taffy
            .new_node(
                taffy::style::Style {
                    size: taffy::geometry::Size {
                        width: taffy::style::Dimension::Points(100.0),
                        height: taffy::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn width_overrides_measure() {
        let mut taffy = taffy::node::Taffy::new();
        let child = taffy
            .new_leaf(
                taffy::style::Style {
                    size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50.0), ..Default::default() },
                    ..Default::default()
                },
                MeasureFunc::Raw(|constraint| taffy::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = taffy.new_node(taffy::style::Style { ..Default::default() }, &[child]).unwrap();
        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn height_overrides_measure() {
        let mut taffy = taffy::node::Taffy::new();
        let child = taffy
            .new_leaf(
                taffy::style::Style {
                    size: taffy::geometry::Size { height: taffy::style::Dimension::Points(50.0), ..Default::default() },
                    ..Default::default()
                },
                MeasureFunc::Raw(|constraint| taffy::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = taffy.new_node(taffy::style::Style { ..Default::default() }, &[child]).unwrap();
        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 50.0);
    }

    #[test]
    fn flex_basis_overrides_measure() {
        let mut taffy = taffy::node::Taffy::new();
        let child0 = taffy
            .new_node(
                taffy::style::Style {
                    flex_basis: taffy::style::Dimension::Points(50.0),
                    flex_grow: 1.0,
                    ..Default::default()
                },
                &[],
            )
            .unwrap();

        let child1 = taffy
            .new_leaf(
                taffy::style::Style {
                    flex_basis: taffy::style::Dimension::Points(50.0),
                    flex_grow: 1.0,
                    ..Default::default()
                },
                MeasureFunc::Raw(|constraint| taffy::geometry::Size {
                    width: constraint.width.or_else(100.0),
                    height: constraint.height.or_else(100.0),
                }),
            )
            .unwrap();

        let node = taffy
            .new_node(
                taffy::style::Style {
                    size: taffy::geometry::Size {
                        width: taffy::style::Dimension::Points(200.0),
                        height: taffy::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child0, child1],
            )
            .unwrap();

        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child0).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child0).unwrap().size.height, 100.0);
        assert_eq!(taffy.layout(child1).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child1).unwrap().size.height, 100.0);
    }

    #[test]
    fn stretch_overrides_measure() {
        let mut taffy = taffy::node::Taffy::new();
        let child = taffy
            .new_leaf(
                taffy::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| taffy::geometry::Size {
                    width: constraint.width.or_else(50.0),
                    height: constraint.height.or_else(50.0),
                }),
            )
            .unwrap();

        let node = taffy
            .new_node(
                taffy::style::Style {
                    size: taffy::geometry::Size {
                        width: taffy::style::Dimension::Points(100.0),
                        height: taffy::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn measure_absolute_child() {
        let mut taffy = taffy::node::Taffy::new();
        let child = taffy
            .new_leaf(
                taffy::style::Style { position_type: taffy::style::PositionType::Absolute, ..Default::default() },
                MeasureFunc::Raw(|constraint| taffy::geometry::Size {
                    width: constraint.width.or_else(50.0),
                    height: constraint.height.or_else(50.0),
                }),
            )
            .unwrap();

        let node = taffy
            .new_node(
                taffy::style::Style {
                    size: taffy::geometry::Size {
                        width: taffy::style::Dimension::Points(100.0),
                        height: taffy::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 50.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 50.0);
    }

    #[test]
    fn ignore_invalid_measure() {
        let mut taffy = taffy::node::Taffy::new();
        let child = taffy
            .new_leaf(
                taffy::style::Style { flex_grow: 1.0, ..Default::default() },
                MeasureFunc::Raw(|_| taffy::geometry::Size { width: 200.0, height: 200.0 }),
            )
            .unwrap();

        let node = taffy
            .new_node(
                taffy::style::Style {
                    size: taffy::geometry::Size {
                        width: taffy::style::Dimension::Points(100.0),
                        height: taffy::style::Dimension::Points(100.0),
                    },
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();

        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
        assert_eq!(taffy.layout(child).unwrap().size.height, 100.0);
    }

    #[test]
    fn only_measure_once() {
        use std::sync::atomic;

        let mut taffy = taffy::node::Taffy::new();
        static NUM_MEASURES: atomic::AtomicU32 = atomic::AtomicU32::new(0);

        let grandchild = taffy
            .new_leaf(
                taffy::style::Style { ..Default::default() },
                MeasureFunc::Raw(|constraint| {
                    NUM_MEASURES.fetch_add(1, atomic::Ordering::Relaxed);
                    taffy::geometry::Size {
                        width: constraint.width.or_else(50.0),
                        height: constraint.height.or_else(50.0),
                    }
                }),
            )
            .unwrap();

        let child = taffy.new_node(taffy::style::Style { ..Default::default() }, &[grandchild]).unwrap();

        let node = taffy.new_node(taffy::style::Style { ..Default::default() }, &[child]).unwrap();
        taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();

        assert_eq!(NUM_MEASURES.load(atomic::Ordering::Relaxed), 2);
    }
}
