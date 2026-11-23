//! The platform-neutral arithmetic behind iOS pinch delivery.
//!
//! It lives outside the iOS module so its tests compile and run on every host:
//! `ios/events.rs` only feeds it UIKit's values and builds the event.

use gpui::TouchPhase;

/// The raw `UIGestureRecognizerState` values, from UIKit.
const STATE_BEGAN: isize = 1;
const STATE_CHANGED: isize = 2;
const STATE_ENDED: isize = 3;
const STATE_CANCELLED: isize = 4;
const STATE_FAILED: isize = 5;

/// Maps a gesture recognizer state to the phase of the pinch step it carries.
///
/// Returns `None` for states that carry no step (`Possible`, and any state
/// UIKit adds later).
pub(crate) fn pinch_phase(state: isize) -> Option<TouchPhase> {
    match state {
        STATE_BEGAN => Some(TouchPhase::Started),
        STATE_CHANGED => Some(TouchPhase::Moved),
        STATE_ENDED => Some(TouchPhase::Ended),
        STATE_CANCELLED | STATE_FAILED => Some(TouchPhase::Cancelled),
        _ => None,
    }
}

/// Turns UIKit's absolute pinch scale into the relative delta one event carries.
///
/// UIKit reports the scale cumulatively since the gesture began, while
/// `PinchEvent::delta` is the change since the previous event: a result of 0.5
/// is a 50% increase over the scale `last` the previous step ended at.
pub(crate) fn pinch_delta(scale: f64, last: f64) -> f32 {
    (scale / last - 1.) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinch_delta_is_relative_to_the_previous_scale() {
        assert_eq!(pinch_delta(1.0, 1.0), 0.0);
        assert_eq!(pinch_delta(1.5, 1.0), 0.5);
        assert_eq!(pinch_delta(3.0, 1.5), 1.0);
        assert_eq!(pinch_delta(0.5, 1.0), -0.5);
    }

    #[test]
    fn pinch_phase_maps_every_recognizer_state() {
        assert_eq!(pinch_phase(1), Some(TouchPhase::Started));
        assert_eq!(pinch_phase(2), Some(TouchPhase::Moved));
        assert_eq!(pinch_phase(3), Some(TouchPhase::Ended));
        assert_eq!(pinch_phase(4), Some(TouchPhase::Cancelled));
        assert_eq!(pinch_phase(5), Some(TouchPhase::Cancelled));
        assert_eq!(pinch_phase(0), None);
    }
}
