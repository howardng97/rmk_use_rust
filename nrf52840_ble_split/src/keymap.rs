use rmk::action::KeyAction;
use rmk::keycode::ModifierCombination;
use rmk::{a, k, layer, mo,wm};
pub(crate) const COL: usize = 20;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 3;
#[rustfmt::skip]
pub fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(Tab), k!(Q), k!(W), k!(E), k!(R), k!(T), a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No), k!(Y), k!(U), k!(I), k!(O), k!(P), k!(Backspace)],
            [k!(Escape), k!(A), k!(S), k!(D), k!(F), k!(G),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No), k!(H), k!(J), k!(K), k!(L), k!(Semicolon), k!(Quote)],
            [k!(LShift), k!(Z), k!(X), k!(C), k!(V), k!(B),k!(KbVolumeUp),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),k!(KbVolumeDown), k!(N), k!(M), k!(Comma), k!(Dot), k!(Slash), k!(RShift)],
            [a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No), k!(LGui), k!(LCtrl), k!(Space), k!(Space), mo!(1), k!(RAlt), a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No)]
        ]),
        layer!([
            [k!(Grave), k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5), a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No), k!(Kc6), k!(Kc7), k!(Kc8), k!(Kc9), k!(Kc0), k!(Backslash)],
            [k!(Minus), wm!(Minus,ModifierCombination::new_from(false,false,false,true,false)), k!(MouseLeft), k!(MouseUp), k!(MouseDown), k!(MouseRight),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No), k!(Left), k!(Down), k!(UP), k!(Right), wm!(Equal,ModifierCombination::new_from(false,false,false,true,false)), k!(Equal)],
            [a!(Transparent), k!(LeftBracket), k!(RightBracket), wm!(LeftBracket,ModifierCombination::new_from(false,false,false,true,false)), wm!(RightBracket,ModifierCombination::new_from(false,false,false,true,false)), k!(MouseBtn2),k!(Calculator),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),k!(PrintScreen), k!(MouseBtn1), k!(MouseWheelDown), k!(MouseWheelUp), a!(No), a!(No), a!(No)],
            [a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No), mo!(2), a!(Transparent), k!(Enter), a!(No), a!(No), a!(No), a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No)]
        ]),
        layer!([
            [a!(No), k!(F1), k!(F2), k!(F3), k!(F4), k!(F5), a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No), k!(F6), k!(F7), k!(F8), k!(F9), k!(F10), a!(No)],
            [a!(No), k!(F11), a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No), a!(No),a!(No),a!(No),a!(No), k!(F12), a!(No)],
            [a!(No), k!(User0), k!(X), k!(C), k!(V), k!(B),k!(KbVolumeUp),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),k!(KbVolumeDown), k!(N), k!(M), k!(Comma), k!(Dot), k!(Slash), k!(RShift)],
            [a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No), a!(No),a!(No),a!(No),a!(No),a!(No),a!(No), a!(No),a!(No),a!(No),a!(No),a!(No),a!(No),a!(No)]
        ]),
    ]
}
