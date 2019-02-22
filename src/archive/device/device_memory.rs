/**
 *
 * Abstraction layer over all of the device memories
 * 
 */

enum MemoryKind {
    Read,
    ReadWrite,
    DeviceOnly,
    Texture,
    Pixels
}

/**
 * Abstract core class
 */
trait DeviceMemory {
    fn new(device: &Device, name: &str, kind: MemoryKind) -> Self;

    /**
     * Host allocation on the device? All host_pointer memory should be
     * allocated with these functions, for devices that support using
     * the same pointer for host and device. */
    // host_alloc
    // host_free

    fn device_alloc(&self) {
        self.device().mem_alloc();
    }

    fn device_free(&self);

    fn device_copy_to(&self);

    fn device_copy_from(&self, y: i32, w: i32, h: i32, elem: i32);

    fn device_zero(&self);
}