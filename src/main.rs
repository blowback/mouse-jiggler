#![allow(dead_code)]

use esp32_nimble::{
    enums::*, hid::*, utilities::mutex::Mutex, BLECharacteristic, BLEDevice, BLEHIDDevice,
    BLEServer,
};
use esp_idf_sys as _;
use std::sync::Arc;

const MOUSE_ID: u8 = 0x01;

const HID_REPORT_DESCRIPTOR: &[u8] = hid!(
    (USAGE_PAGE, 0x01), // USAGE_PAGE (Generic Desktop Ctrls)
    (USAGE, 0x02),      // USAGE (Mouse)
    (COLLECTION, 0x01), // COLLECTION (Application)
    (REPORT_ID, MOUSE_ID),
    (USAGE, 0x01),      //   USAGE (Pointer)
    (COLLECTION, 0x00), //   COLLECTION (Physical)
    // ------------------------------------------------- Buttons (Left, Right, Middle, Back, Forward)
    (USAGE_PAGE, 0x09),      //     USAGE_PAGE (Buttons)
    (USAGE_MINIMUM, 0x01),   //     USAGE_MINIMUM (Button 1)
    (USAGE_MAXIMUM, 0x05),   //     USAGE_MAXIMUM (Button 5)
    (LOGICAL_MINIMUM, 0x00), //     LOGICAL_MINIMUM (0)
    (LOGICAL_MAXIMUM, 0x01), //     LOGICAL_MAXIMUM (1)
    (REPORT_SIZE, 0x01),     //     REPORT_SIZE (1) (1 bit)
    (REPORT_COUNT, 0x05),    //     REPORT_COUNT (5) (5 times)
    (HIDINPUT, 0x02),        //     INPUT (Data, Variable, Absolute)
    // ------------------------------------------------- Padding
    (REPORT_SIZE, 0x03),  //     REPORT_SIZE (3) (3 bits)
    (REPORT_COUNT, 0x01), //     REPORT_COUNT (1) (1 time)
    (HIDINPUT, 0x03),     //     INPUT (Constant, Variable, Absolute) ;3 bit padding
    // ------------------------------------------------- X/Y position, Wheel
    (USAGE_PAGE, 0x01),      //    USAGE_PAGE (Generic Desktop)
    (USAGE, 0x30),           //    USAGE (X)
    (USAGE, 0x31),           //    USAGE (Y)
    (USAGE, 0x38),           //    USAGE (Wheel)
    (LOGICAL_MINIMUM, 0x81), //    LOGICAL_MINIMUM (-127)
    (LOGICAL_MAXIMUM, 0x7f), //    LOGICAL_MAXIMUM (127)
    (REPORT_SIZE, 0x08),     //    REPORT_SIZE (8)
    (REPORT_COUNT, 0x03),    //    REPORT_COUNT (3)
    (HIDINPUT, 0x06),        //    INPUT (Data, Variable, Relative) ;3 bytes (X,Y,Wheel)
    // ------------------------------------------------- Horizontal wheel
    (USAGE_PAGE, 0x0c),      //    USAGE PAGE (Consumer Devices)
    (USAGE, 0x38, 0x02),     //    USAGE (AC Pan)
    (LOGICAL_MINIMUM, 0x81), //    LOGICAL_MINIMUM (-127)
    (LOGICAL_MAXIMUM, 0x7f), //    LOGICAL_MAXIMUM (127)
    (REPORT_SIZE, 0x08),     //    REPORT_SIZE (8)
    (REPORT_COUNT, 0x01),    //    REPORT_COUNT (1)
    (HIDINPUT, 0x06),        //    INPUT (Data, Var, Rel)
    (END_COLLECTION),        //   END_COLLECTION
    (END_COLLECTION),        // END_COLLECTION
);

#[repr(packed)]
struct MouseReport {
    buttons: u8,
    x: i8,
    y: i8,
    wheel: i8,
    horizontal_wheel: i8,
}

struct Mouse {
    server: &'static mut BLEServer,
    input_mouse: Arc<Mutex<BLECharacteristic>>,
    mouse_report: MouseReport,
}

impl Mouse {
    fn new() -> Self {
        let device = BLEDevice::take();
        device
            .security()
            .set_auth(AuthReq::all())
            .set_io_cap(SecurityIOCap::NoInputNoOutput);

        let server = device.get_server();
        let mut hid = BLEHIDDevice::new(server);

        let input_mouse = hid.input_report(MOUSE_ID);

        hid.manufacturer("ant.org");
        hid.pnp(0x02, 0x05ac, 0x820a, 0x0210);
        hid.hid_info(0x00, 0x02);

        hid.report_map(HID_REPORT_DESCRIPTOR);

        hid.set_battery_level(100);

        let ble_advertising = device.get_advertising();
        ble_advertising
            .name("BTMouse")
            .appearance(0x03C2) // 0x03c1 is KEYBOARD
            .add_service_uuid(hid.hid_service().lock().uuid())
            .scan_response(false);

        ble_advertising.start().unwrap();

        Self {
            server,
            input_mouse,
            mouse_report: MouseReport {
                buttons: 0,
                x: 0,
                y: 0,
                wheel: 0,
                horizontal_wheel: 0,
            },
        }
    }

    fn connected(&self) -> bool {
        self.server.connected_count() > 0
    }

    fn click(&mut self, b: u8) {
        self.mouse_report.buttons = b;
        self.send_report(&self.mouse_report);
    }

    fn move_mouse(&mut self, x: i8, y: i8, wheel: i8, horizontal_wheel: i8) {
        self.mouse_report.x = x;
        self.mouse_report.y = y;
        self.mouse_report.wheel = wheel;
        self.mouse_report.horizontal_wheel = horizontal_wheel;
        self.send_report(&self.mouse_report);
    }

    fn send_report(&self, mouse: &MouseReport) {
        self.input_mouse.lock().set_from(mouse).notify();
        esp_idf_hal::delay::Ets::delay_ms(7);
    }
}

fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    // WDT OFF
    // unsafe {
    //     esp_idf_sys::esp_task_wdt_delete(esp_idf_sys::xTaskGetIdleTaskHandleForCPU(
    //         esp_idf_hal::cpu::core() as u32,
    //     ));
    // };

    let mut mouse = Mouse::new();

    loop {
        if mouse.connected() {
            ::log::info!("Sending mousage...");
            mouse.move_mouse(10, 10, 0, 0);
            esp_idf_hal::delay::FreeRtos::delay_ms(100);
            mouse.move_mouse(-10, -10, 0, 0);
            esp_idf_hal::delay::FreeRtos::delay_ms(100);
            mouse.move_mouse(10, 10, 0, 0);
            esp_idf_hal::delay::FreeRtos::delay_ms(100);
            mouse.move_mouse(-10, -10, 0, 0);
            esp_idf_hal::delay::FreeRtos::delay_ms(100);
        }
        esp_idf_hal::delay::FreeRtos::delay_ms(60 * 1000);
    }
}
