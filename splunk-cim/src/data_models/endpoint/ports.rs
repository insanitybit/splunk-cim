use serde::{Serialize, Deserialize};

type Timestamp = String;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Ports {
    /// The time at which the network port started listening on the endpoint.
    creation_time: Timestamp,
    /// The endpoint on which the port is listening.
    dest: String,
    /// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
    dest_bunit: String,
    /// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
    dest_category: String,
    /// Network port listening on the endpoint, such as 53.
    dest_port: u64,
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
    /// The "remote" system connected to the listening port (if applicable).
    src: String,
    /// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
    src_category: String,
    /// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
    src_priority: String,
    /// The "remote" port connected to the listening port (if applicable).
    src_port: u64,
    /// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
    src_requires_av: bool,
    /// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
    src_should_timesync: bool,
    /// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
    src_should_update: bool,
    /// The status of the listening port, such as established, listening, etc.
    state: String,
    /// This automatically generated field is used to access tags from within data models. Add-on builders do not need to populate it.
    tag: String,
    /// The network transport protocol associated with the listening port, such as tcp, udp, etc."
    transport: String,
    /// Calculated as transport/dest_port, such as tcp/53.
    transport_dest_port: String,
    /// The user account associated with the listening port.
    user: String,
    /// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
    user_bunit: String,
    /// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
    user_category: String,
    /// This field is automatically provided by asset and identity correlation features of applications like Splunk Enterprise Security. Do not define extractions for this fields when writing add-ons.
    user_priority: String,
}