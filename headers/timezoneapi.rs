#[cfg(feature="winapi_app")] #[repr(C)] pub struct TIME_ZONE_INFORMATION { Bias: ::winnt::LONG, StandardName: *mut [::winnt::WCHAR; 32], StandardDate: ::minwinbase::SYSTEMTIME, StandardBias: ::winnt::LONG, DaylightName: *mut [::winnt::WCHAR; 32], DaylightDate: ::minwinbase::SYSTEMTIME, DaylightBias: ::winnt::LONG } /* timezoneapi.h:47:16, timezoneapi.h:47:16, timezoneapi.h:47:16 */
#[cfg(feature="winapi_app")] pub type PTIME_ZONE_INFORMATION = *mut ::timezoneapi::TIME_ZONE_INFORMATION; /* timezoneapi.h:55:27, timezoneapi.h:55:27, timezoneapi.h:55:27 */
#[cfg(feature="winapi_app")] pub type LPTIME_ZONE_INFORMATION = *mut ::timezoneapi::TIME_ZONE_INFORMATION; /* timezoneapi.h:55:52, timezoneapi.h:55:52, timezoneapi.h:55:52 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct DYNAMIC_TIME_ZONE_INFORMATION { Bias: ::winnt::LONG, StandardName: *mut [::winnt::WCHAR; 32], StandardDate: ::minwinbase::SYSTEMTIME, StandardBias: ::winnt::LONG, DaylightName: *mut [::winnt::WCHAR; 32], DaylightDate: ::minwinbase::SYSTEMTIME, DaylightBias: ::winnt::LONG, TimeZoneKeyName: *mut [::winnt::WCHAR; 128], DynamicDaylightTimeDisabled: ::winnt::BOOLEAN } /* timezoneapi.h:57:16, timezoneapi.h:57:16, timezoneapi.h:57:16 */
#[cfg(feature="winapi_app")] pub type PDYNAMIC_TIME_ZONE_INFORMATION = *mut ::timezoneapi::DYNAMIC_TIME_ZONE_INFORMATION; /* timezoneapi.h:67:35, timezoneapi.h:67:35, timezoneapi.h:67:35 */