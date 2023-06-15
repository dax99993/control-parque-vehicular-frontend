use chrono::{DateTime, Utc};
use yew::{prelude::*, virtual_dom::VChild};
use std::collections::BinaryHeap;
use core::cmp::Reverse;
use gloo::timers::callback::Timeout;

use super::container::{ToastContainer, ToastComponent};
use super::toast::{Toast, ToastPosition};


#[doc(hidden)]
#[derive(Debug)]
pub enum ToastAction {
    ShowToast(Toast)
}

/// An agent for displaying toasts.
#[derive(Clone, PartialEq)]
#[derive(Debug)]
pub struct Toaster {
    callback: Callback<ToastAction>,
}

impl Toaster {
    /// Request a toast from the toast viewer.
    pub fn toast(&self, toast: Toast) {
        self.callback.emit(ToastAction::ShowToast(toast))
    }
}


// ----------------- Components -----------------
#[derive(Clone, PartialEq, Properties)]
pub struct ToasterViewerProps {
    pub children: Children,
}

pub struct ToastEntry {
    id: usize,
    position: ToastPosition,
    toast: VChild<ToastComponent>,
    timeout: Option<DateTime<Utc>>,
}

/// A component to view toast alerts.
///
/// Exactly one instance is required in your page in order to actually show the toasts.
pub struct ToasterViewer {
    context: Toaster,
    // array of 6 ToasterContainer for each position
    toasts: Vec<ToastEntry>,
    counter: usize,

    task: Option<Timeout>,
    timeouts: BinaryHeap<Reverse<DateTime<Utc>>>,
}

pub enum ToasterViewerMsg {
    //CreateContainer,
    //DeleteContainer,
    //AddToast,
    //RemoveToast,

    //
    Perform(ToastAction),
    Cleanup,
    Close(usize),
}


impl Component for ToasterViewer {
    type Message = ToasterViewerMsg;
    type Properties = ToasterViewerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let context = Toaster {
            callback: ctx.link().callback(ToasterViewerMsg::Perform),
        };
        Self {
            context,
            toasts: vec![],
            counter: 0,
            task: None,
            timeouts: BinaryHeap::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Perform(action) => self.perform(ctx, action),
            Self::Message::Cleanup => self.cleanup(ctx),
            Self::Message::Close(id) => self.remove_toast(id),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let context = self.context.clone();

        let top_left_toasts = self.get_toasts_with_position(ToastPosition::TopLeft);
        let top_center_toasts = self.get_toasts_with_position(ToastPosition::TopCenter);
        let top_right_toasts = self.get_toasts_with_position(ToastPosition::TopRight);
        //let center = self.get_toasts_with_position(ToastPosition::Center);
        let bottom_left_toasts = self.get_toasts_with_position(ToastPosition::BottomLeft);
        let bottom_center_toasts= self.get_toasts_with_position(ToastPosition::BottomCenter);
        let bottom_right_toasts = self.get_toasts_with_position(ToastPosition::BottomRight);

        //{ for self.toasts.iter().filter(|entry| entry.position == ToastPosition::BottomRight).map(|entry| entry.toast.clone()) } 
        html!{
            <ContextProvider<Toaster> {context}>
                {top_left_toasts}
                {top_center_toasts}
                {top_right_toasts}
                {bottom_left_toasts}
                {bottom_center_toasts}
                {bottom_right_toasts}
                { for ctx.props().children.iter() }
            </ContextProvider<Toaster>>
        }
    }
}


impl ToasterViewer {
    fn now() -> DateTime<Utc> {
        Utc::now()
    }

    fn get_toasts_with_position(&self, position: ToastPosition) -> Html {
        let mut toasts = self.toasts.iter()
            .filter(|entry| entry.position == position)
            .map(|entry| entry.toast.clone()).peekable();

        if toasts.peek().is_some() {
            html!{
                <ToastContainer position={position.clone()}>
                    { toasts.collect::<Html>() }
                </ToastContainer>
            }
        } else {
            html!{}
        }

    }

    fn perform(&mut self, ctx: &Context<Self>, action: ToastAction) -> bool {
        match action {
            ToastAction::ShowToast(toast) => self.add_toast(ctx, toast),
        }
        true
    }

    fn add_toast(&mut self, ctx: &Context<Self>, toast: Toast) {
        let now = Self::now();
        let timeout = toast
            .timeout
            .map(|timeout| now + timeout);
        // get an id for toast
        let id = self.counter;
        self.counter += 1;
        
        // Only can close toast if has no timeout, otherwise closes itself when run out of time
        let onclose = match toast.timeout {
            None => Some(ctx.link().callback(move |_: _| ToasterViewerMsg::Close(id))),
            Some(_) => None,
        };

        // Add toast to internal store used in view fn
        self.toasts.push(ToastEntry {
            id,
            position: toast.position,
            toast: html_nested!{
                <ToastComponent r#type={toast.r#type} onclose={onclose}>
                    { toast.body }
                </ToastComponent>
            },
            timeout,
        });
        
        if let Some(timeout) = timeout {
            self.schedule_cleanup(ctx, timeout);
        }
    }

    fn schedule_cleanup(&mut self, ctx: &Context<Self>, timeout: DateTime<Utc>) {
        log::debug!("Schedule cleanup: {:?}", timeout);

        //store timeouts in reverse (the ones that will occur first)
        self.timeouts.push(Reverse(timeout));
        self.trigger_next_cleanup(ctx);
    }

    fn trigger_next_cleanup(&mut self, ctx: &Context<Self>) {
        if self.task.is_some() {
            log::debug!("Already have a task");
            return;
        }

        // We poll timeouts from the heap until we find one that is in the future, or we run
        // out of candidates.
        while let Some(next) = self.timeouts.pop() {
            let timeout = next.0;
            log::debug!("Next timeout: {:?}", timeout);
            //time left to delete toast
            let duration = timeout - Self::now();
            // verify duration is positive (toast needs to be deleted in the future)
            let duration = duration.to_std();
            if let Ok(duration) = duration {
                let link = ctx.link().clone();
                // Task will be executed when duration is 0
                self.task = Some(Timeout::new(duration.as_millis() as u32, move || {
                    link.send_message(ToasterViewerMsg::Cleanup);
                }));
                log::debug!("Schedule cleanup {:?}", duration);
                break;
            }
        }
    }



    fn cleanup(&mut self, ctx: &Context<Self>) -> bool {
        let now = Self::now();
        // Task executed, trigger next task if any
        self.task = None;
        self.trigger_next_cleanup(ctx);
        //keep all toasts whos timeout are in the future or dont have timeout 
        self.retain_toast(|toast| {
            if let Some(timeout) = toast.timeout {
                timeout > now
            } else {
                true
            }
        })
    }

    fn retain_toast<F>(&mut self, f: F) -> bool
        where
            F: Fn(&ToastEntry) -> bool
    {
        let before = self.toasts.len();
        self.toasts.retain(f);
        before != self.toasts.len()
    }

    fn remove_toast(&mut self, id: usize) -> bool {
        self.retain_toast(|entry| entry.id != id)
    }

}

/// Get a [`Toaster`] context.
#[hook]
pub fn use_toaster() -> Option<Toaster> {
    use_context()
}
