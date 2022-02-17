use crate::edupage::{edupage::Edupage, edupage_traits::Ringing, edupage_types::RingingTime};

impl Ringing for Edupage {
    fn get_ringing_times(&self) -> Vec<RingingTime> {
        return self.data.as_ref().unwrap().ringing_times.clone();
    }

    fn get_next_lesson_time(&self, time: (i32, i32)) -> (i32, i32) {
        let ringing_times = self.get_ringing_times();

        for ringing_time in ringing_times {
            let start_time = ringing_time.start_time.time;
            let end_time = ringing_time.end_time.time;
            if start_time.0 < time.0 && end_time.1 < time.1 {
                return ringing_time.start_time.time;
            }
        }

        (0, 0)
    }
}
