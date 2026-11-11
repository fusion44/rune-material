//! Material 3 state-layer opacities (md.sys.state).

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StateTokens {
    pub hover: f32,
    pub focus: f32,
    pub pressed: f32,
    pub dragged: f32,
}

impl StateTokens {
    pub const fn material() -> StateTokens {
        StateTokens {
            hover: 0.08,
            focus: 0.12,
            pressed: 0.12,
            dragged: 0.16,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_state_opacities() {
        let s = StateTokens::material();
        assert_eq!(s.hover, 0.08);
        assert_eq!(s.focus, 0.12);
        assert_eq!(s.pressed, 0.12);
        assert_eq!(s.dragged, 0.16);
    }
}
