//! (LLDB extension) Get information about the host we are remotely connected
//! to.
use crate::target::Target;

/// Target Extension - Get information about the host we are remotely connected
/// to.
pub trait LldbHostInfo: Target {
    /// A number that is the mach-o CPU type that is being debugged (base 10).
    fn cputype(&self) -> Option<u32> {
        None
    }
    /// A number that is the mach-o CPU subtype type that is being debugged
    /// (base 10).
    fn cpusubtype(&self) -> Option<u32> {
        None
    }
    /// A string for the target triple (x86_64-apple-macosx) that can be used to
    /// specify arch + vendor + os in one entry.
    fn triple(&self) -> Option<&[u8]> {
        None
    }
    /// A string for the target architecture (x86_64, arm64, etc.), not needed
    /// if "triple" is specified.
    fn arch(&self) -> Option<&str> {
        None
    }
    /// A string for the vendor (apple), not needed if "triple" is specified.
    fn vendor(&self) -> Option<&str> {
        None
    }
    /// A string for the OS being debugged (macosx, linux, freebsd, ios,
    /// watchos), not needed if "triple" is specified.
    fn ostype(&self) -> Option<&str> {
        None
    }
    /// Is one of "little", "big", or "pdp".
    fn endian(&self) -> Option<&str> {
        None
    }
    /// An unsigned number that represents how big pointers are in bytes on the
    fn ptrsize(&self) -> Option<u32> {
        None
    }
    /// The hostname of the host that is running the GDB server if available.
    fn hostname(&self) -> Option<&str> {
        None
    }
    /// A string for the OS build for the remote host as a string value.
    fn os_build(&self) -> Option<&str> {
        None
    }
    /// A string describing the kernel version.
    fn os_kernel(&self) -> Option<&str> {
        None
    }
    /// A version string that represents the current OS version (10.8.2).
    fn os_version(&self) -> Option<&str> {
        None
    }
    /// One of "before" or "after" to specify if a watchpoint is triggered
    /// before or after the pc when it stops.
    fn watchpoint_exceptions_received(&self) -> Option<&str> {
        None
    }
    /// An unsigned number that specifies the default timeout in seconds.
    fn default_packet_timeout(&self) -> Option<u32> {
        None
    }
    /// For linux, specifies distribution id (e.g. ubuntu, fedora, etc.)
    fn distribution_id(&self) -> Option<&str> {
        None
    }
    /// Specifies the major version number of the OS (e.g. for macOS 10.12.2, it
    /// would be 10).
    fn osmajor(&self) -> Option<u32> {
        None
    }
    /// Specifies the minor version number of the OS (e.g. for macOS 10.12.2, it
    /// would be 12).
    fn osminor(&self) -> Option<u32> {
        None
    }
    /// Specifies the patch level number of the OS (e.g. for macOS 10.12.2, it
    /// would be 2).
    fn ospatch(&self) -> Option<u32> {
        None
    }
    /// Specifies the target system VM page size, base 10.
    fn vm_page_size(&self) -> Option<u32> {
        None
    }
    /// Specifies how many bits in addresses are significant for addressing,
    /// base 10.
    fn addressing_bits(&self) -> Option<u32> {
        None
    }
    /// Specifies how many bits in addresses in low memory are significant for
    /// addressing, base 10.
    fn low_mem_addressing_bits(&self) -> Option<u32> {
        None
    }
    /// Specifies how many bits in addresses in high memory are significant for
    /// addressing, base 10.
    fn high_mem_addressing_bits(&self) -> Option<u32> {
        None
    }
}

define_ext!(LldbHostInfoOps, LldbHostInfo);
