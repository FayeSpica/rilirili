#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Sound {
    SoundNone = 0, // no sound

    SoundFocusChange,  // played when the focus changes
    SoundFocusError, // played when the user wants to go somewhere impossible (while the highlight wiggles)
    SoundClick,      // played when the click action runs
    SoundBack,       // played when back action runs
    SoundFocusSidebar, // played when the focus changes to a sidebar item
    SoundClickError, // played when the user clicks a disabled button / a view focused with no click action
    SoundHonk,       // honk
    SoundClickSidebar, // played when a sidebar item is clicked
    SoundTouchUnfocus, // played when touch focus has been interrupted
    SoundTouch,      // played when touch doesn't require it's own click sound
    SoundSliderTick,
    SoundSliderRelease,

    SoundMax, // not an actual sound, just used to count of many sounds there are
}
