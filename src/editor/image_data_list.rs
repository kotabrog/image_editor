use anyhow::Result;
use crate::engine::ImageDataWrapper;

const MAX_IMAGE_DATA_LIST_SIZE: usize = 10;

#[derive(Debug)]
pub struct ImageDataList {
    image_data_list: Vec<ImageDataWrapper>,
    current_index: usize,
}

impl ImageDataList {
    pub fn new() -> Self {
        Self {
            image_data_list: Vec::new(),
            current_index: 0,
        }
    }

    pub fn get_image_data(&self) -> Option<&ImageDataWrapper> {
        self.image_data_list.get(self.current_index)
    }

    pub fn get_image_data_inner_mut(&mut self) -> Option<&mut [u8]> {
        self.image_data_list.get_mut(self.current_index)
            .map(|image_data| image_data.data_mut())
    }

    pub fn is_empty(&self) -> bool {
        self.image_data_list.is_empty()
    }

    pub fn is_first(&self) -> bool {
        self.current_index == 0
    }

    pub fn is_last(&self) -> bool {
        self.is_empty() || self.current_index == self.image_data_list.len() - 1
    }

    pub fn push(&mut self, image_data: ImageDataWrapper) {
        self.image_data_list.truncate(self.current_index + 1);
        if self.image_data_list.len() == MAX_IMAGE_DATA_LIST_SIZE {
            self.image_data_list.remove(0);
        }
        self.image_data_list.push(image_data);
        self.current_index = self.image_data_list.len() - 1;
    }

    pub fn clone_push(&mut self) {
        if let Some(image_data) = self.image_data_list.get(self.current_index) {
            self.push(image_data.clone());
        }
    }

    pub fn data_to_image_data(&mut self) -> Result<()> {
        if let Some(image_data) = self.image_data_list.get_mut(self.current_index) {
            image_data.set_image_data()?;
        }
        Ok(())
    }

    pub fn undo(&mut self) -> Option<&ImageDataWrapper> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(&self.image_data_list[self.current_index])
        } else {
            None
        }
    }

    pub fn redo(&mut self) -> Option<&ImageDataWrapper> {
        if !self.image_data_list.is_empty() &&
                self.current_index < self.image_data_list.len() - 1 {
            self.current_index += 1;
            Some(&self.image_data_list[self.current_index])
        } else {
            None
        }
    }
}
