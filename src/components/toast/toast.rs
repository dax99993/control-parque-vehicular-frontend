use yew::Html;
use chrono::Duration;

// Available toast positions
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ToastPosition {
    #[default]
    TopRight,
    TopLeft,
    TopCenter,
    //Center,
    BottomRight,
    BottomLeft,
    BottomCenter,
}

impl ToastPosition {
    pub fn as_attribute(&self) -> &'static str {
        match self {
            Self::TopRight => "top-right",
            Self::TopLeft => "top-left",
            Self::TopCenter => "top-center",
            //Self::center=> vec!["center"],
            Self::BottomRight => "bottom-right",
            Self::BottomLeft => "bottom-left",
            Self::BottomCenter => "bottom-center",
        }
    }
}

// Types of toast according to importance
#[derive(Clone, PartialEq, Default)]
#[derive(Debug)]
pub enum ToastType {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Danger,
}


impl ToastType {
    pub fn as_classes(&self) -> Vec<&'static str> {
        match self {
            Self::Default => vec![],
            Self::Info => vec!["is-info"],
            Self::Success => vec!["is-success"],
            Self::Warning => vec!["is-warning"],
            Self::Danger => vec!["is-danger"],
        }
    }
}


// General Toast
//
// Used internally with hook to add Toast
#[derive(Clone, Default)]
#[derive(Debug)]
pub struct Toast {
    pub position: ToastPosition,
    pub body: Html,
    /// The timeout when the toast will be removed automatically.
    ///
    /// If no timeout is set, the toast will get a close button.
    pub timeout: Option<Duration>,
    pub r#type: ToastType,
}

impl Toast {
    pub fn new(body: Html, position: ToastPosition, r#type: ToastType, timeout: Option<Duration>) -> Self {
        Self { 
            body,
            position,
            timeout,
            r#type,
        }
    }


}

#[derive(Default)]
pub struct ToastBuilder {
    position: ToastPosition,
    body: Html,
    timeout: Option<Duration>,
    r#type: ToastType,
}

impl ToastBuilder {
    pub fn new() -> Self {
        //Self { position: ToastPosition::TopLeft, body: html!{}, timeout: (), r#type: () }
        Self::default()
    }

    pub fn with_position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    pub fn with_type(mut self, r#type: ToastType) -> Self {
        self.r#type = r#type;
        self
    }

    pub fn with_body(mut self, body: Html) -> Self {
        self.body = body;
        self
    }

    pub fn with_timeout(mut self, timeout: Option<i64>) -> Self {
        let timeout = timeout.map(|t| Duration::milliseconds(t));
        self.timeout = timeout;
        self
    }

    pub fn is_default(mut self) -> Self {
        self.r#type = ToastType::Default;
        self
    }
    pub fn is_info(mut self) -> Self {
        self.r#type = ToastType::Info;
        self
    }
    pub fn is_success(mut self) -> Self {
        self.r#type = ToastType::Success;
        self
    }
    pub fn is_warning(mut self) -> Self {
        self.r#type = ToastType::Warning;
        self
    }
    pub fn is_danger(mut self) -> Self {
        self.r#type = ToastType::Danger;
        self
    }

    pub fn at_top_right(mut self) -> Self {
        self.position = ToastPosition::TopRight;
        self
    }
    pub fn at_top_center(mut self) -> Self {
        self.position = ToastPosition::TopCenter;
        self
    }
    pub fn at_top_left(mut self) -> Self {
        self.position = ToastPosition::TopLeft;
        self
    }
    pub fn at_bottom_right(mut self) -> Self {
        self.position = ToastPosition::BottomRight;
        self
    }
    pub fn at_bottom_center(mut self) -> Self {
        self.position = ToastPosition::BottomCenter;
        self
    }
    pub fn at_bottom_left(mut self) -> Self {
        self.position = ToastPosition::BottomLeft;
        self
    }

    pub fn build(self) -> Toast {
        Toast {
            position: self.position,
            r#type: self.r#type,
            body: self.body,
            timeout: self.timeout,
        }
    }
}
