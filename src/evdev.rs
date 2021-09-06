#![cfg(target_os = "linux")]

use std::io;

use evdev_rs;
use evdev_rs::DeviceWrapper;

pub struct Device {
    pub udevice: evdev_rs::uinput::UInputDevice,
}

impl Device {
    pub fn create() -> Result<Device, io::Error> {
        let device = match evdev_rs::UninitDevice::new() {
            Some(d) => d,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unable to initialize an empty device",
                ))
            }
        };
        // TODO: Un-hardcode this later.
        device.set_name("Sony Corp. FeliCa S320 [PaSoRi]");
        device.enable(&evdev_rs::enums::EventType::EV_MSC)?;
        device.enable(&evdev_rs::enums::EventCode::EV_MSC(
            evdev_rs::enums::EV_MSC::MSC_RAW,
        ))?;

        let udevice = evdev_rs::uinput::UInputDevice::create_from_device(&device)?;

        Ok(Device { udevice })
    }

    pub fn write(&self, value: i32) -> Result<(), io::Error> {
        // Irrelevant for emitting events; the actual timing will be
        // assigned by the code we're calling.
        let null = evdev_rs::TimeVal::new(0, 0);

        let event = evdev_rs::InputEvent {
            time: null,
            event_code: evdev_rs::enums::EventCode::EV_MSC(evdev_rs::enums::EV_MSC::MSC_RAW),
            value,
        };
        self.udevice.write_event(&event)?;

        // Must send a syn event so the client will actually read this
        let syn = evdev_rs::InputEvent {
            time: null,
            event_code: evdev_rs::enums::EventCode::EV_SYN(evdev_rs::enums::EV_SYN::SYN_REPORT),
            value: 0,
        };
        self.udevice.write_event(&syn)?;

        Ok(())
    }
}
