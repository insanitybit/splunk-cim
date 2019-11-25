use serde::{Serialize, Deserialize};

type Timestamp = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Filesystem {
	///  	The action performed on the resource. Expected values: acl_modified, created, deleted, modified, read
	action: String,
	///  	The endpoint pertaining to the filesystem activity.
	dest: String,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_bunit: String,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_category: String,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_priority: String,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_requires_av: bool,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_should_timesync: bool,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	dest_should_update: bool,
	///  	The time that the file (the object of the event) was accessed.
	file_access_time: Timestamp,
	///  	The time that the file (the object of the event) was created.
	file_create_time: Timestamp,
	///  	A cryptographic identifier assigned to the file object affected by the event.
	file_hash: String,
	///  	The time that the file (the object of the event) was altered.
	file_modify_time: Timestamp,
	///  	The name of the file, such as notepad.exe.
	file_name: String,
	///  	The path of the file, such as C:\Windows\System32\notepad.exe.
	file_path: String,
	///  	Access controls associated with the file affected by the event.
	file_acl: String,
	///  	The size of the file that is the object of the event, in kilobytes.
	file_size: String,
	///  	The globally unique identifier of the process assigned by the vendor_product.
	process_guid: String,
	///  	The numeric identifier of the process assigned by the operating system.
	process_id: String,
	///  	This automatically generated field is used to access tags from within data models. Add-on builders do not need to populate it.
	tag: String,
	///  	The user account associated with the filesystem access.
	user: String,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	user_bunit: String,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	user_category: String,
	///  	This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
	user_priority: String,
	///  	The vendor and product name of the Endpoint solution that reported the event, such as Carbon Black Cb Response. This field can be automatically populated by vendor and product fields in your data.
	vendor_product: String,
}