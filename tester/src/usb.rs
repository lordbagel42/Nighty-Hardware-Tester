use crate::log;

pub fn find_device(l: &mut log::LogCtx, vendor_id: u16, product_id: u16) -> bool {
    l.enter("find_usb_device");

    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        if (device_desc.vendor_id() == vendor_id)
            && (device_desc.product_id() == product_id)
        {
            l.inf("Found USB device");

            l.leave("find_usb_device");

            return true;
        }
    }

    l.err("USB device not found");

    l.leave("find_usb_device");

    false
}