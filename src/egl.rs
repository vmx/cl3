// Copyright (c) 2021 Via Technology Ltd. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! OpenCL OpenGl ES Interoperability API.

#[allow(unused_imports)]
use super::error_codes::{CL_INVALID_VALUE, CL_SUCCESS};
pub use super::ffi::cl_egl::*;
#[allow(unused_imports)]
pub use cl_sys::{cl_context, cl_event, cl_int, cl_mem_flags};
#[allow(unused_imports)]
use std::ptr;

/// Create an OpenCL image object, from the EGLImage source provided as image.  
/// Requires the cl_khr_egl_image extension.  
/// Calls clCreateFromEGLImageKHR to create an OpenCL memory object.  
///
/// * `context` - a valid OpenCL context created from an OpenGL context.
/// * `display` - should be of type EGLDisplay, cast into the type CLeglDisplayKHR
/// * `image` - should be of type EGLImageKHR, cast into the type CLeglImageKHR.  
/// * `flags` -  usage information about the memory object being created.  
/// * `properties` - a null terminated list of property names and their
/// corresponding values.  
///
/// returns a Result containing the new OpenCL image object
/// or the error code from the OpenCL C API function.
#[cfg(feature = "cl_khr_egl_image")]
#[inline]
pub fn create_from_egl_image(
    context: cl_context,
    display: CLeglDisplayKHR,
    image: CLeglImageKHR,
    flags: cl_mem_flags,
    properties: *const cl_egl_image_properties_khr,
) -> Result<cl_mem, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let mem =
        unsafe { clCreateFromEGLImageKHR(context, display, image, flags, properties, &mut status) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(mem)
    }
}

/// Acquire OpenCL memory objects that have been created from EGL resources.  
/// Requires the cl_khr_egl_image extension.  
/// Calls clEnqueueAcquireEGLObjectsKHR.  
///
/// * `command_queue` - a valid OpenCL command_queue.
/// * `num_objects` - the number of memory objects to acquire.
/// * `mem_objects` - the memory objects to acquire.
/// * `num_events_in_wait_list` - the number of events in the wait list.
/// * `event_wait_list` - the wait list events.
///
/// returns a Result containing the new OpenCL event
/// or the error code from the OpenCL C API function.
#[cfg(feature = "cl_khr_egl_image")]
#[inline]
pub fn enqueue_acquire_egl_objects(
    command_queue: cl_command_queue,
    num_objects: cl_uint,
    mem_objects: *const cl_mem,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueAcquireEGLObjectsKHR(
            command_queue,
            num_objects,
            mem_objects,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

/// Release OpenCL memory objects that have been created from EGL resources.  
/// Requires the cl_khr_egl_image extension.  
/// Calls clEnqueueReleaseEGLObjectsKHR.  
///
/// * `command_queue` - a valid OpenCL command_queue.
/// * `num_objects` - the number of memory objects to acquire.
/// * `mem_objects` - the memory objects to acquire.
/// * `num_events_in_wait_list` - the number of events in the wait list.
/// * `event_wait_list` - the wait list events.
///
/// returns a Result containing the new OpenCL event
/// or the error code from the OpenCL C API function.
#[cfg(feature = "cl_khr_egl_image")]
#[inline]
pub fn enqueue_release_egl_objects(
    command_queue: cl_command_queue,
    num_objects: cl_uint,
    mem_objects: *const cl_mem,
    num_events_in_wait_list: cl_uint,
    event_wait_list: *const cl_event,
) -> Result<cl_event, cl_int> {
    let mut event: cl_event = ptr::null_mut();
    let status: cl_int = unsafe {
        clEnqueueReleaseEGLObjectsKHR(
            command_queue,
            num_objects,
            mem_objects,
            num_events_in_wait_list,
            event_wait_list,
            &mut event,
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}

/// Create an event object linked to an EGL fence sync object.  
/// Requires the cl_khr_egl_event extension
/// Calls clCreateEventFromEGLSyncKHR.  
///
/// * `context` - a valid OpenCL context.
/// * `sync` - the handle to an EGLSync object.  
/// * `display` - the handle to an EGLDisplay.  
///
/// returns a Result containing the new OpenCL event
/// or the error code from the OpenCL C API function.
#[cfg(feature = "cl_khr_egl_event")]
#[inline]
pub fn create_event_from_egl_sync_khr(
    context: cl_context,
    sync: CLeglSyncKHR,
    display: CLeglDisplayKHR,
) -> Result<cl_event, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let event: cl_event =
        unsafe { clCreateEventFromEGLSyncKHR(context, sync, display, &mut status) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(event)
    }
}
