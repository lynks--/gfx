// Copyright 2015 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Vulkan backend implementation

extern crate gfx_core;

// === placeholder vk ===================================================================
pub mod vk {
    pub type VK_DEFINE_HANDLE = u64;
    pub type VK_DEFINE_NON_DISPATCHABLE_HANDLE = u64;

    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
    pub enum VkResult {
        VK_SUCCESS,
        VK_NOT_READY,
        VK_TIMEOUT,
        VK_EVENT_SET,
        VK_EVENT_RESET,
        VK_INCOMPLETE,
        VK_SUBOPTIMAL_KHR,

        VK_ERROR_OUT_OF_HOST_MEMORY,
        VK_ERROR_OUT_OF_DEVICE_MEMORY,
        VK_ERROR_INITIALIZATION_FAILED,
        VK_ERROR_DEVICE_LOST,
        VK_ERROR_MEMORY_MAP_FAILED
        VK_ERROR_LAYER_NOT_PRESENT,
        VK_ERROR_EXTENSION_NOT_PRESENT,
        VK_ERROR_FEATURE_NOT_PRESENT,
        VK_ERROR_INCOMPATIBLE_DRIVER,
        VK_ERROR_TOO_MANY_OBJECTS,
        VK_ERROR_FORMAT_NOT_SUPPORTED,

        VK_ERROR_SURFACE_LOST_KHR,
        VK_ERROR_OUT_OF_DATE_KHR,
        VK_ERROR_INCOMPATIBLE_DISPLAY_KHR,
        VK_ERROR_NATIVE_WINDOW_IN_USE_KHR,
        VK_ERROR_VALIDATION_FAILED_EXT,

        INTERNAL_ERROR_BAD_VK_RESULT,
    }
    impl VkResult {
        pub fn from_i32(r: i32) -> VkResult {
            match r {
                0   => VkResult::VK_SUCCESS,
                1   => VkResult::VK_NOT_READY,
                2   => VkResult::VK_TIMEOUT,
                3   => VkResult::VK_EVENT_SET,
                4   => VkResult::VK_EVENT_RESET,
                5   => VkResult::VK_INCOMPLETE,

                -1  => VkResult::VK_ERROR_OUT_OF_HOST_MEMORY,
                -2  => VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY,
                -3  => VkResult::VK_ERROR_INITIALIZATION_FAILED,
                -4  => VkResult::VK_ERROR_DEVICE_LOST,
                -5  => VkResult::VK_ERROR_MEMORY_MAP_FAILED,
                -6  => VkResult::VK_ERROR_LAYER_NOT_PRESENT,
                -7  => VkResult::VK_ERROR_EXTENSION_NOT_PRESENT,
                -8  => VkResult::VK_ERROR_FEATURE_NOT_PRESENT,
                -9  => VkResult::VK_ERROR_INCOMPATIBLE_DRIVER,
                -10 => VkResult::VK_ERROR_TOO_MANY_OBJECTS,
                -11 => VkResult::VK_ERROR_FORMAT_NOT_SUPPORTED,

                _   => {
                    println!("error: bad vk result: {}", r);
                    VkResult::INTERNAL_ERROR_BAD_VK_RESULT
                }
            }
        }
    }

    pub mod types {
        pub type VkInstance          = VK_DEFINE_HANDLE;
        pub type VkPhysicalDevice    = VK_DEFINE_HANDLE;
        pub type VkDevice            = VK_DEFINE_HANDLE;
        pub type VkCommandBuffer     = VK_DEFINE_HANDLE;
        pub type VkQueue             = VK_DEFINE_HANDLE;

        pub type VkBuffer            = VK_DEFINE_NON_DISPATCHABLE_HANDLE;
        pub type VkShaderModule      = VK_DEFINE_NON_DISPATCHABLE_HANDLE;
        pub type VkFramebuffer       = VK_DEFINE_NON_DISPATCHABLE_HANDLE;
        pub type VkImage             = VK_DEFINE_NON_DISPATCHABLE_HANDLE;
        pub type VkImageView         = VK_DEFINE_NON_DISPATCHABLE_HANDLE;
        pub type VkSampler           = VK_DEFINE_NON_DISPATCHABLE_HANDLE;
        pub type VkFence             = VK_DEFINE_NON_DISPATCHABLE_HANDLE;
        pub type VkPipeline          = VK_DEFINE_NON_DISPATCHABLE_HANDLE;
    }
}
// === end placeholder vk ===============================================================

// something like this might be handy for returning Results
macro_rules! vktry {
    ($e:expr) => {
        if $e < 0 {
            return Err(vk::VkResult::from_i32($e as i32));
        }
        else {
            $e
        }
    };
};

use gfx_core as d;

const PHYS_DEVICE_MAX_ENUM: u32 = 8;

pub type Buffer         = vk::types::VkBuffer;
pub type ArrayBuffer    = vk::types::VkBuffer;
pub type Shader         = vk::types::VkShaderModule;
pub type Program        = vk::types::VkShaderModule; //?
pub type FrameBuffer    = vk::types::VkFramebuffer;
pub type Surface        = vk::types::VkImageView; //?
pub type Texture        = vk::types::VkImage;
pub type Sampler        = vk::types::VkSampler;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Fence(vk::types::VkFence);

unsafe impl Send for Fence {}
unsafe impl Sync for Fence {}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Resources {}

impl d::Resources for Resources {
    type Buffer              = Buffer;
    type Shader              = Shader;
    type Program             = Program;
    type PipelineStateObject = PipelineState;
    type Texture             = NewTexture;
    type RenderTargetView    = TargetView;
    type DepthStencilView    = TargetView;
    type ShaderResourceView  = ResourceView;
    type UnorderedAccessView = ();
    type Sampler             = FatSampler;
    type Fence               = Fence;
}

pub struct Device {
    instance:        vk::types::VkInstance,
    device:            vk::types::VkDevice,
}

impl Device {
    pub fn new() -> Result<Device, vk::VkResult> {
        // create the instance
        let i: vk::types::VkInstance = {
            let mut i_: vk::types::VkInstance = 0;
            let create_info = vk::types::VkInstanceCreateInfo {
                sType:                   ??,
                pNext:                   ??,
                flags:                   ??,
                pApplicationInfo:        ??,
                enabledLayerCount:       ??,
                ppEnabledLayerNames:     ??,
                enabledExtensionCount:   ??,
                ppEnabledExtensionNames: ??,
            };

            let allocator: vk::types::VkAllocationCallbacks = ??;

            vktry!(unsafe {
                vk::Vk::vkCreateInstance(&create_info, &allocator, &i_)
            });

            i_
        };

        // create the vk device
        let d: vk::types::VkDevice = {
            let mut devices: [vk::types::VkDevice; PHYS_DEVICE_MAX_ENUM] = ??;
            let mut count: u32;

            vktry!(unsafe {
                vk::Vk::vkEnumeratePhysicalDevices(i, &count, &devices)
            });

            let mut d_: vk::types::VkDevice = 0;
            let create_info = vk::types::VkDeviceCreateInfo {
                sType:                    ??,
                // ... //
                pEnabledFeatures:         ??,
            }
            vktry!(unsafe {
                vk::Vk::vkCreateDevice(&devices[0], &create_info, &allocator, &d_);
            });

            d_
        };

        Ok(Device {
            instance:  i,
            device:    d,
        })
    }
}

impl d::Device for Device {
    type Resources = Resources;
    type CommandBuffer = command::CommandBuffer;

    fn get_capabilities<'a>(&'a self) -> &'a Capabilities {
    }

    fn reset_state(&mut self) {
    }

    fn submit(&mut self, SubmitInfo<Self>) {
    }

    fn cleanup(&mut self) {
    }
}


