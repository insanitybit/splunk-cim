use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Processes {
    action: String,
    cpu_load_percent: Option<u8>,  // Number between 0 and 100
	///  	The endpoint for which the process was spawned.
	dest: String,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_bunit: Option<String>,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_category: Option<String>,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this field when writing add-ons.
	dest_is_expected: Option<bool>,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_priority: Option<String>,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_requires_av: Option<bool>,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_should_timesync: Option<bool>,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_should_update: Option<bool>,
	///  	Memory used by the process (in bytes).
	mem_used: Option<u64>,
	///  	The operating system of the resource, such as Microsoft Windows Server 2008r2.
	os: Option<String>,
	///  	The full command String of the parent process.
	parent_process: Option<String>,
	///  	The executable name of the parent process.
	parent_process_exec: Option<String>,
	///  	The numeric identifier of the parent process assigned by the operating system.
	parent_process_id: Option<u64>,
	///  	The globally unique identifier of the parent process assigned by the vendor_product.
	parent_process_guid: Option<String>,
	///  	The friendly name of the parent process, such as notepad.exe.
	parent_process_name: Option<String>,
	///  	The file path of the parent process, such as C:\Windows\System32\notepad.exe.
	parent_process_path: Option<String>,
	///  	The full command String of the spawned process. Such as C:\\WINDOWS\\system32\\cmd.exe \/c \"\"C:\\Program Files\\SplunkUniversalForwarder\\etc\\system\\bin\\powershell.cmd\" --scheme\"".
	process: Option<String>,
	///  	The current working directory used to spawn the process.
	process_current_directory: Option<String>,
	///  	The executable name of the process.
	process_exec: Option<String>,
	///  	The digests of the parent process, such as <md5>, <sha1>, etc.
	process_hash: Option<String>,
	///  	The globally unique identifier of the process assigned by the vendor_product.
	process_guid: Option<String>,
	///  	The numeric identifier of the process assigned by the operating system.
	process_id: u64,
	///  	The Windows integrity level of the process. 	system, high, medium, low, untrusted
	process_integrity_level: Option<String>,
	///  	The friendly name of the process, such as notepad.exe.
	process_name: Option<String>,
	///  	The file path of the process, such as C:\Windows\System32\notepad.exe.
	process_path: Option<String>,
	///  	This automatically generated field is used to access tags from within data models. Add-on builders do not need to populate it.
	tag: Option<String>,
	///  	The user account that spawned the process.
	user: Option<String>,
	///  	The unique identifier of the user account which spawned the process.
	user_id: Option<String>,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	user_bunit: Option<String>,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	user_category: Option<String>,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	user_priority: Option<String>,
	///  	The vendor and product name of the Endpoint solution that reported the event, such as Carbon Black Cb Response. This field can be automatically populated by vendor and product fields in your data."
	vendor_product: Option<String>,
}
