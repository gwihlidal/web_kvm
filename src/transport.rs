use crate::{config::Config, packets};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::thread;
use std::time;

fn flush_command() {
    // Wait 200ms in between commands to ensure that correct results
    // are received, and in the right order.
    let pause_amount = time::Duration::from_millis(200);
    thread::sleep(pause_amount);
}

fn write_command(config: &Config, packet: &[u8]) {
    let socket: SocketAddr = config
        .kvm_address
        .parse()
        .expect("Invalid KVM address socket!");
    if let Ok(mut stream) =
        TcpStream::connect_timeout(&socket, time::Duration::new(config.connect_timeout, 0))
    {
        let write_result = stream.write(&packet);
        match write_result {
            Ok(_) => {
                flush_command();
            }
            Err(err) => {
                error!("Error writing to KVM stream: {:#?}", err);
            }
        }
    } else {
        error!("Failed to connect to KVM at {}!", config.kvm_address);
    }
}

pub fn led_mode(config: &Config, mode: usize) {
    write_command(
        config,
        match mode {
            1 => &packets::LED_TIMEOUT_10SEC,
            2 => &packets::LED_TIMEOUT_30SEC,
            _ => &packets::LED_TIMEOUT_NEVER,
        },
    );
}

pub fn beeping_mode(config: &Config, mode: usize) {
    write_command(
        config,
        match mode {
            0 => &packets::BEEPING_DISABLE,
            _ => &packets::BEEPING_ENABLE,
        },
    );
}

pub fn detect_mode(config: &Config, mode: usize) {
    write_command(
        config,
        match mode {
            0 => &packets::AUTO_DETECT_DISABLE,
            _ => &packets::AUTO_DETECT_ENABLE,
        },
    );
}

pub fn switch_to_port(config: &Config, port: usize) {
    write_command(
        config,
        match port {
            1 => &packets::SWITCH_TO_PC1,
            2 => &packets::SWITCH_TO_PC2,
            3 => &packets::SWITCH_TO_PC3,
            4 => &packets::SWITCH_TO_PC4,
            5 => &packets::SWITCH_TO_PC5,
            6 => &packets::SWITCH_TO_PC6,
            7 => &packets::SWITCH_TO_PC7,
            8 => &packets::SWITCH_TO_PC8,
            _ => &packets::SWITCH_TO_PC1, // Error
        },
    );
}

pub fn read_current_port(config: &Config) -> u8 {
    let mut current_port = 0; // Denotes an error
    let socket: SocketAddr = config
        .kvm_address
        .parse()
        .expect("Invalid KVM address socket!");
    if let Ok(mut stream) =
        TcpStream::connect_timeout(&socket, time::Duration::new(config.connect_timeout, 0))
    {
        let write_result = stream.write(&packets::READ_CURRENT_PORT);
        match write_result {
            Ok(_) => {
                let mut read_port_buffer = [0u8; 6];
                let result = stream.read(&mut read_port_buffer);
                match result {
                    Ok(byte_count) => {
                        if byte_count == 6 {
                            current_port = read_port_buffer[4] + 1; // go from zero to one based
                        } else {
                            error!("Failed to receive correct response. Expected 6 bytes, received {} bytes.", byte_count);
                        }
                    }
                    Err(err) => {
                        error!("Error reading from KVM stream: {:#?}", err);
                    }
                }
            }
            Err(err) => {
                error!("Error writing to KVM stream: {:#?}", err);
            }
        }
    } else {
        error!("Failed to connect to KVM at {}!", config.kvm_address);
    }

    current_port
}
