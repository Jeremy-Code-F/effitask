use crate::widgets::tasks::Msg::{Complete, Edit};
use crate::widgets::Tasks;
use gtk::prelude::*;

#[derive(relm_derive::Msg)]
pub enum Msg {
    Complete(Box<crate::tasks::Task>),
    Edit(Box<crate::tasks::Task>),
    Selected,
    Select(chrono::DateTime<chrono::Local>),
    Update,
}

macro_rules! update {
    ($self:ident, $exp:ident, $task:ident, $get:ident, $list:ident, $date:ident) => {
        let tasks = $self.$get(&$list, $date);

        $self.widgets.$exp.set_expanded(!tasks.is_empty());
        $self.widgets.$exp.set_sensitive(!tasks.is_empty());
        $self
            .components
            .$task
            .emit(crate::widgets::tasks::Msg::Update(tasks));
    };
}

impl Widget {
    fn update_tasks(&self) {
        self.widgets.calendar.clear_marks();

        let list = crate::application::tasks();
        let (y, m, d) = self.widgets.calendar.date();
        let date = chrono::naive::NaiveDate::from_ymd_opt(y as i32, m + 1, d);

        update!(self, past_exp, past, get_past_tasks, list, date);
        update!(self, today_exp, today, get_today_tasks, list, date);
        update!(self, tomorrow_exp, tomorrow, get_tomorrow_tasks, list, date);
        update!(self, week_exp, week, get_week_tasks, list, date);
        update!(self, month_exp, month, get_month_tasks, list, date);
    }

    fn get_past_tasks(
        &self,
        list: &crate::tasks::List,
        date: Option<chrono::naive::NaiveDate>,
    ) -> Vec<crate::tasks::Task> {
        self.get_tasks(list, None, date)
    }

    fn get_today_tasks(
        &self,
        list: &crate::tasks::List,
        date: Option<chrono::naive::NaiveDate>,
    ) -> Vec<crate::tasks::Task> {
        self.get_tasks(list, date, date.and_then(|x| x.succ_opt()))
    }

    fn get_tomorrow_tasks(
        &self,
        list: &crate::tasks::List,
        date: Option<chrono::naive::NaiveDate>,
    ) -> Vec<crate::tasks::Task> {
        self.get_tasks(
            list,
            date.and_then(|x| x.succ_opt()),
            date.map(|x| x + chrono::Duration::days(2)),
        )
    }

    fn get_week_tasks(
        &self,
        list: &crate::tasks::List,
        date: Option<chrono::naive::NaiveDate>,
    ) -> Vec<crate::tasks::Task> {
        self.get_tasks(
            list,
            date.map(|x| x + chrono::Duration::days(2)),
            date.map(|x| x + chrono::Duration::weeks(1)),
        )
    }

    fn get_month_tasks(
        &self,
        list: &crate::tasks::List,
        date: Option<chrono::naive::NaiveDate>,
    ) -> Vec<crate::tasks::Task> {
        self.get_tasks(
            list,
            date.map(|x| x + chrono::Duration::weeks(1)),
            date.map(|x| x + chrono::Duration::weeks(4)),
        )
    }

    fn get_tasks(
        &self,
        list: &crate::tasks::List,
        start: Option<chrono::naive::NaiveDate>,
        end: Option<chrono::naive::NaiveDate>,
    ) -> Vec<crate::tasks::Task> {
        let (_, month, _) = self.widgets.calendar.date();
        let preferences = crate::application::preferences();

        let tasks: Vec<crate::tasks::Task> = list
            .tasks
            .iter()
            .filter(|x| {
                if let Some(due_date) = x.due_date {
                    (preferences.done || !x.finished)
                        && (preferences.defered
                            || x.threshold_date.is_none()
                            || start.is_none()
                            || x.threshold_date.unwrap() <= start.unwrap())
                        && (start.is_none() || due_date >= start.unwrap())
                        && (end.is_none() || due_date < end.unwrap())
                } else {
                    false
                }
            })
            .map(|x| {
                use chrono::Datelike;

                let due_date = x.due_date.unwrap();

                if due_date.month0() == month {
                    self.widgets.calendar.mark_day(due_date.day());
                }

                x.clone()
            })
            .collect();

        tasks
    }
}

#[relm_derive::widget]
impl relm::Widget for Widget {
    fn model(_: ()) {}

    fn update(&mut self, event: Msg) {
        use Msg::*;

        match event {
            Complete(_) => (),
            Edit(_) => (),
            Selected => self.update_tasks(),
            Select(date) => {
                use chrono::Datelike;

                self.widgets
                    .calendar
                    .select_month(date.month0(), date.year() as u32);
                self.widgets.calendar.select_day(date.day());
            }
            Update => self.update_tasks(),
        }
    }

    view! {
        gtk::Box {
            orientation: gtk::Orientation::Horizontal,
            spacing: 10,
            gtk::Box {
                orientation: gtk::Orientation::Vertical,
                #[name="calendar"]
                gtk::Calendar {
                    day_selected => Msg::Selected,
                },
                gtk::Button {
                    child: {
                        padding: 5,
                    },
                    label: "Today",
                    clicked => Msg::Select(chrono::Local::now()),
                },
            },
            gtk::ScrolledWindow {
                child: {
                    expand: true,
                },
                gtk::Box {
                    orientation: gtk::Orientation::Vertical,
                    #[name="past_exp"]
                    gtk::Expander {
                        label: Some("Past due"),
                        #[name="past"]
                        Tasks {
                            vscrollbar_policy: gtk::PolicyType::Never,
                            Complete(ref task) => Msg::Complete(task.clone()),
                            Edit(ref task) => Msg::Edit(task.clone()),
                        },
                    },
                    #[name="today_exp"]
                    gtk::Expander {
                        label: Some("Today"),
                        #[name="today"]
                        Tasks {
                            vscrollbar_policy: gtk::PolicyType::Never,
                            Complete(ref task) => Msg::Complete(task.clone()),
                            Edit(ref task) => Msg::Edit(task.clone()),
                        },
                    },
                    #[name="tomorrow_exp"]
                    gtk::Expander {
                        label: Some("Tomorrow"),
                        #[name="tomorrow"]
                        Tasks {
                            vscrollbar_policy: gtk::PolicyType::Never,
                            Complete(ref task) => Msg::Complete(task.clone()),
                            Edit(ref task) => Msg::Edit(task.clone()),
                        },
                    },
                    #[name="week_exp"]
                    gtk::Expander {
                        label: Some("This week"),
                        #[name="week"]
                        Tasks {
                            vscrollbar_policy: gtk::PolicyType::Never,
                            Complete(ref task) => Msg::Complete(task.clone()),
                            Edit(ref task) => Msg::Edit(task.clone()),
                        },
                    },
                    #[name="month_exp"]
                    gtk::Expander {
                        label: Some("This month"),
                        #[name="month"]
                        Tasks {
                            vscrollbar_policy: gtk::PolicyType::Never,
                            Complete(ref task) => Msg::Complete(task.clone()),
                            Edit(ref task) => Msg::Edit(task.clone()),
                        }
                    },
                },
            },
        }
    }
}
