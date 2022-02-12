use crate::edupage::edupage::{Edupage, EdupageError};
use crate::edupage::edupage_traits::Timeline;
use crate::edupage::edupage_types;

impl Timeline for Edupage {
    fn filter_timeline_by_item_type(
        &self,
        item_type: edupage_types::TimelineItemType,
    ) -> Result<Vec<edupage_types::TimelineItem>, EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn);
        }

        let data = self.data.as_ref().unwrap();

        let mut items = Vec::new();
        for item in data.items.clone() {
            if item.item_type == item_type {
                items.push(item);
            }
        }

        Ok(items)
    }

    fn filter_timeline_by_item_types(
        &self,
        item_types: Vec<edupage_types::TimelineItemType>,
    ) -> Result<Vec<edupage_types::TimelineItem>, EdupageError> {
        if !self.is_logged_in {
            return Err(EdupageError::NotLoggedIn);
        }

        let data = self.data.as_ref().unwrap();

        let mut items = Vec::new();
        for item in data.items.clone() {
            if item_types.contains(&item.item_type) {
                items.push(item);
            }
        }

        Ok(items)
    }
}
