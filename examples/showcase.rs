use iced::{Element, Task, Theme, Length, Border, Color, Shadow, Background};
use iced::widget::{
    text, button, container, text_input, column, row, vertical_space, 
    horizontal_space, radio, checkbox, pick_list, scrollable, combo_box
};

use iced_modern_theme::Modern;
use iced_modern_theme::colors::colors;

fn main() -> iced::Result {
    iced::application("Modern Theme & Colors Showcase", ModernThemeDemo::update, ModernThemeDemo::view)
        .theme(ModernThemeDemo::theme)
        .run_with(ModernThemeDemo::new)
}

struct ModernThemeDemo {
    theme: Theme,
    theme_choice: ThemeChoice,
    text_value: String,
    checkbox_value: bool,
    radio_value: Option<RadioOption>,
    fruit_selection: Option<Fruit>,
    fruits_combo: combo_box::State<Fruit>,
    combo_selection: Option<Fruit>,
    password: String,
    show_password: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThemeChoice {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RadioOption {
    Option1,
    Option2,
    Option3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fruit {
    Apple,
    Banana,
    Orange,
    Pear,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(ThemeChoice),
    RequestRedraw,
    TextInputChanged(String),
    PasswordChanged(String),
    TogglePasswordVisibility(bool),
    CheckboxToggled(bool),
    RadioSelected(RadioOption),
    FruitSelected(Fruit),
    FruitComboSelected(Fruit),
    ButtonClicked(&'static str),
    PrimaryClicked,
    SecondaryClicked,
    SuccessClicked,
    WarningClicked,
    DangerClicked,
    LinkClicked,
    SystemClicked,
    PlainClicked,
}

impl ModernThemeDemo {
    fn new() -> (Self, Task<Message>) {
        let app = ModernThemeDemo {
            theme: Modern::light_theme(),
            theme_choice: ThemeChoice::Light,
            text_value: String::new(),
            checkbox_value: false,
            radio_value: None,
            fruit_selection: None,
            fruits_combo: combo_box::State::new(vec![
                Fruit::Apple, 
                Fruit::Banana, 
                Fruit::Orange, 
                Fruit::Pear
            ]),
            combo_selection: None,
            password: String::new(),
            show_password: false,
        };
        
        (app, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ThemeChanged(choice) => {
                self.theme_choice = choice;
                self.theme = match choice {
                    ThemeChoice::Light => Modern::light_theme(),
                    ThemeChoice::Dark => Modern::dark_theme(),
                };
                
                // Request a redraw to ensure all components update with new theme
                return Task::perform(async {}, |_| Message::RequestRedraw);
            }
            Message::RequestRedraw => {
                // This is a no-op message, just used to trigger a re-render
            }
            Message::TextInputChanged(value) => {
                self.text_value = value;
            }
            Message::PasswordChanged(value) => {
                self.password = value;
            }
            Message::TogglePasswordVisibility(show) => {
                self.show_password = show;
            }
            Message::CheckboxToggled(value) => {
                self.checkbox_value = value;
            }
            Message::RadioSelected(option) => {
                self.radio_value = Some(option);
            }
            Message::FruitSelected(fruit) => {
                self.fruit_selection = Some(fruit);
            }
            Message::ButtonClicked(name) => {
                // Just for demonstration
                println!("Button clicked: {}", name);
            }
            Message::FruitComboSelected(fruit) => {
                self.combo_selection = Some(fruit);
            }
            _ => {
                // Button clicks just for demo
            }
        }
        
        Task::none()
    }
    
    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn view(&self) -> Element<Message> {
        // Header section
        let header = container(
            text("Modern-style Theme & Colors Showcase")
                .size(30)
        ).center_x(Length::Fill)
        .width(Length::Fill)
        .padding(15);

        // Theme selector
        let theme_section = container(
            row![
                text("Theme:").size(16),
                horizontal_space().width(10),
                radio("Light", ThemeChoice::Light, Some(self.theme_choice), Message::ThemeChanged)
                    .style(Modern::radio()),
                horizontal_space().width(10),
                radio("Dark", ThemeChoice::Dark, Some(self.theme_choice), Message::ThemeChanged)
                    .style(Modern::radio()),
            ]
        )
        .padding(10)
        .width(Length::Fill);

        // Color showcase section - display all system colors
        let color_showcase = container(
            column![
                text("System Colors").size(20),
                vertical_space().height(10),
                
                // Primary colors row
                text("Primary Colors").size(16),
                vertical_space().height(5),
                
                row![
                    color_tile("Red", colors::system::RED, colors::system::RED_DARK, &self.theme),
                    color_tile("Orange", colors::system::ORANGE, colors::system::ORANGE_DARK, &self.theme),
                    color_tile("Yellow", colors::system::YELLOW, colors::system::YELLOW_DARK, &self.theme),
                    color_tile("Green", colors::system::GREEN, colors::system::GREEN_DARK, &self.theme),
                ],
                vertical_space().height(5),
                
                row![
                    color_tile("Mint", colors::system::MINT, colors::system::MINT_DARK, &self.theme),
                    color_tile("Teal", colors::system::TEAL, colors::system::TEAL_DARK, &self.theme),
                    color_tile("Cyan", colors::system::CYAN, colors::system::CYAN_DARK, &self.theme),
                    color_tile("Blue", colors::system::BLUE, colors::system::BLUE_DARK, &self.theme),
                ],
                vertical_space().height(5),
                
                row![
                    color_tile("Indigo", colors::system::INDIGO, colors::system::INDIGO_DARK, &self.theme),
                    color_tile("Purple", colors::system::PURPLE, colors::system::PURPLE_DARK, &self.theme),
                    color_tile("Pink", colors::system::PINK, colors::system::PINK_DARK, &self.theme),
                    color_tile("Brown", colors::system::BROWN, colors::system::BROWN_DARK, &self.theme),
                ],
                
                // Gray scale colors
                vertical_space().height(10),
                text("Gray Scale").size(16),
                vertical_space().height(5),
                
                row![
                    color_tile("Gray 1", colors::gray::GRAY1_LIGHT, colors::gray::GRAY1_DARK, &self.theme),
                    color_tile("Gray 2", colors::gray::GRAY2_LIGHT, colors::gray::GRAY2_DARK, &self.theme),
                    color_tile("Gray 3", colors::gray::GRAY3_LIGHT, colors::gray::GRAY3_DARK, &self.theme),
                ],
                vertical_space().height(5),
                
                row![
                    color_tile("Gray 4", colors::gray::GRAY4_LIGHT, colors::gray::GRAY4_DARK, &self.theme),
                    color_tile("Gray 5", colors::gray::GRAY5_LIGHT, colors::gray::GRAY5_DARK, &self.theme),
                    color_tile("Gray 6", colors::gray::GRAY6_LIGHT, colors::gray::GRAY6_DARK, &self.theme),
                ],
            ]
        )
        .style(Modern::card_container())
        .padding(15)
        .width(Length::Fill);

        // Button showcase
        let button_showcase = container(
            column![
                text("Button Styles").size(20),
                vertical_space().height(10),
                
                // Primary row
                row![
                    container(
                        button("Primary")
                            .style(Modern::primary_button())
                            .on_press(Message::PrimaryClicked)
                    )
                    .width(Length::Fill),
                    
                    container(
                        button("Secondary")
                            .style(Modern::secondary_button())
                            .on_press(Message::SecondaryClicked)
                    )
                    .width(Length::Fill),
                ],
                vertical_space().height(5),
                
                // Success & warning row
                row![
                    container(
                        button("Success")
                            .style(Modern::success_button())
                            .on_press(Message::SuccessClicked)
                    )
                    .width(Length::Fill),
                    
                    container(
                        button("Warning")
                            .style(Modern::warning_button())
                            .on_press(Message::WarningClicked)
                    )
                    .width(Length::Fill),
                ],
                vertical_space().height(5),
                
                // Danger & System row
                row![
                    container(
                        button("Danger")
                            .style(Modern::danger_button())
                            .on_press(Message::DangerClicked)
                    )
                    .width(Length::Fill),
                    
                    container(
                        button("System")
                            .style(Modern::system_button())
                            .on_press(Message::SystemClicked)
                    )
                    .width(Length::Fill),
                ],
                vertical_space().height(5),
                
                // Text buttons row
                row![
                    container(
                        button("Link Button")
                            .style(Modern::link_button())
                            .on_press(Message::LinkClicked)
                    )
                    .width(Length::Fill),
                    
                    container(
                        button("Plain Text")
                            .style(Modern::plain_button())
                            .on_press(Message::PlainClicked)
                    )
                    .width(Length::Fill),
                ],
                
                // Disabled buttons
                vertical_space().height(10),
                text("Disabled Buttons").size(16),
                vertical_space().height(5),
                
                row![
                    container(
                        button("Primary")
                            .style(Modern::primary_button())
                    )
                    .width(Length::Fill),
                    
                    container(
                        button("Secondary")
                            .style(Modern::secondary_button())
                    )
                    .width(Length::Fill),
                ],
                vertical_space().height(5),

                // Success & warning row
                row![
                    container(
                        button("Success")
                            .style(Modern::success_button())
                    )
                    .width(Length::Fill),
                    
                    container(
                        button("Warning")
                            .style(Modern::warning_button())
                    )
                    .width(Length::Fill),
                ],
                vertical_space().height(5),
                
                // Danger & System row
                row![
                    container(
                        button("Danger")
                            .style(Modern::danger_button())
                    )
                    .width(Length::Fill),
                    
                    container(
                        button("System")
                            .style(Modern::system_button())
                    )
                    .width(Length::Fill),
                ],
                vertical_space().height(5),
                
                // Text buttons row
                row![
                    container(
                        button("Link Button")
                            .style(Modern::link_button())
                    )
                    .width(Length::Fill),
                    
                    container(
                        button("Plain Text")
                            .style(Modern::plain_button())
                    )
                    .width(Length::Fill),
                ],
            ]
        )
        .style(Modern::card_container())
        .padding(15)
        .width(Length::Fill);
        
        // Additional button styles showcase
        let extended_button_showcase = container(
            column![
                text("Extended Button Styles").size(20),
                vertical_space().height(10),
                
                row![
                    container(
                        button("Teal Button")
                            .style(Modern::teal_button())
                            .on_press(Message::ButtonClicked("Teal"))
                    )
                    .width(Length::Fill),
                    
                    container(
                        button("Indigo Button")
                            .style(Modern::indigo_button())
                            .on_press(Message::ButtonClicked("Indigo"))
                    )
                    .width(Length::Fill),
                ],
                vertical_space().height(5),
                
                row![
                    container(
                        button("Purple Button")
                            .style(Modern::purple_button())
                            .on_press(Message::ButtonClicked("Purple"))
                    )
                    .width(Length::Fill),
                    
                    container(
                        button("Pink Button")
                            .style(Modern::pink_button())
                            .on_press(Message::ButtonClicked("Pink"))
                    )
                    .width(Length::Fill),
                ],
            ]
        )
        .style(Modern::card_container())
        .padding(15)
        .width(Length::Fill);

        // Button variants showcase section
        let button_variants_showcase = container(
            column![
                text("Button Variants").size(20),
                vertical_space().height(10),
                
                // Plain, Gray, Tinted, Filled heading
                row![
                    container(text("Plain").style(Modern::secondary_text()))
                        .width(Length::Fill)
                        .center_x(Length::Fill),
                    
                    container(text("Gray").style(Modern::secondary_text()))
                        .width(Length::Fill)
                        .center_x(Length::Fill),
                    
                    container(text("Tinted").style(Modern::secondary_text()))
                        .width(Length::Fill)
                        .center_x(Length::Fill),
                    
                    container(text("Filled").style(Modern::secondary_text()))
                        .width(Length::Fill)
                        .center_x(Length::Fill),
                ],
                vertical_space().height(5),
                
                // Small buttons row
                text("Small").size(16).style(Modern::secondary_text()),
                vertical_space().height(5),
                
                row![
                    // Plain small button
                    container(
                        button(row![text("⏵").shaping(text::Shaping::Advanced), horizontal_space().width(5), text("Play")])
                            .style(Modern::plain_button())
                            .on_press(Message::ButtonClicked("Small Plain"))
                            .padding(5)
                    )
                    .width(Length::Fill)
                    .center_x(Length::Fill),
                    
                    // Gray small button
                    container(
                        button(row![text("⏵").shaping(text::Shaping::Advanced), horizontal_space().width(5), text("Play")])
                            .style(Modern::gray_button())
                            .on_press(Message::ButtonClicked("Small Gray"))
                            .padding(5)
                    )
                    .width(Length::Fill)
                    .center_x(Length::Fill),
                    
                    // Tinted small button
                    container(
                        button(row![text("⏵").shaping(text::Shaping::Advanced), horizontal_space().width(5), text("Play")])
                            .style(Modern::blue_tinted_button())
                            .on_press(Message::ButtonClicked("Small Tinted"))
                            .padding(5)
                    )
                    .width(Length::Fill)
                    .center_x(Length::Fill),
                    
                    // Filled small button
                    container(
                        button(row![text("⏵").shaping(text::Shaping::Advanced), horizontal_space().width(5), text("Play")])
                            .style(Modern::primary_button())
                            .on_press(Message::ButtonClicked("Small Filled"))
                            .padding(5)
                    )
                    .width(Length::Fill)
                    .center_x(Length::Fill),
                ],
                vertical_space().height(15),
                
                // Medium buttons row
                text("Medium").size(16).style(Modern::secondary_text()),
                vertical_space().height(5),
                
                row![
                    // Plain medium button
                    container(
                        button(row![text("⏵").shaping(text::Shaping::Advanced), horizontal_space().width(5), text("Play")])
                            .style(Modern::plain_button())
                            .on_press(Message::ButtonClicked("Medium Plain"))
                            .padding(8)
                    )
                    .width(Length::Fill)
                    .center_x(Length::Fill),
                    
                    // Gray medium button
                    container(
                        button(row![text("⏵").shaping(text::Shaping::Advanced), horizontal_space().width(5), text("Play")])
                            .style(Modern::gray_button())
                            .on_press(Message::ButtonClicked("Medium Gray"))
                            .padding(8)
                    )
                    .width(Length::Fill)
                    .center_x(Length::Fill),
                    
                    // Tinted medium button
                    container(
                        button(row![text("⏵").shaping(text::Shaping::Advanced), horizontal_space().width(5), text("Play")])
                            .style(Modern::blue_tinted_button())
                            .on_press(Message::ButtonClicked("Medium Tinted"))
                            .padding(8)
                    )
                    .width(Length::Fill)
                    .center_x(Length::Fill),
                    
                    // Filled medium button
                    container(
                        button(row![text("⏵").shaping(text::Shaping::Advanced), horizontal_space().width(5), text("Play")])
                            .style(Modern::primary_button())
                            .on_press(Message::ButtonClicked("Medium Filled"))
                            .padding(8)
                    )
                    .width(Length::Fill)
                    .center_x(Length::Fill),
                ],
                vertical_space().height(15),
                
                // Large buttons row
                text("Large").size(16).style(Modern::secondary_text()),
                vertical_space().height(5),
                
                row![
                    // Plain large button
                    container(
                        button(row![text("⏵").shaping(text::Shaping::Advanced), horizontal_space().width(5), text("Play")])
                            .style(Modern::plain_button())
                            .on_press(Message::ButtonClicked("Large Plain"))
                            .padding(12)
                    )
                    .width(Length::Fill)
                    .center_x(Length::Fill),
                    
                    // Gray large button
                    container(
                        button(row![text("⏵").shaping(text::Shaping::Advanced), horizontal_space().width(5), text("Play")])
                            .style(Modern::gray_button())
                            .on_press(Message::ButtonClicked("Large Gray"))
                            .padding(12)
                    )
                    .width(Length::Fill)
                    .center_x(Length::Fill),
                    
                    // Tinted large button
                    container(
                        button(row![text("⏵").shaping(text::Shaping::Advanced), horizontal_space().width(5), text("Play")])
                            .style(Modern::blue_tinted_button())
                            .on_press(Message::ButtonClicked("Large Tinted"))
                            .padding(12)
                    )
                    .width(Length::Fill)
                    .center_x(Length::Fill),
                    
                    // Filled large button
                    container(
                        button(row![text("⏵").shaping(text::Shaping::Advanced), horizontal_space().width(5), text("Play")])
                            .style(Modern::primary_button())
                            .on_press(Message::ButtonClicked("Large Filled"))
                            .padding(12)
                    )
                    .width(Length::Fill)
                    .center_x(Length::Fill),
                ],
                
                // Additional tinted color variants
                vertical_space().height(15),
                text("Tinted Button Colors").size(16),
                vertical_space().height(5),
                
                row![
                    container(
                        button("Blue")
                            .style(Modern::blue_tinted_button())
                            .on_press(Message::ButtonClicked("Blue Tinted"))
                    )
                    .width(Length::Fill),
                    
                    container(
                        button("Green")
                            .style(Modern::green_tinted_button())
                            .on_press(Message::ButtonClicked("Green Tinted"))
                    )
                    .width(Length::Fill),
                    
                    container(
                        button("Red")
                            .style(Modern::red_tinted_button())
                            .on_press(Message::ButtonClicked("Red Tinted"))
                    )
                    .width(Length::Fill),
                ],
                vertical_space().height(5),
                
                row![
                    container(
                        button("Orange")
                            .style(Modern::orange_tinted_button())
                            .on_press(Message::ButtonClicked("Orange Tinted"))
                    )
                    .width(Length::Fill),
                    
                    container(
                        button("Purple")
                            .style(Modern::purple_tinted_button())
                            .on_press(Message::ButtonClicked("Purple Tinted"))
                    )
                    .width(Length::Fill),
                    
                    container(
                        button("Pink")
                            .style(Modern::pink_tinted_button())
                            .on_press(Message::ButtonClicked("Pink Tinted"))
                    )
                    .width(Length::Fill),
                ],
            ]
        )
        .style(Modern::card_container())
        .padding(15)
        .width(Length::Fill);

        // Container styles showcase
        let container_showcase = container(
            column![
                text("Container Styles").size(20),
                vertical_space().height(10),
                
                // Card container
                container(
                    text("Card Container")
                ).center_x(Length::Fill)
                .style(Modern::card_container())
                .padding(10)
                .width(Length::Fill),
                vertical_space().height(5),

                // Sheet container
                container(
                    text("Sheet Container")
                ).center_x(Length::Fill)
                .style(Modern::sheet_container())
                .padding(10)
                .width(Length::Fill),
                vertical_space().height(5),
                
                // Group container
                container(
                    text("Group Container")
                ).center_x(Length::Fill)
                .style(Modern::group_container())
                .padding(10)
                .width(Length::Fill),
                vertical_space().height(5),
                
                // Sidebar container
                container(
                    text("Sidebar Container")
                ).center_x(Length::Fill)
                .style(Modern::sidebar_container())
                .padding(10)
                .width(Length::Fill),
                vertical_space().height(5),
                
                // Accent container
                container(
                    text("Accent Container")
                ).center_x(Length::Fill)
                .style(Modern::accent_container())
                .padding(10)
                .width(Length::Fill),
                vertical_space().height(5),
                
                // Toolbar
                container(
                    text("Toolbar Container")
                ).center_x(Length::Fill)
                .style(Modern::toolbar_container())
                .padding(10)
                .width(Length::Fill),
                vertical_space().height(5),
                
                // Floating container
                container(
                    text("Floating Container")
                ).center_x(Length::Fill)
                .style(Modern::floating_container())
                .padding(10)
                .width(Length::Fill),
            ]
        )
        .style(Modern::card_container())
        .padding(15)
        .width(Length::Fill);

        // Form Controls section for the showcase app
        let form_controls_section = container(
                column![
                    text("Form Controls").size(20),
                    vertical_space().height(10),
                    
                    // Text inputs
                    text("Text Inputs").size(16),
                    vertical_space().height(5),
                    
                    // Standard text input
                    text_input("Standard text input...", &self.text_value)
                        .style(Modern::text_input())
                        .on_input(Message::TextInputChanged)
                        .padding(10),
                    vertical_space().height(5),
                        
                    // Password input with reveal checkbox
                    row![
                        text_input("Password...", &self.password)
                            .style(Modern::text_input())
                            .on_input(Message::PasswordChanged)
                            .secure(!self.show_password)
                            .padding(10),
                        horizontal_space().width(10),
                        checkbox("Show", self.show_password)
                            .style(Modern::checkbox())
                            .on_toggle(Message::TogglePasswordVisibility),
                    ],
                    vertical_space().height(5),
                    
                    // Search input
                    text_input("Search...", &self.text_value)
                        .style(Modern::search_input())
                        .on_input(Message::TextInputChanged)
                        .padding(10),
                    vertical_space().height(10),

                    // Inline input
                    text_input("Inline input", &self.text_value)
                        .style(Modern::inline_text_input())
                        .on_input(Message::TextInputChanged),
                    vertical_space().height(10),
                    
                    // Checkboxes
                    text("Checkboxes").size(16),
                    vertical_space().height(5),
                    
                    row![
                        checkbox("Unchecked", false)
                            .style(Modern::checkbox()),
                        horizontal_space().width(20),
                        checkbox("Checked", true)
                            .style(Modern::checkbox()),
                        horizontal_space().width(20),
                        checkbox("Interactive", self.checkbox_value)
                            .style(Modern::checkbox())
                            .on_toggle(Message::CheckboxToggled),
                    ],
                    vertical_space().height(10),
                    
                    // Radio buttons
                    text("Radio Buttons").size(16),
                    vertical_space().height(5),
                    
                    row![
                        radio(
                            "Option 1", 
                            RadioOption::Option1, 
                            self.radio_value, 
                            Message::RadioSelected
                        )
                        .style(Modern::radio()),
                        horizontal_space().width(20),
                        radio(
                            "Option 2", 
                            RadioOption::Option2, 
                            self.radio_value, 
                            Message::RadioSelected
                        )
                        .style(Modern::radio()),
                        horizontal_space().width(20),
                        radio(
                            "Option 3", 
                            RadioOption::Option3, 
                            self.radio_value, 
                            Message::RadioSelected
                        )
                        .style(Modern::radio()),
                    ],
                    vertical_space().height(10),
                    
                    // Pick list
                    text("Pick List").size(16),
                    vertical_space().height(5),
                    
                    pick_list(
                        &[Fruit::Apple, Fruit::Banana, Fruit::Orange, Fruit::Pear][..],
                        self.fruit_selection,
                        Message::FruitSelected
                    )
                    .style(Modern::pick_list())
                    .placeholder("Choose a fruit...")
                    .padding(10),
                    vertical_space().height(10),

                    // Combo Box
                    text("Combo Box").size(16),
                    vertical_space().height(5),

                    combo_box(
                        &self.fruits_combo,
                        "Select a fruit...",
                        self.combo_selection.as_ref(),
                        Message::FruitComboSelected
                    )
                    .input_style(Modern::combo_box())
                    .menu_style(Modern::combo_box_menu())
                    .width(250)
                    .padding(10),
                    vertical_space().height(10),
                    
                    // Text styles showcase 
                    text("Text Styles").size(16),
                    vertical_space().height(5),
                    
                    text("Primary Text Style").style(Modern::primary_text()),
                    text("Secondary Text Style").style(Modern::secondary_text()),
                    text("Tertiary Text Style").style(Modern::tertiary_text()),
                    text("Link Style Text").style(Modern::link_text()),
                    
                    vertical_space().height(5),
                    row![
                        text("Red Text").style(Modern::red_text()),
                        horizontal_space().width(10),
                        text("Blue Text").style(Modern::blue_text()),
                        horizontal_space().width(10),
                        text("Green Text").style(Modern::green_text()),
                    ],
                    vertical_space().height(5),
                    row![
                        text("Orange Text").style(Modern::orange_text()),
                        horizontal_space().width(10),
                        text("Purple Text").style(Modern::purple_text()),
                        horizontal_space().width(10),
                        text("Pink Text").style(Modern::pink_text()),
                    ],
                ]
            )
            .style(Modern::card_container())
            .padding(15)
            .width(Length::Fill);

        // Content column (NOT using Length::Fill for height)
        let content_column = column![
            header,
            theme_section,
            color_showcase,
            vertical_space().height(10),
            button_showcase,
            vertical_space().height(10),
            extended_button_showcase,
            vertical_space().height(10),
            button_variants_showcase,
            vertical_space().height(10),
            container_showcase,
            vertical_space().height(10),
            form_controls_section,
        ]
        .spacing(0)
        .padding(10)
        .width(Length::Fill); // Width can be Fill

        // Create scrollable content with all sections
        let content = scrollable(content_column);

        // Main container
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}


// Helper function to create a color tile
fn color_tile<'a>(name: &'a str, light_color: Color, dark_color: Color, current_theme: &'a Theme) -> Element<'a, Message> {
    // Determine which color to use based on the passed theme
    let current_color = match current_theme {
        Theme::Light => light_color,
        Theme::Dark => dark_color,
        Theme::Custom(custom_name) => {
            // Check if the custom theme name contains "Dark"
            if custom_name.to_string().contains("Dark") {
                dark_color
            } else {
                light_color
            }
        }
        _ => {light_color}
    };
    
    container(
        column![
            container(text("")).height(30).width(60),
            text(name).size(12),
        ]
    ).center_x(Length::Fill)
    .style(move |_theme| {
        container::Style {
            text_color: Some(Color::WHITE),
            background: Some(Background::Color(current_color)),
            border: Border {
                radius: 4.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            shadow: Shadow::default(),
        }
    })
    .padding(5)
    .center_x(Length::Fill)
    .into()
}

impl std::fmt::Display for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fruit::Apple => write!(f, "Apple"),
            Fruit::Banana => write!(f, "Banana"),
            Fruit::Orange => write!(f, "Orange"),
            Fruit::Pear => write!(f, "Pear"),
        }
    }
}