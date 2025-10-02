use ash::vk;

fn main() {
    unsafe {
        // Create dynamically linked entry
        let entry = ash::Entry::load().unwrap();

        // Create instance, setting the version to 1.3
        let api_version = vk::make_api_version(0, 1, 3, 0);
        let instance = entry
            .create_instance(
                &vk::InstanceCreateInfo::default()
                    .application_info(&vk::ApplicationInfo::default().api_version(api_version)),
                None,
            )
            .unwrap();

        // List physical devices and just use the 1st (or 0th rather) one.
        // Hopefully your loader will list them in a sensible order otherwise
        // llvmpipe or something might be selected!
        let physical_devices = instance.enumerate_physical_devices().unwrap();
        let physical_device = physical_devices[0];

        let device = instance
            .create_device(
                physical_device,
                &vk::DeviceCreateInfo::default()
                    // Select a single queue from a single queue family with a priority
                    // of 1.0. Completely meaningless information when we're just using
                    // a single queue.
                    .queue_create_infos(&[
                        vk::DeviceQueueCreateInfo::default().queue_priorities(&[1.0])
                    ]),
                None,
            )
            .unwrap();

        // Get the queue, just picking the first one from the first family.
        // Hopefully these are listed in a sensible order by the driver otherwise
        // it could choose e.g. a transfer-only queue
        let queue = device.get_device_queue(0, 0);

        // Get the right type of memory for the buffer. We need to select a type that
        // is visible to the host (CPU code) in order to read it back. This is the
        // sort of thing that an GPU allocator library would do for you.
        let memory_types = instance.get_physical_device_memory_properties(physical_device);
        let host_visible_memory_index = memory_types
            .memory_types_as_slice()
            .iter()
            .position(|ty| {
                ty.property_flags.contains(
                    vk::MemoryPropertyFlags::DEVICE_LOCAL | vk::MemoryPropertyFlags::HOST_VISIBLE,
                )
            })
            .unwrap();

        let width = 100;
        let height = 100;
        let allocation_size = width * height * 4;

        let memory = device
            .allocate_memory(
                &vk::MemoryAllocateInfo::default()
                    .allocation_size(allocation_size)
                    .memory_type_index(host_visible_memory_index as u32),
                None,
            )
            .unwrap();

        // Create command pool with which to allocate command buffers
        let command_pool = device
            .create_command_pool(&vk::CommandPoolCreateInfo::default(), None)
            .unwrap();

        // Allocate a command buffer
        let command_buffers = device
            .allocate_command_buffers(
                &vk::CommandBufferAllocateInfo::default()
                    .command_buffer_count(1)
                    .command_pool(command_pool),
            )
            .unwrap();
        let command_buffer = command_buffers[0];

        // Create a buffer object that wraps the memory. As we want to be able to fill
        // it with values, the `TRANSFER_DST` usage flags needs to be set.
        let buffer = device
            .create_buffer(
                &vk::BufferCreateInfo::default()
                    .size(allocation_size)
                    .usage(vk::BufferUsageFlags::TRANSFER_DST),
                None,
            )
            .unwrap();

        // Bind the memory to the buffer
        device.bind_buffer_memory(buffer, memory, 0).unwrap();

        // Record into the command buffer
        device
            .begin_command_buffer(command_buffer, &vk::CommandBufferBeginInfo::default())
            .unwrap();
        device.cmd_fill_buffer(
            command_buffer,
            buffer,
            0,
            allocation_size,
            u32::from_le_bytes([20, 20, 35, 255]),
        );
        device.end_command_buffer(command_buffer).unwrap();

        // Create a fence, submit the command buffer to the queue and wait on the fence
        // with a timeout of 1 billion nanoseconds (1 second)

        let fence = device
            .create_fence(&vk::FenceCreateInfo::default(), None)
            .unwrap();

        device
            .queue_submit(
                queue,
                &[vk::SubmitInfo::default().command_buffers(&[command_buffer])],
                fence,
            )
            .unwrap();

        device
            .wait_for_fences(&[fence], true, 1_000_000_000)
            .unwrap();

        // Map the memory to the CPU, getting a pointer.
        let mapped_ptr = device
            .map_memory(memory, 0, allocation_size, vk::MemoryMapFlags::empty())
            .unwrap();

        let slice = std::slice::from_raw_parts(mapped_ptr as *const u8, allocation_size as usize);

        use std::io::Write;
        let mut output = std::io::BufWriter::new(std::fs::File::create("output.ppm").unwrap());
        write!(output, "P3 {} {} 255", width, height).unwrap();
        for rgba in slice.chunks(4) {
            write!(output, " {} {} {}", rgba[0], rgba[1], rgba[2]).unwrap();
        }
    }
}
