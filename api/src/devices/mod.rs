extern crate serde;

pub mod models {
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Device {
        pub id: String,
        pub vid: String,
        pub lid: String,
        pub name: String,
        #[serde(rename="type")]
        pub device_type: String,
        pub tags: Vec<String>,
        pub serial: Option<String>,
        pub group: Option<String>,
        pub notes: Option<String>,
        pub created: i64
    }

    // mocks querying a database for all devices or a specific device by id
    impl Device {
        pub fn all() -> Vec<Device> {
            vec![
                Device {
                    id: String::from("RP_1"),
                    vid: String::from("VT_1"),
                    lid: String::from("LT_1"),
                    name: String::from("Device 1"),
                    device_type: String::from("raspi"),
                    tags: Vec::new(),
                    serial: None,
                    group: None,
                    notes: None,
                    created: 1_000_000_000
                },
                Device {
                    id: String::from("IP_2"),
                    vid: String::from("VT_2"),
                    lid: String::from("LT_2"),
                    name: String::from("Device 2"),
                    device_type: String::from("ip-camera"),
                    tags: vec![String::from("ip-camera"), String::from("hallway")],
                    serial: Some(String::from("IP_2_SERIAL")),
                    group: None,
                    notes: None,
                    created: 1_000_000_000
                },
            ]
        }

        pub fn find(id: String) -> Option<Device> {
            match Self::all().iter().find(|device| device.id == id) {
                Some(device) => Some(Device {
                    id: device.id.to_owned(),
                    vid: device.vid.to_owned(),
                    lid: device.lid.to_owned(),
                    name: device.name.to_owned(),
                    serial: device.serial.to_owned(),
                    device_type: device.device_type.to_owned(),
                    tags: device.tags.to_owned(),
                    group: device.group.to_owned(),
                    notes: device.notes.to_owned(),
                    created: device.created.to_owned()
                }),
                None => None,
            }
        }
    }
}
