#[test]
fn percentage_multiple_nested_with_padding_margin_and_percentage_values() {
    let mut taffy = taffy::Taffy::new();
    let node000 = taffy
        .new_node(
            taffy::style::Style {
                size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.45f32), ..Default::default() },
                margin: taffy::geometry::Rect {
                    start: taffy::style::Dimension::Percent(0.05f32),
                    end: taffy::style::Dimension::Percent(0.05f32),
                    top: taffy::style::Dimension::Percent(0.05f32),
                    bottom: taffy::style::Dimension::Percent(0.05f32),
                    ..Default::default()
                },
                padding: taffy::geometry::Rect {
                    start: taffy::style::Dimension::Points(3f32),
                    end: taffy::style::Dimension::Points(3f32),
                    top: taffy::style::Dimension::Points(3f32),
                    bottom: taffy::style::Dimension::Points(3f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node00 = taffy
        .new_node(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Percent(0.5f32), ..Default::default() },
                margin: taffy::geometry::Rect {
                    start: taffy::style::Dimension::Points(5f32),
                    end: taffy::style::Dimension::Points(5f32),
                    top: taffy::style::Dimension::Points(5f32),
                    bottom: taffy::style::Dimension::Points(5f32),
                    ..Default::default()
                },
                padding: taffy::geometry::Rect {
                    start: taffy::style::Dimension::Percent(0.03f32),
                    end: taffy::style::Dimension::Percent(0.03f32),
                    top: taffy::style::Dimension::Percent(0.03f32),
                    bottom: taffy::style::Dimension::Percent(0.03f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node000],
        )
        .unwrap();
    let node0 = taffy
        .new_node(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: taffy::style::Dimension::Percent(0.1f32),
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(0.6f32),
                    ..Default::default()
                },
                margin: taffy::geometry::Rect {
                    start: taffy::style::Dimension::Points(5f32),
                    end: taffy::style::Dimension::Points(5f32),
                    top: taffy::style::Dimension::Points(5f32),
                    bottom: taffy::style::Dimension::Points(5f32),
                    ..Default::default()
                },
                padding: taffy::geometry::Rect {
                    start: taffy::style::Dimension::Points(3f32),
                    end: taffy::style::Dimension::Points(3f32),
                    top: taffy::style::Dimension::Points(3f32),
                    bottom: taffy::style::Dimension::Points(3f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node1 = taffy
        .new_node(
            taffy::style::Style {
                flex_grow: 4f32,
                flex_basis: taffy::style::Dimension::Percent(0.15f32),
                min_size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Percent(0.2f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = taffy
        .new_node(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Points(200f32),
                    height: taffy::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::undefined()).unwrap();
    assert_eq!(taffy.layout(node).unwrap().size.width, 200f32);
    assert_eq!(taffy.layout(node).unwrap().size.height, 200f32);
    assert_eq!(taffy.layout(node).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node).unwrap().location.y, 0f32);
    assert_eq!(taffy.layout(node0).unwrap().size.width, 190f32);
    assert_eq!(taffy.layout(node0).unwrap().size.height, 48f32);
    assert_eq!(taffy.layout(node0).unwrap().location.x, 5f32);
    assert_eq!(taffy.layout(node0).unwrap().location.y, 5f32);
    assert_eq!(taffy.layout(node00).unwrap().size.width, 92f32);
    assert_eq!(taffy.layout(node00).unwrap().size.height, 25f32);
    assert_eq!(taffy.layout(node00).unwrap().location.x, 8f32);
    assert_eq!(taffy.layout(node00).unwrap().location.y, 8f32);
    assert_eq!(taffy.layout(node000).unwrap().size.width, 36f32);
    assert_eq!(taffy.layout(node000).unwrap().size.height, 6f32);
    assert_eq!(taffy.layout(node000).unwrap().location.x, 10f32);
    assert_eq!(taffy.layout(node000).unwrap().location.y, 10f32);
    assert_eq!(taffy.layout(node1).unwrap().size.width, 200f32);
    assert_eq!(taffy.layout(node1).unwrap().size.height, 142f32);
    assert_eq!(taffy.layout(node1).unwrap().location.x, 0f32);
    assert_eq!(taffy.layout(node1).unwrap().location.y, 58f32);
}
