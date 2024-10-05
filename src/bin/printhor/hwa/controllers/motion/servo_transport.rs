//! TODO: This feature is still in incubation

use crate::hwa;
use crate::math::Real;
// use cfg_if;
// #[allow(unused)]
// use embedded_hal_02::Pwm;
// use printhor_hwa_common::InterruptControllerRef;
/// The `TransportTrait` defines a set of asynchronous methods for communicating with an external servo controller.
///
/// # Methods
///
/// * `send_motion(&self, pos: Real, vel: Real, acc: Real)` - Sends the motion information to the servo controller
///
#[allow(unused)]
#[allow(async_fn_in_trait)]
pub trait TransportTrait {
    async fn send_motion(&self, pos: Real, vel: Real, acc: Real);
}



/// The `SPIServoTransport` structure is responsible for implementing the communication protocol 
/// used by the external servo controller.
///
/// # Fields
///
/// * `transport` - A reference to an interrupt-controlled spi hardware instance.
/// 
#[cfg(feature = "with-spi")]
#[derive(Clone)]
pub struct SPIServoTransport {
    /// A reference to an interrupt-controlled spi hardware instance.
    transport: hwa::device::SpiDeviceRef,
}
#[cfg(feature = "with-spi")]
impl SPIServoTransport {
    pub fn new(
        transport: hwa::device::SpiDeviceRef,
    ) -> Self {
        Self { transport }
    }

}
#[cfg(feature = "with-spi")]
impl TransportTrait for SPIServoTransport {
    #[allow(unused)]
    async fn send_motion(&self, pos: Real, vel: Real, acc: Real) {
        let words =  [pos.to_f32().to_le_bytes(),
        vel.to_f32().to_le_bytes(),
        acc.to_f32().to_le_bytes()].concat();

        match self.transport.try_lock() {
            Err(_) => {
                panic!("Could not lock SPI mutex");
            }
            Ok(mut _spi) => {
                _spi.write(&words).await;
            }
        }
    }
}
