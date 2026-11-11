//! Material 3 motion tokens (md.sys.motion): durations and CSS cubic-bezier easings.

use std::time::Duration;

use rune::Easing;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionTokens {
    pub short1: Duration,
    pub short2: Duration,
    pub short3: Duration,
    pub short4: Duration,
    pub medium1: Duration,
    pub medium2: Duration,
    pub medium3: Duration,
    pub medium4: Duration,
    pub long1: Duration,
    pub long2: Duration,
    pub long3: Duration,
    pub long4: Duration,
    pub extra_long1: Duration,
    pub extra_long2: Duration,
    pub extra_long3: Duration,
    pub extra_long4: Duration,
    pub emphasized: Easing,
    pub emphasized_accelerate: Easing,
    pub emphasized_decelerate: Easing,
    pub standard: Easing,
    pub standard_accelerate: Easing,
    pub standard_decelerate: Easing,
    pub legacy: Easing,
    pub legacy_accelerate: Easing,
    pub legacy_decelerate: Easing,
    pub linear: Easing,
}

impl MotionTokens {
    pub const fn material() -> MotionTokens {
        MotionTokens {
            short1: Duration::from_millis(50),
            short2: Duration::from_millis(100),
            short3: Duration::from_millis(150),
            short4: Duration::from_millis(200),
            medium1: Duration::from_millis(250),
            medium2: Duration::from_millis(300),
            medium3: Duration::from_millis(350),
            medium4: Duration::from_millis(400),
            long1: Duration::from_millis(450),
            long2: Duration::from_millis(500),
            long3: Duration::from_millis(550),
            long4: Duration::from_millis(600),
            extra_long1: Duration::from_millis(700),
            extra_long2: Duration::from_millis(800),
            extra_long3: Duration::from_millis(900),
            extra_long4: Duration::from_millis(1000),
            emphasized: Easing::cubic(0.2, 0.0, 0.0, 1.0),
            emphasized_accelerate: Easing::cubic(0.3, 0.0, 0.8, 0.15),
            emphasized_decelerate: Easing::cubic(0.05, 0.7, 0.1, 1.0),
            standard: Easing::cubic(0.2, 0.0, 0.0, 1.0),
            standard_accelerate: Easing::cubic(0.3, 0.0, 1.0, 1.0),
            standard_decelerate: Easing::cubic(0.0, 0.0, 0.0, 1.0),
            legacy: Easing::cubic(0.4, 0.0, 0.2, 1.0),
            legacy_accelerate: Easing::cubic(0.4, 0.0, 1.0, 1.0),
            legacy_decelerate: Easing::cubic(0.0, 0.0, 0.2, 1.0),
            linear: Easing::Linear,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_motion_tokens() {
        let m = MotionTokens::material();
        assert_eq!(m.short1, Duration::from_millis(50));
        assert_eq!(m.short2, Duration::from_millis(100));
        assert_eq!(m.short3, Duration::from_millis(150));
        assert_eq!(m.short4, Duration::from_millis(200));
        assert_eq!(m.medium1, Duration::from_millis(250));
        assert_eq!(m.medium2, Duration::from_millis(300));
        assert_eq!(m.medium3, Duration::from_millis(350));
        assert_eq!(m.medium4, Duration::from_millis(400));
        assert_eq!(m.long1, Duration::from_millis(450));
        assert_eq!(m.long2, Duration::from_millis(500));
        assert_eq!(m.long3, Duration::from_millis(550));
        assert_eq!(m.long4, Duration::from_millis(600));
        assert_eq!(m.extra_long1, Duration::from_millis(700));
        assert_eq!(m.extra_long2, Duration::from_millis(800));
        assert_eq!(m.extra_long3, Duration::from_millis(900));
        assert_eq!(m.extra_long4, Duration::from_millis(1000));
    }

    #[test]
    fn cubic_curves_match_material() {
        let m = MotionTokens::material();
        assert_eq!(
            m.emphasized.eval(0.5),
            Easing::cubic(0.2, 0.0, 0.0, 1.0).eval(0.5)
        );
        assert_eq!(m.emphasized, Easing::cubic(0.2, 0.0, 0.0, 1.0));
        assert_eq!(m.standard_decelerate, Easing::cubic(0.0, 0.0, 0.0, 1.0));
    }
}
