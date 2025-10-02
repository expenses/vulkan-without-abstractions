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
    }
}
