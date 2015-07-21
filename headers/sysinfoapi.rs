#[cfg(feature="winapi_app")] #[repr(C)] pub struct SYSTEM_INFO { u: ::sysinfoapi::SYSTEM_INFO_Child_0, dwPageSize: ::minwindef::DWORD, lpMinimumApplicationAddress: ::minwindef::LPVOID, lpMaximumApplicationAddress: ::minwindef::LPVOID, dwActiveProcessorMask: ::basetsd::DWORD_PTR, dwNumberOfProcessors: ::minwindef::DWORD, dwProcessorType: ::minwindef::DWORD, dwAllocationGranularity: ::minwindef::DWORD, wProcessorLevel: ::minwindef::WORD, wProcessorRevision: ::minwindef::WORD } /* sysinfoapi.h:75:16, sysinfoapi.h:75:16, sysinfoapi.h:75:16 */
#[cfg(feature="winapi_app")] #[repr(C)] pub /*union*/ struct SYSTEM_INFO_Child_0 { _payload0: u32 } #[cfg(feature="winapi_app")] union_field! { SYSTEM_INFO_Child_0.{dwOemId, dwOemId_mut}: ::minwindef::DWORD } #[cfg(feature="winapi_app")] union_field! { SYSTEM_INFO_Child_0.{s, s_mut}: ::sysinfoapi::SYSTEM_INFO_Child_0_Child_1 } /* sysinfoapi.h:76:5, sysinfoapi.h:76:5, sysinfoapi.h:76:5 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct SYSTEM_INFO_Child_0_Child_1 { wProcessorArchitecture: ::minwindef::WORD, wReserved: ::minwindef::WORD } /* sysinfoapi.h:78:9, sysinfoapi.h:78:9, sysinfoapi.h:78:9 */
#[cfg(feature="winapi_app")] pub type LPSYSTEM_INFO = *mut ::sysinfoapi::SYSTEM_INFO; /* sysinfoapi.h:92:17, sysinfoapi.h:92:17, sysinfoapi.h:92:17 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct MEMORYSTATUSEX { dwLength: ::minwindef::DWORD, dwMemoryLoad: ::minwindef::DWORD, ullTotalPhys: ::winnt::DWORDLONG, ullAvailPhys: ::winnt::DWORDLONG, ullTotalPageFile: ::winnt::DWORDLONG, ullAvailPageFile: ::winnt::DWORDLONG, ullTotalVirtual: ::winnt::DWORDLONG, ullAvailVirtual: ::winnt::DWORDLONG, ullAvailExtendedVirtual: ::winnt::DWORDLONG } /* sysinfoapi.h:115:16, sysinfoapi.h:115:16, sysinfoapi.h:115:16 */
#[cfg(feature="winapi_desktop")] pub type LPMEMORYSTATUSEX = *mut ::sysinfoapi::MEMORYSTATUSEX; /* sysinfoapi.h:125:20, sysinfoapi.h:125:20, sysinfoapi.h:125:20 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub enum COMPUTER_NAME_FORMAT {ComputerNameNetBIOS = 0, ComputerNameDnsHostname = 1, ComputerNameDnsDomain = 2, ComputerNameDnsFullyQualified = 3, ComputerNamePhysicalNetBIOS = 4, ComputerNamePhysicalDnsHostname = 5, ComputerNamePhysicalDnsDomain = 6, ComputerNamePhysicalDnsFullyQualified = 7, ComputerNameMax = 8} pub use self::COMPUTER_NAME_FORMAT::{ComputerNameNetBIOS, ComputerNameDnsHostname, ComputerNameDnsDomain, ComputerNameDnsFullyQualified, ComputerNamePhysicalNetBIOS, ComputerNamePhysicalDnsHostname, ComputerNamePhysicalDnsDomain, ComputerNamePhysicalDnsFullyQualified, ComputerNameMax}; /* sysinfoapi.h:332:14, sysinfoapi.h:332:14, sysinfoapi.h:332:14 */
#[cfg(feature="winapi_desktop")] pub const SCEX2_ALT_NETBIOS_NAME: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* sysinfoapi.h:618:9, sysinfoapi.h:618:9, sysinfoapi.h:618:9 */