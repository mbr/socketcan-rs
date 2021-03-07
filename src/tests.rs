
use crate::{CANSocket};

#[cfg(feature = "vcan_tests")]
use std::time;

#[cfg(feature = "vcan_tests")]
use crate::ShouldRetry;


#[test]
fn test_nonexistant_device() {
    assert!(CANSocket::open("invalid").is_err());
}

#[test]
#[cfg(feature = "vcan_tests")]
fn vcan0_timeout() {
    let cs = CANSocket::open("vcan0").unwrap();
    cs.set_read_timeout(time::Duration::from_millis(100))
        .unwrap();
    assert!(cs.read_frame().should_retry());
}


#[test]
#[cfg(feature = "vcan_tests")]
fn vcan0_set_error_mask() {
    let cs = CANSocket::open("vcan0").unwrap();
    cs.error_filter_drop_all().unwrap();
    cs.error_filter_accept_all().unwrap();
}

#[test]
#[cfg(feature = "vcan_tests")]
fn vcan0_test_nonblocking() {
    let cs = CANSocket::open("vcan0").unwrap();
    cs.set_nonblocking(true).unwrap();

    // no timeout set, but should return immediately
    assert!(cs.read_frame().should_retry());
}

#[test]
#[cfg(feature = "vcan_tests")]
fn vcan0_test_fd() {
    let cs = CANSocket::open("vcan0").unwrap();
    maybe_enable_fd_frames(&cs);
    for _ in 0..3 {
        let frame = cs.read_frame().unwrap();
        println!("Received frame: {:X}", frame);
        cs.write_frame(&frame).unwrap();
    }
}

#[cfg(all(feature = "can_fd", feature = "vcan_tests"))]
fn maybe_enable_fd_frames(can_socket: &CANSocket) {
    can_socket.set_fd_frames(true).unwrap();
}

#[cfg(all(feature = "vcan_tests", not(feature = "can_fd")))]
fn maybe_enable_fd_frames(_: &CANSocket) {
    /* do nothing */
}
