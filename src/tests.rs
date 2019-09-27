
use crate::{CANSocket};

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
    cs.set_nonblocking(true);

    // no timeout set, but should return immediately
    assert!(cs.read_frame().should_retry());
}
