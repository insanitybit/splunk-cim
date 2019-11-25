use serde::{Serialize, Deserialize};

/// https://docs.splunk.com/Documentation/CIM/4.14.0/User/Endpoint#Registry
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Registry {
    /// The action performed on the resource. Expected values: created, deleted, modified, read
    action: String,
    /// The endpoint pertaining to the registry events.
    dest: String,
    /// This automatically generated field is used to access tags from within data models. Add-on builders do not need to populate it.
    dest_bunit: String,
    /// This automatically generated field is used to access tags from within data models. Add-on builders do not need to populate it.
    dest_category: String,
    /// This automatically generated field is used to access tags from within data models. Add-on builders do not need to populate it.
    dest_priority: String,
    /// This automatically generated field is used to access tags from within data models. Add-on builders do not need to populate it.
    dest_requires_av: bool,
    /// This automatically generated field is used to access tags from within data models. Add-on builders do not need to populate it.
    dest_should_timesync: bool,
    /// This automatically generated field is used to access tags from within data models. Add-on builders do not need to populate it.
    dest_should_update: bool,
    /// The globally unique identifier of the process assigned by the vendor_product.
    process_guid: String,
    /// The numeric identifier of the process assigned by the operating system.
    process_id: String,
    /// The logical grouping of registry keys, subkeys, and values. 	HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE\\SAM, HKEY_LOCAL_MACHINE\\Security, HKEY_LOCAL_MACHINE\\Software, HKEY_LOCAL_MACHINE\\System, HKEY_USERS\\.DEFAULT
    registry_hive: String,
    /// The path to the registry value, such as \win\directory\directory2\{676235CD-B656-42D5-B737-49856E97D072}\PrinterDriverData.
    registry_path: String,
    /// The name of the registry key, such as PrinterDriverData.
    registry_key_name: String,
    /// The unaltered registry value.
    registry_value_data: String,
    /// The name of the registry value.
    registry_value_name: String,
    /// The textual representation of registry_value_data (if applicable).
    registry_value_text: String,
    /// The type of the registry value. Expected values: REG_BINARY, REG_DWORD, REG_DWORD_LITTLE_ENDIAN, REG_DWORD_BIG_ENDIAN, REG_EXPAND_SZ, REG_LINK, REG_MULTI_SZ, REG_NONE, REG_QWORD, REG_QWORD_LITTLE_ENDIAN, REG_SZ
    registry_value_type: String,
    /// The outcome of the registry action. 	failure, success
    status: String,
    /// This automatically generated field is used to access tags from within data models. Add-on builders do not need to populate it.
    tag: String,
    /// The user account associated with the registry access.
    user: String,
    /// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
    user_bunit: String,
    /// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
    user_category: String,
    /// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
    user_priority: String,
    /// The vendor and product name of the Endpoint solution that reported the event, such as Carbon Black Cb Response. This field can be automatically populated by vendor and product fields in your data.
    vendor_product: String,
}
