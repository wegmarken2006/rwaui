use serde_derive::{Deserialize, Serialize};

// Define the structure for yaml configuration file
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GridRowElement {
    pub dropdown: Option<Dropdown>,
    pub button: Option<Button>,
    pub form: Option<Form>,
    pub input: Option<Input>,
    pub slider: Option<Slider>,
    pub textarea: Option<Textarea>,
    pub label: Option<Label>,
    pub h2: Option<H2>,
    pub p: Option<Paragraph>,
    pub canvas: Option<Canvas>,
    pub image: Option<Image>,
    pub date: Option<Date>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Date {
    pub id: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Paragraph {
    pub id: Option<String>,
    pub text: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Dropdown {
    pub id: String,
    pub defaultind: i32,
    pub items: vt!(String),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Button {
    pub id: String,
    pub text: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Form {
    pub id: String,
    pub text: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Input {
    pub id: String,
    pub text: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Slider {
    pub id: String,
    pub minmaxini: vt!(i32),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Textarea {
    pub id: String,
    pub text: String,
    pub lines: i32,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Label {
    pub id: String,
    pub text: String,
    pub mutable: bool,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct H2 {
    pub id: String,
    pub text: String,
    pub mutable: bool,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Canvas {
    pub id: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Image {
    pub id: String,
}

// Define the structure for each grid row
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GuiDescr {
    pub tab: Tab,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Tab {
    pub id: Option<String>,
    pub text: Option<String>,
    pub rows: vt!(Row),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Row {
    pub gridrow: vt!(GridRowElement),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RxTxMessage {
    pub text: String,
    pub textarea: String,
    pub backgroundcolor: String,
    pub color: String,
    pub imagename: String,
    pub list: vt!(String),
}

impl Default for RxTxMessage {
    fn default() -> RxTxMessage {
        RxTxMessage {
            text: "".to_string(),
            textarea: "".to_string(),
            backgroundcolor: "".to_string(),
            color: "".to_string(),
            imagename: "".to_string(),
            list: vec![]
        }
    }
}
