use crate::app::Message;
use cosmic::{iced, iced::Color, Theme, Element, widget};
use cosmic::iced::Length;
use cosmic::iced::widget::{pick_list, image};
use cosmic::widget::{icon, button, text};
use cosmic::widget::container;


pub fn page1<'a>() -> Element<'a, Message> {
    let header = widget::text::title2("Typography Examples")
        .size(28);

    let intro = widget::text("The text module provides standard typography presets for COSMIC applications.")
        .size(16);

    widget::column()
        .push(header)
        .push(widget::container(intro).padding([0, 0, 20, 0]))
        .push(widget::text::title1("Title 1 Example"))
        .push(widget::text::title2("Title 2 Example"))
        .push(widget::text::title3("Title 3 Example"))
        .push(widget::text::heading("Heading Example"))
        .push(widget::text::body("Body Text Example"))
        .push(widget::text::caption_heading("Caption Heading Example"))
        .push(widget::text::caption("Caption Example"))
        .push(widget::text::monotext("Monotext Example"))
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page2<'a>() -> Element<'a, Message> {
    let header = widget::text::title2("Container, Column & Row Examples");
    
    // Container example
    let container_demo = widget::container(
        widget::text("This text is in a container with padding")
    )
    .padding(20)
    .width(Length::Fill);

    // Column example
    let column_demo = widget::column()
        .push(widget::text::title3("Column Items"))
        .push(widget::text("Item 1"))
        .push(widget::text("Item 2"))
        .push(widget::text("Item 3"))
        .spacing(10)
        .width(Length::Fill);

    // Row example
    let row_demo = widget::row()
        .push(widget::button::standard("Button 1").on_press(Message::SubscriptionChannel))
        .push(widget::button::standard("Button 2").on_press(Message::SubscriptionChannel))
        .push(widget::button::standard("Button 3").on_press(Message::SubscriptionChannel))
        .spacing(10)
        .width(Length::Fill);

    widget::column()
        .push(header)
        .push(container_demo)
        .push(column_demo)
        .push(row_demo)
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page3<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Button Examples"))
        .push(widget::text::heading("Standard Buttons"))
        .push(widget::text("Buttons will trigger the SubscriptionChannel message"))
        .push(
            widget::row()
                .push(widget::button::standard("Regular").on_press(Message::SubscriptionChannel))
                .push(widget::button::standard("Another").on_press(Message::SubscriptionChannel))
                .push(widget::button::standard("Third").on_press(Message::SubscriptionChannel))
                .spacing(10)
        )
        .push(widget::text::heading("Text Buttons"))
        .push(
            widget::row()
                .push(widget::button::text("Text Button 1").on_press(Message::SubscriptionChannel))
                .push(widget::button::text("Text Button 2").on_press(Message::SubscriptionChannel))
                .push(widget::button::text("Text Button 3").on_press(Message::SubscriptionChannel))
                .spacing(10)
        )
        .push(widget::text::heading("Link Buttons"))
        .push(widget::button::link("Click this link").on_press(Message::SubscriptionChannel))
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
}



pub fn page4<'a>() -> Element<'a, Message> {
    let header = widget::text::title2("Icons, Images & SVG Examples");
    
    // Icon examples section
    let icon_section = widget::column()
        .push(widget::text::heading("Icons"))
        .push(
            widget::row()
                .push(icon::from_name("applications-system-symbolic"))
                .push(icon::from_name("edit-copy-symbolic"))
                .push(icon::from_name("document-new-symbolic"))
                .push(icon::from_name("folder-symbolic"))
                .spacing(20)
        )
        .push(
            widget::row()
                .push(icon::from_name("user-trash-symbolic").size(32))
                .push(icon::from_name("system-search-symbolic").size(32))
                .push(icon::from_name("preferences-system-symbolic").size(32))
                .spacing(20)
        );

    // Image examples section
    let image_section = widget::column()
        .push(widget::text::heading("Images"))
        .push(
            widget::row()
                .push(image::viewer(image::Handle::from_path("assets/sample1.jpg")))
                .push(image::viewer(image::Handle::from_path("assets/sample2.jpg")))
                .spacing(20)
        );

    // SVG examples section
    let svg_data = r#"<svg viewBox="0 0 100 100">
        <circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red"/>
    </svg>"#;
    
    let svg_section = widget::column()
        .push(widget::text::heading("SVG Graphics"))
        .push(
            widget::row()
                .push(widget::svg(widget::svg::Handle::from_memory(svg_data.as_bytes())))
                .push(widget::svg(widget::svg::Handle::from_path("assets/icon.svg")))
                .spacing(20)
        );

    // Main layout
    widget::column()
        .push(header)
        .push(icon_section)
        .push(widget::text("")) // Spacer instead of rule
        .push(image_section)
        .push(widget::text("")) // Spacer instead of rule
        .push(svg_section)
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
}



pub fn page5<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Divider & Space"))
        .push(widget::text("Item 1"))
        .push(widget::container(widget::text("")).height(Length::Fixed(20.0)))
        .push(widget::text("Item 2"))
        .push(widget::text("Item 3"))
        .push(widget::container(widget::text("")).height(Length::Fixed(40.0)))
        .push(widget::text("Item 4"))
        .width(Length::Fill)
        .padding(20)
        .into()
}

pub fn page6<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Text Input Examples"))
        .push(widget::text_input("Regular input", ""))
        .push(widget::text_input("Password", "").password())
        .push(widget::text_input("With placeholder", "Type here..."))
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page7<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Toggler, Slider, Radio"))
        .push(widget::checkbox("Check me", false))
        .push(widget::radio("Radio Option 1", "1", Some("1"), |_| Message::SubscriptionChannel))
        .push(widget::radio("Radio Option 2", "2", Some("1"), |_| Message::SubscriptionChannel))
        .push(widget::slider(
            0..=100,
            50,
            |_| Message::SubscriptionChannel
        ))
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page8<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Dropdown Examples"))
        .push(pick_list(
            &["Option 1", "Option 2", "Option 3"][..],
            Some("Option 1"),
            |_| Message::SubscriptionChannel
        ))
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
}



pub fn page9<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Spin Button"))
        .push(
            widget::container(
                widget::spin_button(
                    "Value",
                    50,
                    1,
                    0,
                    100,
                    |_| Message::SubscriptionChannel
                )
            )
            .padding(10)
            .style(|theme: &Theme| {
                let cosmic = theme.cosmic();
                container::Style {
                    text_color: Some(cosmic.accent.base.into()),
                    icon_color: Some(cosmic.accent.base.into()),
                    background: Some(Color { r: 0.2, g: 0.3, b: 0.8, a: 0.1 }.into()),
                    border: iced::Border {
                        radius: cosmic.corner_radii.radius_m.into(),
                        width: 1.0,
                        color: cosmic.accent.base.into(),
                    },
                    ..Default::default()
                }
            })
        )
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
 }

pub fn page10<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Color Selection"))
        .push(widget::text("Color selection controls"))
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page11<'a>() -> Element<'a, Message> {
    let flex_row = widget::row()
        .push(button::standard("Item 1").on_press(Message::SubscriptionChannel))
        .push(button::standard("Item 2").on_press(Message::SubscriptionChannel))
        .push(button::standard("Item 3").on_press(Message::SubscriptionChannel))
        .spacing(10);

    widget::column()
        .push(widget::text::title2("Flex Row Examples"))
        .push(flex_row)
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page12<'a>() -> Element<'a, Message> {
    let row1 = widget::row()
        .push(text("Cell 1"))
        .push(text("Cell 2"))
        .spacing(10);
        
    let row2 = widget::row()
        .push(text("Cell 3"))
        .push(text("Cell 4"))
        .spacing(10);

    widget::column()
        .push(widget::text::title2("Grid Layout"))
        .push(row1)
        .push(row2)
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page13<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Segmented Controls"))
        .push(widget::row()
            .push(button::standard("Option 1").on_press(Message::SubscriptionChannel))
            .push(button::standard("Option 2").on_press(Message::SubscriptionChannel))
            .push(button::standard("Option 3").on_press(Message::SubscriptionChannel))
            .spacing(10))
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page14<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Tab Bar Examples"))
        .push(widget::row()
            .push(button::standard("Tab 1").on_press(Message::SubscriptionChannel))
            .push(button::standard("Tab 2").on_press(Message::SubscriptionChannel))
            .push(button::standard("Tab 3").on_press(Message::SubscriptionChannel))
            .spacing(10))
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page15<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Segmented Controls"))
        .push(widget::row()
            .push(button::standard("Control 1").on_press(Message::SubscriptionChannel))
            .push(button::standard("Control 2").on_press(Message::SubscriptionChannel))
            .push(button::standard("Control 3").on_press(Message::SubscriptionChannel))
            .spacing(10))
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page16<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Context Menu Examples"))
        .push(button::standard("Right click me").on_press(Message::SubscriptionChannel))
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page17<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Panel Layout Examples"))
        .push(widget::row()
            .push(widget::container(widget::text("Panel 1")).padding(10))
            .push(widget::container(widget::text("Panel 2")).padding(10))
            .spacing(10))
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
}

