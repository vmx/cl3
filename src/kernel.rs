// Copyright (c) 2020-2021 Via Technology Ltd. All Rights Reserved.
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

//! OpenCL Kernel Object API.

#![allow(non_camel_case_types)]

pub use cl_sys::{
    CL_KERNEL_ARG_ACCESS_NONE, CL_KERNEL_ARG_ACCESS_READ_ONLY, CL_KERNEL_ARG_ACCESS_READ_WRITE,
    CL_KERNEL_ARG_ACCESS_WRITE_ONLY, CL_KERNEL_ARG_ADDRESS_CONSTANT, CL_KERNEL_ARG_ADDRESS_GLOBAL,
    CL_KERNEL_ARG_ADDRESS_LOCAL, CL_KERNEL_ARG_ADDRESS_PRIVATE, CL_KERNEL_ARG_TYPE_CONST,
    CL_KERNEL_ARG_TYPE_NONE, CL_KERNEL_ARG_TYPE_PIPE, CL_KERNEL_ARG_TYPE_RESTRICT,
    CL_KERNEL_ARG_TYPE_VOLATILE, CL_KERNEL_EXEC_INFO_SVM_FINE_GRAIN_SYSTEM,
    CL_KERNEL_EXEC_INFO_SVM_PTRS,
};

use super::error_codes::{CL_INVALID_VALUE, CL_SUCCESS};
use super::info_type::InfoType;
#[allow(unused_imports)]
use super::types::{
    cl_device_id, cl_int, cl_kernel, cl_kernel_arg_access_qualifier,
    cl_kernel_arg_address_qualifier, cl_kernel_arg_info, cl_kernel_exec_info, cl_kernel_info,
    cl_kernel_sub_group_info, cl_kernel_work_group_info, cl_program, cl_uint, cl_ulong,
};
use super::{
    api2_info_size, api2_info_value, api2_info_vector, api_info_size, api_info_value,
    api_info_vector,
};
#[allow(unused_imports)]
use cl_sys::{
    clCloneKernel, clCreateKernel, clCreateKernelsInProgram, clGetKernelArgInfo, clGetKernelInfo,
    clGetKernelSubGroupInfo, clGetKernelWorkGroupInfo, clReleaseKernel, clRetainKernel,
    clSetKernelArg, clSetKernelArgSVMPointer, clSetKernelExecInfo,
};

use libc::{c_void, intptr_t, size_t};
use std::ffi::CStr;
use std::mem;
use std::ptr;

/// Create an OpenCL kernel object for a program with a successfully built executable.  
/// Calls clCreateKernel to create an OpenCL kernel object.  
///
/// * `program` - a valid OpenCL program.
/// * `kernel_name` - a kernel function name in the program.
///
/// returns a Result containing the new OpenCL kernel object
/// or the error code from the OpenCL C API function.
#[inline]
pub fn create_kernel(program: cl_program, kernel_name: &CStr) -> Result<cl_kernel, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let kernel: cl_kernel = unsafe { clCreateKernel(program, kernel_name.as_ptr(), &mut status) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(kernel)
    }
}

fn count_kernels_in_program(program: cl_program) -> Result<cl_uint, cl_int> {
    let mut count: cl_uint = 0;
    let status: cl_int =
        unsafe { clCreateKernelsInProgram(program, 0, ptr::null_mut(), &mut count) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(count)
    }
}

/// Create OpenCL kernel objects for all kernel functions in a program.  
/// Calls clCreateKernelsInProgram to create OpenCL kernel objects.  
///
/// * `program` - a valid OpenCL program.
///
/// returns a Result containing the new OpenCL kernel objects
/// or the error code from the OpenCL C API function.
#[inline]
pub fn create_kernels_in_program(program: cl_program) -> Result<Vec<cl_kernel>, cl_int> {
    let count: cl_uint = count_kernels_in_program(program)?;
    let mut kernels: Vec<cl_kernel> = Vec::with_capacity(count as usize);
    let status: cl_int = unsafe {
        kernels.set_len(count as usize);
        clCreateKernelsInProgram(
            program,
            count,
            kernels.as_mut_ptr() as *mut cl_kernel,
            ptr::null_mut(),
        )
    };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(kernels)
    }
}

/// Clone an OpenCL kernel object.  
/// Calls clCloneKernel to clone an OpenCL kernel object.  
/// CL_VERSION_2_1
///
/// * `source_kernel` - a valid OpenCL cl_kernel object that will be copied.
///
/// returns a Result containing the new OpenCL kernel object
/// or the error code from the OpenCL C API function.
#[cfg(feature = "CL_VERSION_2_1")]
#[inline]
pub fn clone_kernel(source_kernel: cl_kernel) -> Result<cl_kernel, cl_int> {
    let mut status: cl_int = CL_INVALID_VALUE;
    let kernel: cl_kernel = unsafe { clCloneKernel(source_kernel, &mut status) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(kernel)
    }
}

/// Retain an OpenCL kernel.  
/// Calls clRetainKernel to increment the kernel reference count.
///
/// * `program` - the OpenCL kernel.
///
/// returns an empty Result or the error code from the OpenCL C API function.
#[inline]
pub fn retain_kernel(kernel: cl_kernel) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clRetainKernel(kernel) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Release an OpenCL kernel.  
/// Calls clReleaseKernel to decrement the kernel reference count.
///
/// * `kernel` - the OpenCL kernel.
///
/// returns an empty Result or the error code from the OpenCL C API function.
#[inline]
pub fn release_kernel(kernel: cl_kernel) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clReleaseKernel(kernel) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Set the argument value for a specific argument of a kernel.  
/// Calls clSetKernelArg.  
///
/// * `kernel` - the OpenCL kernel.
/// * `arg_index` - the kernel argument index.
/// * `arg_ptr` - pointer to the data for the argument at arg_index.
///
/// returns an empty Result or the error code from the OpenCL C API function.
#[inline]
pub fn set_kernel_arg(
    kernel: cl_kernel,
    arg_index: cl_uint,
    arg_size: size_t,
    arg_value: *const c_void,
) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clSetKernelArg(kernel, arg_index, arg_size, arg_value) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Set set a SVM pointer as the argument value for a specific argument of a kernel.  
/// Calls clSetKernelArgSVMPointer.  
///
/// * `kernel` - the OpenCL kernel.
/// * `arg_index` - the kernel argument index.
/// * `arg_ptr` - the SVM pointer to the data for the argument at arg_index.
///
/// returns an empty Result or the error code from the OpenCL C API function.
#[inline]
pub fn set_kernel_arg_svm_pointer(
    kernel: cl_kernel,
    arg_index: cl_uint,
    arg_ptr: *const c_void,
) -> Result<(), cl_int> {
    let status: cl_int = unsafe { clSetKernelArgSVMPointer(kernel, arg_index, arg_ptr) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Pass additional information other than argument values to a kernel.  
/// Calls clSetKernelExecInfo.  
///
/// * `kernel` - the OpenCL kernel.
/// * `param_name` - the information to be passed to kernel, see:
/// [Kernel Execution Properties](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#kernel-exec-info-table).
/// * `param_ptr` - pointer to the data for the param_name.
///
/// returns an empty Result or the error code from the OpenCL C API function.
#[inline]
pub fn set_kernel_exec_info(
    kernel: cl_kernel,
    param_name: cl_kernel_exec_info,
    param_value_size: size_t,
    param_value: *const c_void,
) -> Result<(), cl_int> {
    let status: cl_int =
        unsafe { clSetKernelExecInfo(kernel, param_name, param_value_size, param_value) };
    if CL_SUCCESS != status {
        Err(status)
    } else {
        Ok(())
    }
}

/// Get data about an OpenCL kernel.
/// Calls clGetKernelInfo to get the desired data about the kernel.
pub fn get_kernel_data(
    kernel: cl_kernel,
    param_name: cl_kernel_info,
) -> Result<Vec<u8>, cl_int> {
    api_info_size!(get_size, clGetKernelInfo);
    let size = get_size(kernel, param_name)?;
    api_info_vector!(get_vector, u8, clGetKernelInfo);
    Ok(get_vector(kernel, param_name, size)?)
}

// cl_kernel_info
#[derive(Clone, Copy, Debug)]
pub enum KernelInfo {
    CL_KERNEL_FUNCTION_NAME = 0x1190,
    CL_KERNEL_NUM_ARGS = 0x1191,
    CL_KERNEL_REFERENCE_COUNT = 0x1192,
    CL_KERNEL_CONTEXT = 0x1193,
    CL_KERNEL_PROGRAM = 0x1194,
    // CL_VERSION_1_2
    CL_KERNEL_ATTRIBUTES = 0x1195,
}

/// Get specific information about an OpenCL kernel.  
/// Calls clGetKernelInfo to get the desired information about the kernel.
///
/// * `kernel` - the OpenCL kernel.
/// * `param_name` - the type of kernel information being queried, see:
/// [Kernel Object Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#kernel-info-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_kernel_info(kernel: cl_kernel, param_name: KernelInfo) -> Result<InfoType, cl_int> {
    let param_id = param_name as cl_kernel_info;
    match param_name {
        KernelInfo::CL_KERNEL_FUNCTION_NAME | KernelInfo::CL_KERNEL_ATTRIBUTES => {
            Ok(InfoType::VecUchar(get_kernel_data(kernel, param_id)?))
        }

        KernelInfo::CL_KERNEL_NUM_ARGS | KernelInfo::CL_KERNEL_REFERENCE_COUNT => {
            api_info_value!(get_value, cl_uint, clGetKernelInfo);
            Ok(InfoType::Uint(get_value(kernel, param_id)?))
        }

        KernelInfo::CL_KERNEL_CONTEXT | KernelInfo::CL_KERNEL_PROGRAM => {
            api_info_value!(get_value, intptr_t, clGetKernelInfo);
            Ok(InfoType::Ptr(get_value(kernel, param_id)?))
        }
    }
}

/// Get data about arguments of an OpenCL kernel.
/// Calls clGetKernelArgInfo to get the desired data about arguments of the kernel.
pub fn get_kernel_arg_data(
    kernel: cl_kernel,
    arg_indx: cl_uint,
    param_name: cl_kernel_info,
) -> Result<Vec<u8>, cl_int> {
    api2_info_size!(get_size, cl_uint, clGetKernelArgInfo);
    let size = get_size(kernel, arg_indx, param_name)?;
    api2_info_vector!(get_vector, cl_uint, u8, clGetKernelArgInfo);
    Ok(get_vector(kernel, arg_indx, param_name, size)?)
}

// cl_kernel_arg_info
#[derive(Clone, Copy, Debug)]
pub enum KernelArgInfo {
    CL_KERNEL_ARG_ADDRESS_QUALIFIER = 0x1196,
    CL_KERNEL_ARG_ACCESS_QUALIFIER = 0x1197,
    CL_KERNEL_ARG_TYPE_NAME = 0x1198,
    CL_KERNEL_ARG_TYPE_QUALIFIER = 0x1199,
    CL_KERNEL_ARG_NAME = 0x119A,
}

/// Get specific information about arguments of an OpenCL kernel.  
/// Calls clGetKernelArgInfo to get the desired information about the kernel.
///
/// * `kernel` - the OpenCL kernel.
/// * `arg_index` - the kernel argument index.
/// * `param_name` - the type of kernel information being queried, see:
/// [Kernel Argument Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#kernel-argument-info-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_kernel_arg_info(
    kernel: cl_kernel,
    arg_indx: cl_uint,
    param_name: KernelArgInfo,
) -> Result<InfoType, cl_int> {
    let param_id = param_name as cl_kernel_arg_info;
    match param_name {
        KernelArgInfo::CL_KERNEL_ARG_ADDRESS_QUALIFIER
        | KernelArgInfo::CL_KERNEL_ARG_ACCESS_QUALIFIER => {
            api2_info_value!(get_index_value, cl_uint, cl_uint, clGetKernelArgInfo);
            Ok(InfoType::Uint(get_index_value(kernel, arg_indx, param_id)?))
        }

        KernelArgInfo::CL_KERNEL_ARG_TYPE_QUALIFIER => {
            api2_info_value!(get_index_value, cl_uint, cl_ulong, clGetKernelArgInfo);
            Ok(InfoType::Ulong(get_index_value(
                kernel, arg_indx, param_id,
            )?))
        }

        KernelArgInfo::CL_KERNEL_ARG_TYPE_NAME | KernelArgInfo::CL_KERNEL_ARG_NAME => {
            Ok(InfoType::VecUchar(get_kernel_arg_data(kernel, arg_indx, param_id)?))
        }
    }
}

/// Get data about work groups of an OpenCL kernel.
/// Calls clGetKernelArgInfo to get the desired data about work groups of the kernel.
pub fn get_kernel_work_group_data(
    kernel: cl_kernel,
    device: cl_device_id,
    param_name: cl_kernel_work_group_info,
) -> Result<Vec<u8>, cl_int> {
    api2_info_size!(get_size, cl_device_id, clGetKernelWorkGroupInfo);
    let size = get_size(kernel, device, param_name)?;
    api2_info_vector!(get_vector, cl_device_id, u8, clGetKernelWorkGroupInfo);
    Ok(get_vector(kernel, device, param_name, size)?)
}

// cl_kernel_work_group_info
#[derive(Clone, Copy, Debug)]
pub enum KernelWorkGroupInfo {
    CL_KERNEL_WORK_GROUP_SIZE = 0x11B0,
    CL_KERNEL_COMPILE_WORK_GROUP_SIZE = 0x11B1,
    CL_KERNEL_LOCAL_MEM_SIZE = 0x11B2,
    CL_KERNEL_PREFERRED_WORK_GROUP_SIZE_MULTIPLE = 0x11B3,
    CL_KERNEL_PRIVATE_MEM_SIZE = 0x11B4,
    CL_KERNEL_GLOBAL_WORK_SIZE = 0x11B5,
}

/// Get specific information about work groups of an OpenCL kernel.  
/// Calls clGetKernelWorkGroupInfo to get the desired information about the kernel.
///
/// * `kernel` - the OpenCL kernel.
/// * `device` - a specific device in the list of devices associated with kernel.
/// * `param_name` - the type of kernel information being queried, see:
/// [Kernel Object Device Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#kernel-workgroup-info-table).
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
pub fn get_kernel_work_group_info(
    kernel: cl_kernel,
    device: cl_device_id,
    param_name: KernelWorkGroupInfo,
) -> Result<InfoType, cl_int> {
    let param_id = param_name as cl_kernel_work_group_info;
    match param_name {
        KernelWorkGroupInfo::CL_KERNEL_WORK_GROUP_SIZE
        | KernelWorkGroupInfo::CL_KERNEL_PREFERRED_WORK_GROUP_SIZE_MULTIPLE => {
            api2_info_value!(
                get_index_value,
                cl_device_id,
                size_t,
                clGetKernelWorkGroupInfo
            );
            Ok(InfoType::Size(get_index_value(kernel, device, param_id)?))
        }

        KernelWorkGroupInfo::CL_KERNEL_COMPILE_WORK_GROUP_SIZE
        | KernelWorkGroupInfo::CL_KERNEL_GLOBAL_WORK_SIZE => {
            api2_info_size!(get_device_size, cl_device_id, clGetKernelWorkGroupInfo);
            api2_info_vector!(
                get_device_vec,
                cl_device_id,
                size_t,
                clGetKernelWorkGroupInfo
            );
            let size = get_device_size(kernel, device, param_id)?;
            Ok(InfoType::VecSize(get_device_vec(
                kernel, device, param_id, size,
            )?))
        }

        KernelWorkGroupInfo::CL_KERNEL_LOCAL_MEM_SIZE
        | KernelWorkGroupInfo::CL_KERNEL_PRIVATE_MEM_SIZE => {
            api2_info_value!(
                get_index_value,
                cl_device_id,
                cl_ulong,
                clGetKernelWorkGroupInfo
            );
            Ok(InfoType::Ulong(get_index_value(kernel, device, param_id)?))
        }
    }
}

// cl_kernel_sub_group_info
#[derive(Clone, Copy, Debug)]
pub enum KernelSubGroupInfo {
    CL_KERNEL_MAX_SUB_GROUP_SIZE_FOR_NDRANGE = 0x2033,
    CL_KERNEL_SUB_GROUP_COUNT_FOR_NDRANGE = 0x2034,
    CL_KERNEL_LOCAL_SIZE_FOR_SUB_GROUP_COUNT = 0x11B8,
    CL_KERNEL_MAX_NUM_SUB_GROUPS = 0x11B9,
    CL_KERNEL_COMPILE_NUM_SUB_GROUPS = 0x11BA,
}

/// Get specific information about sub groups of an OpenCL kernel.  
/// Calls clGetKernelSubGroupInfo to get the desired information about the kernel.  
/// CL_VERSION_2_1
///
/// * `kernel` - the OpenCL kernel.
/// * `device` - a specific device in the list of devices associated with kernel.
/// * `param_name` - the type of kernel information being queried, see:
/// [Kernel Object Subgroup Queries](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html#kernel-subgroup-info-table).
/// * `input_value_size` - the size in bytes of memory pointed to by input_value.
/// * `input_value` -  pointer to memory where the appropriate parameterization
/// of the query is passed from.
///
/// returns a Result containing the desired information in an InfoType enum
/// or the error code from the OpenCL C API function.
#[cfg(feature = "CL_VERSION_2_1")]
pub fn get_kernel_sub_group_info(
    kernel: cl_kernel,
    device: cl_device_id,
    param_name: KernelSubGroupInfo,
    input_value_size: size_t,
    input_value: *const c_void,
) -> Result<InfoType, cl_int> {
    let mut size: size_t = mem::size_of::<size_t>();
    let param_id = param_name as cl_kernel_sub_group_info;
    match param_name {
        KernelSubGroupInfo::CL_KERNEL_MAX_SUB_GROUP_SIZE_FOR_NDRANGE
        | KernelSubGroupInfo::CL_KERNEL_SUB_GROUP_COUNT_FOR_NDRANGE
        | KernelSubGroupInfo::CL_KERNEL_MAX_NUM_SUB_GROUPS
        | KernelSubGroupInfo::CL_KERNEL_COMPILE_NUM_SUB_GROUPS => {
            // get the value
            let mut data: size_t = 0;
            let data_ptr: *mut size_t = &mut data;
            let status = unsafe {
                clGetKernelSubGroupInfo(
                    kernel,
                    device,
                    param_id,
                    input_value_size,
                    input_value,
                    size,
                    data_ptr as *mut c_void,
                    ptr::null_mut(),
                )
            };
            if CL_SUCCESS != status {
                Err(status)
            } else {
                Ok(InfoType::Size(data))
            }
        }

        KernelSubGroupInfo::CL_KERNEL_LOCAL_SIZE_FOR_SUB_GROUP_COUNT => {
            // get the size
            let status: cl_int = unsafe {
                clGetKernelSubGroupInfo(
                    kernel,
                    device,
                    param_id,
                    input_value_size,
                    input_value,
                    0,
                    ptr::null_mut(),
                    &mut size,
                )
            };
            if CL_SUCCESS != status {
                Err(status)
            } else {
                // Get the information.
                let count = size / mem::size_of::<size_t>();
                let mut data: Vec<size_t> = Vec::with_capacity(count);
                let status = unsafe {
                    data.set_len(count);
                    clGetKernelSubGroupInfo(
                        kernel,
                        device,
                        param_id,
                        input_value_size,
                        input_value,
                        size,
                        data.as_mut_ptr() as *mut c_void,
                        ptr::null_mut(),
                    )
                };
                if CL_SUCCESS != status {
                    Err(status)
                } else {
                    Ok(InfoType::VecSize(data))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{create_context, release_context};
    use crate::device::{get_device_ids, CL_DEVICE_TYPE_GPU};
    use crate::error_codes::error_text;
    use crate::platform::get_platform_ids;
    use crate::program::{build_program, create_program_with_source, release_program};
    use std::ffi::CString;

    #[test]
    fn test_kernel() {
        let platform_ids = get_platform_ids().unwrap();

        // Choose the first platform
        let platform_id = platform_ids[0];

        let device_ids = get_device_ids(platform_id, CL_DEVICE_TYPE_GPU).unwrap();
        assert!(0 < device_ids.len());

        let device_id = device_ids[0];

        let context = create_context(&device_ids, ptr::null(), None, ptr::null_mut());
        let context = context.unwrap();

        let source = r#"
            kernel void saxpy_float (global float* z,
                global float const* x,
                global float const* y,
                float a)
            {
            size_t i = get_global_id(0);
            z[i] = a*x[i] + y[i];
            }
        "#;

        // Convert source to an array
        let sources = [source];
        let program = create_program_with_source(context, &sources).unwrap();

        let options = CString::new("-cl-kernel-arg-info").unwrap();
        build_program(program, &device_ids, &options, None, ptr::null_mut()).unwrap();

        let kernel_name = "saxpy_float";
        let name = CString::new(kernel_name).unwrap();
        let kernel = create_kernel(program, &name).unwrap();

        let value = get_kernel_info(kernel, KernelInfo::CL_KERNEL_FUNCTION_NAME).unwrap();
        let value = value.to_string();
        println!("CL_KERNEL_FUNCTION_NAME: {}", value);
        assert!(0 < value.len());

        let value = get_kernel_info(kernel, KernelInfo::CL_KERNEL_NUM_ARGS).unwrap();
        let value = value.to_uint();
        println!("CL_KERNEL_NUM_ARGS: {}", value);
        assert!(0 < value);

        let value = get_kernel_info(kernel, KernelInfo::CL_KERNEL_REFERENCE_COUNT).unwrap();
        let value = value.to_uint();
        println!("CL_KERNEL_REFERENCE_COUNT: {}", value);
        assert!(0 < value);

        let value = get_kernel_info(kernel, KernelInfo::CL_KERNEL_CONTEXT).unwrap();
        let value = value.to_ptr();
        println!("CL_KERNEL_CONTEXT: {}", value);
        assert!(0 < value);

        let value = get_kernel_info(kernel, KernelInfo::CL_KERNEL_PROGRAM).unwrap();
        let value = value.to_ptr();
        println!("CL_KERNEL_PROGRAM: {}", value);
        assert!(0 < value);

        let value = get_kernel_info(kernel, KernelInfo::CL_KERNEL_ATTRIBUTES).unwrap();
        let value = value.to_string();
        println!("CL_KERNEL_ATTRIBUTES: {}", value);

        match get_kernel_arg_info(kernel, 0, KernelArgInfo::CL_KERNEL_ARG_ADDRESS_QUALIFIER) {
            Ok(value) => {
                let value = value.to_uint();
                println!("CL_KERNEL_ARG_ADDRESS_QUALIFIER: {:X}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_KERNEL_ARG_ADDRESS_QUALIFIER: {}",
                error_text(e)
            ),
        }

        match get_kernel_arg_info(kernel, 0, KernelArgInfo::CL_KERNEL_ARG_ACCESS_QUALIFIER) {
            Ok(value) => {
                let value = value.to_uint();
                println!("CL_KERNEL_ARG_ACCESS_QUALIFIER: {:X}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_KERNEL_ARG_ACCESS_QUALIFIER: {}",
                error_text(e)
            ),
        }

        match get_kernel_arg_info(kernel, 0, KernelArgInfo::CL_KERNEL_ARG_TYPE_NAME) {
            Ok(value) => {
                let value = value.to_string();
                println!("CL_KERNEL_ARG_TYPE_NAME: {}", value);
                assert!(0 < value.len())
            }
            Err(e) => println!("OpenCL error, CL_KERNEL_ARG_TYPE_NAME: {}", error_text(e)),
        }

        match get_kernel_arg_info(kernel, 0, KernelArgInfo::CL_KERNEL_ARG_TYPE_QUALIFIER) {
            Ok(value) => {
                let value = value.to_ulong();
                println!("CL_KERNEL_ARG_TYPE_QUALIFIER: {:X}", value)
            }
            Err(e) => println!(
                "OpenCL error, CL_KERNEL_ARG_TYPE_QUALIFIER: {}",
                error_text(e)
            ),
        }

        match get_kernel_arg_info(kernel, 0, KernelArgInfo::CL_KERNEL_ARG_NAME) {
            Ok(value) => {
                let value = value.to_string();
                println!("CL_KERNEL_ARG_NAME: {}", value);
                assert!(0 < value.len())
            }
            Err(e) => println!("OpenCL error, CL_KERNEL_ARG_NAME: {}", error_text(e)),
        }

        let value = get_kernel_work_group_info(
            kernel,
            device_id,
            KernelWorkGroupInfo::CL_KERNEL_WORK_GROUP_SIZE,
        )
        .unwrap();
        let value = value.to_size();
        println!("CL_KERNEL_WORK_GROUP_SIZE: {}", value);

        let value = get_kernel_work_group_info(
            kernel,
            device_id,
            KernelWorkGroupInfo::CL_KERNEL_COMPILE_WORK_GROUP_SIZE,
        )
        .unwrap();
        let value = value.to_vec_size();
        println!("CL_KERNEL_COMPILE_WORK_GROUP_SIZE: {}", value.len());

        let value = get_kernel_work_group_info(
            kernel,
            device_id,
            KernelWorkGroupInfo::CL_KERNEL_LOCAL_MEM_SIZE,
        )
        .unwrap();
        let value = value.to_ulong();
        println!("CL_KERNEL_LOCAL_MEM_SIZE: {}", value);

        let value = get_kernel_work_group_info(
            kernel,
            device_id,
            KernelWorkGroupInfo::CL_KERNEL_PREFERRED_WORK_GROUP_SIZE_MULTIPLE,
        )
        .unwrap();
        let value = value.to_size();
        println!("CL_KERNEL_PREFERRED_WORK_GROUP_SIZE_MULTIPLE: {}", value);

        let value = get_kernel_work_group_info(
            kernel,
            device_id,
            KernelWorkGroupInfo::CL_KERNEL_PRIVATE_MEM_SIZE,
        )
        .unwrap();
        let value = value.to_ulong();
        println!("CL_KERNEL_PRIVATE_MEM_SIZE: {}", value);

        match get_kernel_work_group_info(
            kernel,
            device_id,
            KernelWorkGroupInfo::CL_KERNEL_GLOBAL_WORK_SIZE,
        ) {
            Ok(value) => {
                let value = value.to_vec_size();
                println!("CL_KERNEL_GLOBAL_WORK_SIZE: {}", value.len())
            }
            Err(e) => println!(
                "OpenCL error, CL_KERNEL_GLOBAL_WORK_SIZE: {}",
                error_text(e)
            ),
        }

        release_kernel(kernel).unwrap();
        release_program(program).unwrap();
        release_context(context).unwrap();
    }
}
