// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn stereo_msgs__msg__DisparityImage__init(msg: *mut DisparityImage) -> bool;
    fn stereo_msgs__msg__DisparityImage__fini(msg: *mut DisparityImage);
    fn stereo_msgs__msg__DisparityImage__Sequence__init(msg: *mut DisparityImageSequence, size: usize) -> bool;
    fn stereo_msgs__msg__DisparityImage__Sequence__fini(msg: *mut DisparityImageSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__stereo_msgs__msg__DisparityImage() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct DisparityImage {
    pub header: std_msgs::msg::Header,
    pub image: sensor_msgs::msg::Image,
    pub f: f32,
    pub t: f32,
    pub valid_window: sensor_msgs::msg::RegionOfInterest,
    pub min_disparity: f32,
    pub max_disparity: f32,
    pub delta_d: f32,
}

impl DisparityImage {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { stereo_msgs__msg__DisparityImage__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for DisparityImage {
    fn drop(&mut self) {
        unsafe { stereo_msgs__msg__DisparityImage__fini(self) };
    }
}

impl TopicMsg for DisparityImage {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__stereo_msgs__msg__DisparityImage()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct DisparityImageSequence {
    data: *mut DisparityImage,
    size: usize,
    capacity: usize,
}

impl DisparityImageSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { stereo_msgs__msg__DisparityImage__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[DisparityImage]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [DisparityImage]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for DisparityImageSequence {
    fn drop(&mut self) {
        unsafe { stereo_msgs__msg__DisparityImage__Sequence__fini(self) };
    }
}
