use crate::app::Message;
use cosmic::{Element, widget};
use cosmic::iced::Length;

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
        // Standard buttons
        .push(widget::text::heading("Standard Buttons"))
        .push(
            widget::row()
                .push(widget::button::standard("Regular").on_press(Message::SubscriptionChannel))
                .push(widget::button::standard("Another").on_press(Message::SubscriptionChannel))
                .push(widget::button::standard("Third").on_press(Message::SubscriptionChannel))
                .spacing(10)
        )
        // Text buttons
        .push(widget::text::heading("Text Buttons"))
        .push(
            widget::row()
                .push(widget::button::text("Text Button 1").on_press(Message::SubscriptionChannel))
                .push(widget::button::text("Text Button 2").on_press(Message::SubscriptionChannel))
                .push(widget::button::text("Text Button 3").on_press(Message::SubscriptionChannel))
                .spacing(10)
        )
        // Link buttons
        .push(widget::text::heading("Link Buttons"))
        .push(widget::button::link("Click this link").on_press(Message::SubscriptionChannel))
        .spacing(20)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page4<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Interactive Elements"))
        .push(widget::text_input("Placeholder", ""))
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page5<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Interactive Elements"))
        .push(widget::text_input("Placeholder", ""))
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page6<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Interactive Elements"))
        .push(widget::text_input("Placeholder", ""))
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page7<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Interactive Elements"))
        .push(widget::text_input("Placeholder", ""))
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page8<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Interactive Elements"))
        .push(widget::text_input("Placeholder", ""))
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page9<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Interactive Elements"))
        .push(widget::text_input("Placeholder", ""))
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page10<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Interactive Elements"))
        .push(widget::text_input("Placeholder", ""))
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page11<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Interactive Elements"))
        .push(widget::text_input("Placeholder", ""))
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page12<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Interactive Elements"))
        .push(widget::text_input("Placeholder", ""))
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page13<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Interactive Elements"))
        .push(widget::text_input("Placeholder", ""))
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page14<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Interactive Elements"))
        .push(widget::text_input("Placeholder", ""))
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page15<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Interactive Elements"))
        .push(widget::text_input("Placeholder", ""))
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page16<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Interactive Elements"))
        .push(widget::text_input("Placeholder", ""))
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .into()
}

pub fn page17<'a>() -> Element<'a, Message> {
    widget::column()
        .push(widget::text::title2("Interactive Elements"))
        .push(widget::text_input("Placeholder", ""))
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .into()
}

