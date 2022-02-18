use chrono::NaiveDateTime;

use crate::edupage::{edupage::Edupage, edupage_traits::Ringing, edupage_types::RingingTime};

impl Ringing for Edupage {
    fn get_ringing_times(&self) -> Vec<RingingTime> {
        match &self.data {
            Some(x) => x.ringing_times.clone(),
            None => Vec::new(),
        }
    }

    fn get_next_lesson_time(&self, time: NaiveDateTime) -> Option<chrono::NaiveDateTime> {
        for lesson in self.get_ringing_times() {
            if time < lesson.start_time {
                return Some(lesson.start_time);
            } else if time < lesson.end_time {
                return Some(lesson.end_time);
            }
        }

        return None;
    }
}