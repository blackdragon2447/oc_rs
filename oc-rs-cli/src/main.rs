use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

use oc_rs::device_bus::RPCBus;
use oc_rs::rpc_device::{FileImportExportCard, RPCDeviceDescriptor, RedstoneInterface};
use oc_rs::util::Side;

fn main() {
    let bus_raw = RPCBus::init("/dev/hvc0").unwrap();
    let bus = Mutex::new(bus_raw);
    let args: Vec<String> = env::args().collect();

    // println!("Argh... cant seem to wrap my head around this all...");

    match args.get(1).unwrap_or(&String::new()).as_str() {
        "methods" => print_methods(bus),
        "devices" => print_devices(bus),
        "device" => {
            if let Some(a) = args.get(2) {
                match args.get(3).unwrap_or(&String::new()).as_str() {
                    "methods" => {
                        let mut bus = bus.lock().unwrap();
                        let device = bus.find(a).unwrap();

                        println!("-------------------------------------------------------");

                        for m in bus.methods(device).unwrap() {
                            println!("{m}");
                            println!("-------------------------------------------------------");
                        }
                    }
                    &_ => match a.as_str() {
                        "redstone" => {
                            let device = bus.lock().unwrap().find("redstone").unwrap();
                            let mut device = RedstoneInterface { device, bus };
                            let side = match args.get(4).unwrap().as_str() {
                                "up" => Side::up,
                                "down" => Side::down,
                                "north" => Side::north,
                                "east" => Side::east,
                                "south" => Side::south,
                                "west" => Side::west,
                                &_ => Side::north,
                            };
                            match args.get(3).unwrap_or(&String::new()).as_str() {
                                "getIn" => device.get_redstone_input(side).unwrap(),
                                "getOut" => device.get_redstone_output(side).unwrap(),
                                "on" => {
                                    device.set_redstone_output(side, 15).unwrap();
                                    0
                                }
                                "off" => {
                                    device.set_redstone_output(side, 0).unwrap();
                                    0
                                }
                                &_ => {
                                    println!("missing arguments");
                                    0
                                }
                            };
                        }
                        &_ => (),
                    },
                }
            } else {
                println!("missing device specifier");
            }
        }
        "import-file" => {
            let device = bus.lock().unwrap().find("file_import_export").unwrap();
            let mut device = FileImportExportCard { device, bus };

            device.reset().unwrap();
            device.request_import_file().unwrap();

            let info = device.begin_import_file().unwrap();
            let out_name = if let Some(s) = args.get(2) {
                s.clone()
            } else {
                info.get_name()
            };

            if Path::new(format!("./{}", &out_name).as_str()).exists() {
                eprintln!("That file already exists");
                return;
            }

            let data = device.read_import_file().unwrap();
            let mut file = OpenOptions::new()
                .write(true)
                .open(format!("./{}", &out_name))
                .unwrap();
            file.write_all(&data).unwrap();
        }
        &_ => {
            // let device = bus.lock().unwrap().find("redstone").unwrap();
            // let mut device = RedstoneInterface {
            //     device,
            //     bus,
            // };
            // device.set_redstone_output("up".to_string(), 15).unwrap();
            // thread::sleep(Duration::from_secs(2));
            // device.set_redstone_output("up".to_string(), 0).unwrap();

            // let device = bus.lock().unwrap().find("oc2:sound_card").unwrap();
            // let mut device = SoundCard {
            //     device,
            //     bus,
            // };
            // let sound = device.find_sound("block.anvil.use".to_string()).unwrap().get(0).unwrap().clone();
            // device.play_sound(sound).unwrap();

            // let device = bus.lock().unwrap().find("oc2:computer").unwrap();
            // let mut device = Computer {
            //     device,
            //     bus,
            // };
            // println!("{:?}", device.get_item_slot_count().unwrap());
            // println!("{:?}", device.get_item_stack_in_slot(0).unwrap());
            // println!("{}", device.get_item_slot_limit(0).unwrap());
            // println!("{}", device.get_energy_stored().unwrap());
            // println!("{}", device.get_max_energy_stored().unwrap());
            // println!("{}", device.can_extract_energy().unwrap());
            // println!("{}", device.can_receive_energy().unwrap());

            // let device = bus
            //     .lock()
            //     .unwrap()
            //     .find("oc2:file_import_export_card")
            //     .unwrap();
            // let mut device = FileImportExportCard { device, bus };
            // println!("{:?}", device.begin_import_file().unwrap());
        }
    }
}

fn print_devices(bus: Mutex<RPCBus>) {
    let mut bus = bus.lock().unwrap();
    for d in bus.list().unwrap() {
        println!("{d}");
    }
}

fn print_methods(bus: Mutex<RPCBus>) {
    let mut bus = bus.lock().unwrap();
    for d in bus.list().unwrap() {
        for m in bus.methods(d.device_id).unwrap() {
            println!("{m}")
        }
    }
}
