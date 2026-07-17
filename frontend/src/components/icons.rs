use leptos::prelude::*;

pub type Icon = AnyView;

fn svg(path_content: &str, width: u32, height: u32, stroke_width: &str) -> Icon {
    let path_content = path_content.to_string();
    let stroke_width = stroke_width.to_string();
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" width=width height=height viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width=stroke_width stroke-linecap="round" stroke-linejoin="round"
            inner_html=path_content />
    }.into_any()
}

pub fn icon_check_square() -> Icon {
    svg(
        r#"<rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><path d="m9 12 2 2 4-4"/>"#,
        16,
        16,
        "2.5",
    )
}

pub fn icon_refresh_cw() -> Icon {
    svg(
        r#"<path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M8 16H3v5"/>"#,
        10,
        10,
        "2",
    )
}

pub fn icon_layout_dashboard() -> Icon {
    svg(
        r#"<rect width="7" height="9" x="3" y="3" rx="1"/><rect width="7" height="5" x="14" y="3" rx="1"/><rect width="7" height="9" x="14" y="12" rx="1"/><rect width="7" height="5" x="3" y="16" rx="1"/>"#,
        14,
        14,
        "2",
    )
}

pub fn icon_trending_up() -> Icon {
    svg(
        r#"<polyline points="22 7 13.5 15.5 8.5 10.5 2 17"/><polyline points="16 7 22 7 22 13"/>"#,
        12,
        12,
        "2",
    )
}

pub fn icon_layers() -> Icon {
    svg(
        r#"<path d="m12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83Z"/><path d="m6.08 9.5-3.5 1.6a1 1 0 0 0 0 1.81l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9a1 1 0 0 0 0-1.83l-3.5-1.59"/><path d="m6.08 14.5-3.5 1.6a1 1 0 0 0 0 1.81l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9a1 1 0 0 0 0-1.83l-3.5-1.59"/>"#,
        18,
        18,
        "2",
    )
}

pub fn icon_cloud_lightning() -> Icon {
    svg(
        r#"<path d="M6 16.326A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 .5 8.973"/><path d="m13 12-3 5h4l-3 5"/>"#,
        14,
        14,
        "2",
    )
}

pub fn icon_plus() -> Icon {
    svg(r#"<path d="M5 12h14"/><path d="M12 5v14"/>"#, 14, 14, "2")
}

pub fn icon_chevron_down() -> Icon {
    svg(r#"<path d="m6 9 6 6 6-6"/>"#, 13, 13, "2")
}

pub fn icon_chevron_up() -> Icon {
    svg(r#"<path d="m18 15-6-6-6 6"/>"#, 13, 13, "2")
}

pub fn icon_alert_circle() -> Icon {
    svg(
        r#"<circle cx="12" cy="12" r="10"/><line x1="12" x2="12" y1="8" y2="12"/><line x1="12" x2="12.01" y1="16" y2="16"/>"#,
        12,
        12,
        "2",
    )
}

pub fn icon_search() -> Icon {
    svg(
        r#"<circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>"#,
        16,
        16,
        "2",
    )
}

pub fn icon_sliders_horizontal() -> Icon {
    svg(
        r#"<line x1="21" x2="14" y1="4" y2="4"/><line x1="10" x2="3" y1="4" y2="4"/><line x1="21" x2="12" y1="12" y2="12"/><line x1="8" x2="3" y1="12" y2="12"/><line x1="21" x2="16" y1="20" y2="20"/><line x1="12" x2="3" y1="20" y2="20"/><line x1="14" x2="14" y1="2" y2="6"/><line x1="8" x2="8" y1="10" y2="14"/><line x1="16" x2="16" y1="18" y2="22"/>"#,
        13,
        13,
        "2",
    )
}

pub fn icon_arrow_up_down() -> Icon {
    svg(
        r#"<path d="m21 16-4 4-4-4"/><path d="M17 20V4"/><path d="m3 8 4-4 4 4"/><path d="M7 4v16"/>"#,
        13,
        13,
        "2",
    )
}

pub fn icon_trash2() -> Icon {
    svg(
        r#"<path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/><line x1="10" x2="10" y1="11" y2="17"/><line x1="14" x2="14" y1="11" y2="17"/>"#,
        12,
        12,
        "2",
    )
}

pub fn icon_trash2_sm() -> Icon {
    svg(
        r#"<path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/><line x1="10" x2="10" y1="11" y2="17"/><line x1="14" x2="14" y1="11" y2="17"/>"#,
        14,
        14,
        "2",
    )
}

pub fn icon_edit() -> Icon {
    svg(
        r#"<path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"/><path d="m15 5 4 4"/>"#,
        14,
        14,
        "2",
    )
}

pub fn icon_check() -> Icon {
    svg(r#"<path d="M20 6 9 17l-5-5"/>"#, 11, 11, "3")
}

pub fn icon_check_13() -> Icon {
    svg(r#"<path d="M20 6 9 17l-5-5"/>"#, 13, 13, "2")
}

pub fn icon_x() -> Icon {
    svg(
        r#"<path d="M18 6 6 18"/><path d="m6 6 12 12"/>"#,
        13,
        13,
        "2",
    )
}

pub fn icon_eye() -> Icon {
    svg(
        r#"<path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/>"#,
        14,
        14,
        "2",
    )
}

pub fn icon_eye_off() -> Icon {
    svg(
        r#"<path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"/><path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"/><path d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"/><line x1="2" x2="22" y1="2" y2="22"/>"#,
        14,
        14,
        "2",
    )
}

pub fn icon_tag() -> Icon {
    svg(
        r#"<path d="M12.586 2.586A2 2 0 0 0 11.172 2H4a2 2 0 0 0-2 2v7.172a2 2 0 0 0 .586 1.414l8.704 8.704a2.426 2.426 0 0 0 3.42 0l6.58-6.58a2.426 2.426 0 0 0 0-3.42z"/><circle cx="7.5" cy="7.5" r=".5" fill="currentColor"/>"#,
        9,
        9,
        "2",
    )
}

pub fn icon_calendar() -> Icon {
    svg(
        r#"<path d="M8 2v4"/><path d="M16 2v4"/><rect width="18" height="18" x="3" y="4" rx="2"/><path d="M3 10h18"/>"#,
        9,
        9,
        "2",
    )
}

pub fn icon_calendar_11() -> Icon {
    svg(
        r#"<path d="M8 2v4"/><path d="M16 2v4"/><rect width="18" height="18" x="3" y="4" rx="2"/><path d="M3 10h18"/>"#,
        11,
        11,
        "2",
    )
}

pub fn icon_bar_chart() -> Icon {
    svg(
        r#"<line x1="12" x2="12" y1="20" y2="10"/><line x1="18" x2="18" y1="20" y2="4"/><line x1="6" x2="6" y1="20" y2="14"/>"#,
        15,
        15,
        "2",
    )
}

pub fn icon_check_circle() -> Icon {
    svg(
        r#"<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><path d="m9 11 3 3L22 4"/>"#,
        15,
        15,
        "2",
    )
}

pub fn icon_alert_triangle() -> Icon {
    svg(
        r#"<path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><path d="M12 9v4"/><path d="M12 17h.01"/>"#,
        15,
        15,
        "2",
    )
}

pub fn icon_clock() -> Icon {
    svg(
        r#"<circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/>"#,
        15,
        15,
        "2",
    )
}
