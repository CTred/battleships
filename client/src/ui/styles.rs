use kayak_ui::core::{
    styles::{Corner, Edge, LayoutType, Style, StyleProp, Units},
    Color, CursorIcon,
};

// const COLOR_PRIMARY: (f32, f32, f32, f32) = (44., 54., 57., 0.);
pub const COLOR_PRIMARY: (f32, f32, f32, f32) = (44., 54., 57., 1.);
pub const COLOR_SECONDARY: (f32, f32, f32, f32) = (63., 78., 79., 1.);
// pub const COLOR_ALERT: (f32, f32, f32, f32) = (162., 123., 92., 1.);
pub const COLOR_TEXT: (f32, f32, f32, f32) = (220., 215., 201., 1.);

#[derive(Debug, Clone, Eq, PartialEq)]
enum AnchorPoint {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

fn anchor(size: (f32, f32), parent: (f32, f32), anchor_point: AnchorPoint) -> (f32, f32) {
    match anchor_point {
        AnchorPoint::TopLeft => (0., 0.),
        AnchorPoint::TopCenter => (parent.0 / 2. - size.0 / 2., 0.),
        AnchorPoint::TopRight => (parent.0 - size.0, 0.),
        AnchorPoint::CenterLeft => (0., parent.1 / 2. - size.1 / 2.),
        AnchorPoint::Center => (parent.0 / 2. - size.0 / 2., parent.1 / 2. - size.1 / 2.),
        AnchorPoint::CenterRight => (parent.0 - size.0, parent.1 / 2. - size.1 / 2.),
        AnchorPoint::BottomLeft => (0., parent.1 - size.1),
        AnchorPoint::BottomCenter => (parent.1 / 2. - size.0 / 2., parent.1 - size.1),
        AnchorPoint::BottomRight => (parent.0 - size.0, parent.1 - size.1),
    }
}

pub fn bg_primary() -> Style {
    let (r, g, b, a) = COLOR_PRIMARY;
    Style {
        background_color: StyleProp::Value(Color::new(r / 256., g / 256., b / 256., a)),
        ..Default::default()
    }
}

pub fn bg_secondary() -> Style {
    let (r, g, b, a) = COLOR_SECONDARY;
    Style {
        background_color: StyleProp::Value(Color::new(r / 256., g / 256., b / 256., a)),
        ..Default::default()
    }
}

pub fn debug() -> Style {
    Style {
        border_color: StyleProp::Value(Color::new(0., 1., 0., 1.)),
        border: StyleProp::Value(Edge::all(2.0)),
        ..Default::default()
    }
}

pub fn container_style() -> Style {
    Style {
        height: StyleProp::Value(Units::Pixels(500.0)),
        width: StyleProp::Value(Units::Pixels(360.0)),

        // background_color: StyleProp::Value(Color::WHITE),
        layout_type: StyleProp::Value(LayoutType::Column),
        border_radius: StyleProp::Value(Corner::all(15.0)),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        row_between: StyleProp::Value(Units::Pixels(20.0)),
        ..Default::default()
    }
}

pub fn image_styles() -> Style {
    Style {
        width: StyleProp::Value(Units::Pixels(20.)),
        height: StyleProp::Value(Units::Pixels(20.)),
        border_radius: StyleProp::Value(Corner::all(20.0)),
        ..Style::default()
    }
}

pub fn button_style() -> Style {
    Style {
        background_color: StyleProp::Value(Color::BLACK),
        height: StyleProp::Value(Units::Pixels(50.0)),
        width: StyleProp::Value(Units::Pixels(200.0)),
        padding_top: StyleProp::Value(Units::Stretch(1.0)),
        padding_bottom: StyleProp::Value(Units::Stretch(1.0)),
        cursor: CursorIcon::Hand.into(),
        ..Default::default()
    }
}

pub fn center() -> Style {
    Style {
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        left: StyleProp::Value(Units::Stretch(1.0)),
        right: StyleProp::Value(Units::Stretch(1.0)),
        ..Default::default()
    }
}

pub fn center_top() -> Style {
    Style {
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        // top: StyleProp::Value(Units::Stretch(1.0)),
        left: StyleProp::Value(Units::Stretch(1.0)),
        right: StyleProp::Value(Units::Stretch(1.0)),
        ..Default::default()
    }
}

pub fn row() -> Style {
    Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        ..Default::default()
    }
}

pub fn col() -> Style {
    Style {
        layout_type: StyleProp::Value(LayoutType::Column),
        ..Default::default()
    }
}

pub fn center_left() -> Style {
    Style {
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        // left: StyleProp::Value(Units::Stretch(1.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        right: StyleProp::Value(Units::Stretch(1.0)),
        ..Default::default()
    }
}

pub fn center_right() -> Style {
    Style {
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        left: StyleProp::Value(Units::Stretch(1.0)),
        // right: StyleProp::Value(Units::Stretch(1.0)),
        ..Default::default()
    }
}
