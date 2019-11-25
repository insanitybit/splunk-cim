use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Services {
	/// The description of the service.
	description: String,
	/// The endpoint for which the service is installed.
	dest: String,
	/// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_bunit: String,
	/// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_category: String,
	/// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this field when writing add-ons.
	dest_is_expected: bool,
	/// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_priority: String,
	/// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_requires_av: bool,
	/// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_should_timesync: bool,
	/// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_should_update: bool,
	/// The globally unique identifier of the process assigned by the vendor_product.
	process_guid: String,
	/// The numeric identifier of the process assigned by the operating system.
	process_id: String,
	/// The full service name.
	service: String,
	/// The dynamic link library associated with the service.
	service_dll: String,
	/// The file path to the dynamic link library assocatied with the service, such as C:\Windows\System32\comdlg32.dll.
	service_dll_path: String,
	/// The digests of the dynamic link library associated with the service, such as <md5>, <sha1>, etc.
	service_dll_hash: String,
	/// Whether or not the dynamic link library associated with the service has a digitally signed signature.
	service_dll_signature_exists: bool,
	/// Whether or not the dynamic link library associated with the service has had its digitally signed signature verified.
	service_dll_signature_verified: bool,
	/// The executable name of the service.
	service_exec: String,
	/// The digest(s) of the service, such as <md5>, <sha1>, etc.
	service_hash: String,
	/// The unique identifier of the service assigned by the operating system.
	service_id: String,
	/// The friendly service name.
	service_name: String,
	/// The file path of the service, such as C:\WINDOWS\system32\svchost.exe.
	service_path: String,
	/// Whether or not the service has a digitally signed signature.
	service_signature_exists: bool,
	/// Whether or not the service has had its digitally signed signature verified.
	service_signature_verified: bool,
	/// The start mode for the service.
	start_mode: String,
	/// The status of the service. Expected values: critical, started, stopped, warning
	status: String,
	/// This automatically generated field is used to access tags from within data models. Add-on builders do not need to populate it.
	tag: String,
	/// The user account associated with the service.
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