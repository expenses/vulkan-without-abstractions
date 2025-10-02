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
    }
}
