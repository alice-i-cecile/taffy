#[test]
fn flex_grow_within_constrained_max_row() {
    let mut taffy = taffy::Taffy::new();
    let node00 = taffy
        .new_node(
            taffy::style::Style {
                flex_shrink: 1f32,
                flex_basis: taffy::style::Dimension::Points(100f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node01 = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(50f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node0 = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size { height: taffy::style::Dimension::Points(100f32), ..Default::default() },
                max_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node00, node01],
        )
        .unwrap();
    let node = taffy
        .new_node(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Points(200f32), ..Default::default() },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 200f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 100f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().size.width, 67f32);
    assert_eq!(taffy.layout(node00).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node00).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node00).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node01).unwrap().size.width, 33f32);
    assert_eq!(taffy.layout(node01).unwrap().size.height, 100f32);
    assert_eq!(taffy.layout(node01).unwrap().location.x, 67f32);
    assert_eq!(taffy.layout(node01).unwrap().location.y, 0f32);
}
