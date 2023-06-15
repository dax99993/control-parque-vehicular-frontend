use chrono::{NaiveDate, Utc, Datelike, Weekday, Days};
use chrono_utilities::naive::DateTransitions;
use yew::{prelude::*, virtual_dom::VNode};

//use crate::calendar::language::CalendarLang;
//use crate::calendar::color::CalendarColor;
use super::language::CalendarLang;
use super::color::CalendarColor;

use std::ops::Deref;


pub struct Calendar {
    //start_date: NaiveDate,
    //end_date: NaiveDate,
    pub current_date: NaiveDate,
    pub selected_date: Option<NaiveDate>,
    
    active_datepicker: bool,

    active_yearpicker: bool,
    active_monthpicker: bool,
    active_daypicker: bool,
}




#[derive(PartialEq, Properties)]
pub struct CalendarProps {
    //#[prop_or_default]
    pub date: UseStateHandle<Option<NaiveDate>>,
    #[prop_or_default]
    pub color: CalendarColor,
    #[prop_or_default]
    pub lang: CalendarLang,
    //#[prop_or_default]
    //isrange: bool
    //#[prop_or_default]
    //allow_same_day_range: bool
    //#[prop_or_default]
    //start_date: Option<NaiveDate>
    //#[prop_or_default]
    //end_date: Option<NaiveDate>
    #[prop_or_default]
    pub min_date: Option<NaiveDate>,
    #[prop_or_default]
    pub max_date: Option<NaiveDate>,
    //#[prop_or_default]
    //disableDates: Vec<NaiveDate>,
    #[prop_or_default]
    pub disable_weekdays: Vec<Weekday>,
    //#[prop_or_default]
    //highlightedays: Vec<NaiveDate>,
    //weekStart: u32,
    //date_format: String or enum,
    #[prop_or_default]
    pub enable_footer: bool,
    //#[prop_or(true)]
    //enable_month_switch: bool,
    //#[prop_or(true)]
    //enable_year_switch: bool,
    #[prop_or(10)]
    pub display_years_count: usize,
    #[prop_or(String::from("Today"))]
    pub today_label: String,
    #[prop_or(String::from("Cancel"))]
    pub cancel_label: String,
    #[prop_or(String::from("Clear"))]
    pub clear_label: String,
}

pub enum CalendarMsg {
    ShowYears,
    ShowMonths,
    ShowDays,
    SelectDateAsToday,
    ClearDatePicker,
    CloseDatePicker,
    ToggleDatePicker,
    NextMonth,
    PrevMonth,
    SelectMonth(u32),
    SelectYear(i32),
    SelectDate(NaiveDate),
}

impl Component for Calendar {
    type Message = CalendarMsg;
    type Properties = CalendarProps;

    fn create(ctx: &Context<Self>) -> Self {
        let selected_date = if let Some(date) = ctx.props().date.deref() {
            Some(date.clone())
        } else {
            None
        };
        Self { 
            //start_date: ,
            //end_date: (),
            selected_date,
            current_date: Self::today().start_of_month().unwrap(),
            active_datepicker: false,
            active_yearpicker: false,
            active_monthpicker: false,
            active_daypicker: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CalendarMsg::ShowYears => { self.show_yearpicker() },
            CalendarMsg::ShowMonths => { self.show_monthpicker() },
            CalendarMsg::ShowDays => { self.show_daypicker() },
            CalendarMsg::ToggleDatePicker => { self.toggle_datepicker() },
            CalendarMsg::ClearDatePicker => { self.clear_datepicker() },
            CalendarMsg::CloseDatePicker => { self.close_datepicker() },
            CalendarMsg::PrevMonth => { self.prev_month() },
            CalendarMsg::NextMonth => { self.next_month() },
            CalendarMsg::SelectMonth(month) => { self.update_month(month) },
            CalendarMsg::SelectYear(year) => { self.update_year(year) },
            CalendarMsg::SelectDate(date) => { self.update_date(ctx, date) },
            CalendarMsg::SelectDateAsToday => { self.date_as_today() },

        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        // Callbacks
        let cb_toggle_datepicker = ctx.link().callback(|_| CalendarMsg::ToggleDatePicker);
        let cb_clear_datepicker = ctx.link().callback(|_| CalendarMsg::ClearDatePicker);


        //Classes
        let classes_datetimepicker = classes!["datetimepicker", ctx.props().color.as_classname(), "is-datetimepicker-default", self.active_datepicker.then(|| "is-active")];
        //let mut classes_datetimepicker = classes!["datetimepicker", "is-datetimepicker-default", self.active_datepicker.then(|| "is-active")];
        //classes_datetimepicker.extend(color_class);
        let classes_datepicker = classes!["datepicker", self.active_datepicker.then(|| "is-active")];
        let classes_daypicker  = classes!["datepicker-dates", self.active_daypicker.then(|| "is-active")];

        //Variables
        let selected_date_string = if let Some(date) = self.selected_date {
            date.to_string()
        } else {
            String::new()
        };

        log::debug!("current date = {}", self.current_date);
        log::debug!("selected date = {:?}", self.selected_date);

        html!{
            <>
            // Input 
            <div class="datetimepicker-dummy is-primary">
                <div class="datetimepicker-dummy-wrapper" onclick={cb_toggle_datepicker.clone()}>
                    <input class="datetimepicker-dummy-input" placeholder={""} type="text" readonly={true} value={selected_date_string}/>
                    <input class="input bulmaCalendar is-hidden" type="text" onclick={cb_toggle_datepicker.clone()}/>
                </div>
                <button class="datetimepicker-clear-button" type="button" onclick={cb_clear_datepicker}>{"+"}</button>
            </div>

            // Selector
            <div class="datetimepicker-wrapper">
                // Should be active
                <div class={classes_datetimepicker} style="position:absolute;">
                    // Container for header
                    <div class="datetimepicker-container">
                        //Header
                        {self.header(&ctx)}
                        //Main component Datepicker
                        <div class={classes_datepicker}>
                            //Navigation
                            {self.navigation(&ctx)}
                            // Body
                            <div class="datepicker-body">
                                //Dates (Active) exclusively active with month and years
                                <div class={classes_daypicker}>
                                    //Weekdays
                                    <div class="datepicker-weekdays">
                                        {self.weekdays(&ctx)}
                                    </div>
                                    //Days
                                    <div class="datepicker-days">
                                    {self.daypicker(&ctx)}
                                    </div>
                                </div>
                                // Months (Active by button)
                                {self.monthpicker(&ctx)}
                                // Years (Active by button)
                                {self.yearpicker(&ctx)}
                            </div>
                        </div>
                    </div>
                    //Footer
                    if ctx.props().enable_footer {
                        {self.footer(&ctx)}
                    }
                </div>
            </div>
            </>
        }
    }
}

impl Calendar {
    fn today() -> NaiveDate {
        Utc::now().date_naive()
        //NaiveDate::
    }

    fn show_yearpicker(&mut self) {
        self.active_yearpicker = true;
        self.active_monthpicker = false;
        self.active_daypicker = false;
    }

    fn show_monthpicker(&mut self) {
        self.active_yearpicker = false;
        self.active_monthpicker = true;
        self.active_daypicker = false;
    }

    fn show_daypicker(&mut self) {
        self.active_yearpicker = false;
        self.active_monthpicker = false;
        self.active_daypicker = true;
    }

    fn toggle_datepicker(&mut self) {
        self.active_datepicker = !self.active_datepicker;
    }

    fn clear_datepicker(&mut self) {
        self.active_datepicker = false;
        self.selected_date = None;
        self.current_date = Self::today();
    }

    fn close_datepicker(&mut self) {
        self.active_datepicker = false;
    }

    fn is_valid_date(date: NaiveDate, min: Option<NaiveDate>, max: Option<NaiveDate>) -> bool {
        match (min, max) {
            (Some(min), Some(max)) => { 
                min <= date && date <= max
            },
            (Some(min), None) => { min <= date },
            (None, Some(max)) => { date <= max},
            (None, None) => { true }
        }
    }

    fn next_month(&mut self)  {
        if let Some(date) = self.current_date.start_of_succ_month() {
            self.current_date = date;
        }
    }

    fn prev_month(&mut self)  {
        if let Some(date) = self.current_date.start_of_pred_month() {
            self.current_date = date;
        }
    }

    fn update_year(&mut self, year: i32)  {
        if let Some(date) = self.current_date.with_year(year) {
            self.current_date = date;
        }
        self.show_daypicker();
    }

    fn update_month(&mut self, month: u32)  {
        if let Some(date) = self.current_date.with_month0(month) {
            self.current_date = date;
        }
        self.show_daypicker();
    }

    fn update_date(&mut self, ctx: &Context<Self>, date: NaiveDate)  {
        if let Some(d) = date.start_of_month() {
            self.current_date = d;
        }
        self.selected_date = Some(date);
        ctx.props().date.set(self.selected_date);
        self.toggle_datepicker();
    }

    fn date_as_today(&mut self)  {
        self.selected_date = Some(Self::today());
        self.toggle_datepicker();
    }

    fn navigation(&self, ctx: &Context<Self>) -> Html {
        // Callbacks
        let cb_show_yearpicker = ctx.link().callback(|_| CalendarMsg::ShowYears);
        let cb_show_monthpicker = ctx.link().callback(|_| CalendarMsg::ShowMonths);
        let cb_next_month = ctx.link().callback(|_| CalendarMsg::NextMonth);
        let cb_prev_month = ctx.link().callback(|_| CalendarMsg::PrevMonth);

        // Variables
        let year = self.current_date.year();
        let months = ctx.props().lang.months_short();
        let month_index = self.current_date.month0(); //Should be 0 indexed!
        let month = months.get(month_index as usize);
        log::debug!("months = {:?}", months);
        log::debug!("month index = {}", month_index);
        log::debug!("month as str = {:?}", month);

        //HTML
        html!{
        <div class="datepicker-nav">
        // Button prev
        <button class="datepicker-nav-previous button is-small is-text" type="button" onclick={cb_prev_month}>
            //should be an svg
            <svg viewBox="0 0 50 80" xml={true.to_string()} space="preserve">
                <polyline fill="none" stroke-width=".5em" stroke-linecap="round"
                stroke-linejoin="round" points="45.63,75.8 0.375,38.087 45.63,0.375 ">
                </polyline>
            </svg>
        </button>
        // Navigation Date
        <div class="datepicker-nav-month-year">
            <div class="datepicker-nav-month" onclick={cb_show_monthpicker}>{month}</div>
            <div>{ "\u{00a0}" }</div> //Whitespace
            <div class="datepicker-nav-year" onclick={cb_show_yearpicker}>{year}</div>
        </div>
        // Button next
        <button class="datepicker-nav-next button is-small is-text" type="button" onclick={cb_next_month}>
            //should be an svg
            <svg viewBox="0 0 50 80" xml={true.to_string()} space="preserve">
                <polyline fill="none" stroke-width=".5em" stroke-linecap="round"
                stroke-linejoin="round" points="0.375,0.375 45.63,38.087 0.375,75.8 ">
                </polyline>
            </svg>
        </button>
        </div>
        }
    }


    fn weekdays(&self, ctx: &Context<Self>) -> Html {
        let weekdays = ctx.props().lang.weekdays_short();

        weekdays.iter()
            .map(|w| html!{
                <div class="datepicker-date">{w}</div>
             })
            .collect()
    }

    fn monthpicker(&self, ctx: &Context<Self>) -> Html {
        //Callbacks
        let cb_select_month = |month| {ctx.link().callback(move |_| CalendarMsg::SelectMonth(month))};

        //Classes
        let classes_monthpicker = classes!["datepicker-months", if self.active_monthpicker {"is-active"} else {""}];

        // Variables
        let months = ctx.props().lang.months_short();

        let months = months.iter().enumerate().map(|(index, month)| {
            let is_active = self.current_date.month0() == index as u32;
            let classes = classes!["datepicker-month",  is_active.then(|| "is-active")];
            html!{
                <div class={classes}  onclick={cb_select_month(index as u32)} >
                {month}
                </div>
            }
        }).collect::<Html>();

        //Html
        html!{
            <div class={classes_monthpicker}>
            {months}
            </div>
        }
    }

    fn yearpicker(&self, ctx: &Context<Self>) -> Html {
        //Callback
        let cb_select_year = |year| { ctx.link().callback(move |_| CalendarMsg::SelectYear(year as i32)) };

        //Classes
        let classes_yearpicker = classes!["datepicker-years", self.active_yearpicker.then(|| "is-active")];

        //Props
        let display_years_count = ctx.props().display_years_count;

        //Should check min date and maxdate or give a default range
        let min_date = if let Some(date) = ctx.props().min_date {
            date
        } else {
            Self::today().start_of_year().unwrap().with_year(self.current_date.year() - display_years_count as i32).unwrap()
        };
        let max_date = if let Some(date) = ctx.props().max_date {
            date
        } else{
            Self::today().start_of_year().unwrap().with_year(self.current_date.year() + display_years_count as i32).unwrap()
        };

        let years = (min_date.year()..=max_date.year()).map(|year| {
            let is_active = self.current_date.year() == year as i32;
            let classes = classes!["datepicker-year",  is_active.then(|| "is-active")];
            html!{
                <div class={classes} onclick={cb_select_year(year)}>
                    <span class="item">{year.to_string()}</span>
                </div>
            }
        }).collect::<Html>();

        html!{
            <div class={classes_yearpicker}>
            {years}
            </div>
        }
    }


    fn daypicker(&self, ctx: &Context<Self>) -> Html {
        // get current month last and first day 
        let current_month_start = self.current_date.start_of_month().unwrap();
        let current_month_lastday = self.current_date.last_day_of_month();

        let current_month = self.generate_days(ctx, current_month_start, current_month_lastday as usize, true);

        // Calculate the days from previues month and next
        // to complete the 5x7 grid of dates
        let prev_month_fit_days = current_month_start.weekday().num_days_from_sunday();
        let prev_month_start = current_month_start.checked_sub_days(Days::new(prev_month_fit_days as u64)).unwrap();

        let prev_month = self.generate_days(ctx, prev_month_start, prev_month_fit_days as usize, false);

        let next_month_start = current_month_start.start_of_succ_month().unwrap();
        let mut next_month_fit_days: i32 = 35 - current_month_lastday as i32 - prev_month_fit_days as i32;
        if next_month_fit_days < 0 { next_month_fit_days += 7 }

        let next_month = self.generate_days(ctx, next_month_start, next_month_fit_days as usize, false);

        // Add appropiate classes based on month, diableddates and current_date
        // attach callback to date to update current_date
        let mut days = vec![];
        days.extend(prev_month);
        days.extend(current_month);
        days.extend(next_month);

        return days.into_iter().collect();
    }

    fn generate_days(&self, ctx: &Context<Self>, start_date: NaiveDate, number_of_days: usize, is_current_month: bool) -> Vec<VNode> {
        //Callback 
        let cb_select_date = |date| { ctx.link().callback(move |_| CalendarMsg::SelectDate(date)) };

        //Props
        let disable_weekdays = ctx.props().disable_weekdays.clone();
        let min_date = ctx.props().min_date.clone();
        let max_date = ctx.props().max_date.clone();

        let month = start_date.iter_days().take(number_of_days)
            .map(|date| {
                let is_active = 
                    if let Some(selected_date) = self.selected_date {
                        date == selected_date
                    } else {
                        false
                    };
                let is_disabled = disable_weekdays.iter().any(|dw| dw == &date.weekday()) ||
                    !Self::is_valid_date(date, min_date, max_date);
                     
                let classes = classes!["datepicker-date", is_current_month.then(|| "is-current-month"), is_disabled.then(|| "is-disabled")];
                let classes_item = classes!["date-item", is_active.then(|| "is-active")];

                html!{ 
                    if is_disabled {
                    <div class={classes} >
                        <button class={classes_item} type="button">{date.day()}</button>
                    </div>
                    } else {
                    <div class={classes} onclick={cb_select_date(date)}>
                        <button class={classes_item} type="button">{date.day()}</button>
                    </div>
                    }
                }
            }).collect::<Vec<_>>();

        month
    }


    fn header(&self, ctx: &Context<Self>) -> Html {
        let date_selection = 
        if let Some(date) = self.selected_date {
            let day = date.day();
            let weekdays = ctx.props().lang.weekdays();
            let weekday = weekdays.get(date.weekday().num_days_from_sunday() as usize).unwrap();
            let months = ctx.props().lang.months();
            let month = months.get(date.month0() as usize).unwrap();
            let year = date.year();
            let month_year = format!("{} {}", month, year);

            html!{
                <>
                    // Actual day number
                    <div class="datetimepicker-selection-day">{day}</div>
                    <div class="datetimepicker-selection-date">
                        // Actual month year and weekday
                        <div class="datetimepicker-selection-month">{month_year}</div>
                        <div class="datetimepicker-selection-weekday">{weekday}</div>
                    </div>
                </>
            }
        } else {
            let day = "--";
            let month_year = "";
            let weekday = "";

            html!{
                <>
                    <div class="datetimepicker-selection-day">{day}</div>
                    <div class="datetimepicker-selection-date">
                        // Actual month year and weekday
                        <div class="datetimepicker-selection-month">{month_year}</div>
                        <div class="datetimepicker-selection-weekday">{weekday}</div>
                    </div>
                </>
            }
        };
                

        html!{
            <div class="datetimepicker-header is-date-only">
                <div class="datetimepicker-selection-details">
                    <div class="datetimepicker-selection-details">
                        <div class="datetimepicker-selection-start is-centered">
                            <div class="datetimepicker-selection-wrapper">
                            {date_selection}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }


    fn footer(&self, ctx: &Context<Self>) -> Html {
        // Callbacks
        let cb_select_date_as_today = ctx.link().callback(|_| CalendarMsg::SelectDateAsToday);
        let cb_clear_datepicker = ctx.link().callback(|_| CalendarMsg::ClearDatePicker);
        let cb_close_datepicker = ctx.link().callback(|_| CalendarMsg::CloseDatePicker);

        //Props
        let props = ctx.props();

        html!{
            <div class="datetimepicker-footer">
                <div class="datetimepicker-footer-today has-text-warning button is-small is-text" type="button" onclick={cb_select_date_as_today}>
                    {&props.today_label}
                </div>
                <div class="datetimepicker-footer-clear has-text-danger button is-small is-text" type="button" onclick={cb_clear_datepicker}>
                    {&props.clear_label}
                </div>
                <div class="datetimepicker-footer-cancel button is-small is-text" type="button" onclick={cb_close_datepicker}>
                    {&props.cancel_label}
                </div>
            </div>
        }
    }
}

