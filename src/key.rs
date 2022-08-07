/// Key on a Keyboard.
///
/// Named after keys on Ardaku 64-key keyboard with 4 "levels":
/// 
///  0. ASCII Keys
///  1. Function (Fn) Keys
///  2. System (Sys) Keys
///  3. Numpad / Calculator (Num) Keys
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Key {
    // Row 1
    /// ```text
    /// ` ~
    /// ```
    Grave = 0,
    /// ```text
    /// 1 !
    /// ```
    Digit1 = 1,
    /// ```text
    /// 2 @
    /// ```
    Digit2 = 2,
    /// ```text
    /// 3 #
    /// ```
    Digit3 = 3,
    /// ```text
    /// 4 $
    /// ```
    Digit4 = 4,
    /// ```text
    /// 5 %
    /// ```
    Digit5 = 5,
    /// ```text
    /// 6 ^
    /// ```
    Digit6 = 6,
    /// ```text
    /// 7 &
    /// ```
    Digit7 = 7,
    /// ```text
    /// 8 *
    /// ```
    Digit8 = 8,
    /// ```text
    /// 9 (
    /// ```
    Digit9 = 9,
    /// ```text
    /// 0 )
    /// ```
    Digit0 = 10,
    /// ```text
    /// - _
    /// ```
    Minus = 11,
    /// ```text
    /// = +
    /// ```
    Equals = 12,
    /// ```text
    /// \ |
    /// ```
    Backslash = 13,

    // Row 2
    /// ```text
    /// Tab Untab
    /// ```
    Tab = 14,
    /// ```text
    /// q Q
    /// ```
    Q = 15,
    /// ```text
    /// w W
    /// ```
    W = 16,
    /// ```text
    /// e E
    /// ```
    E = 17,
    /// ```text
    /// r R
    /// ```
    R = 18,
    /// ```text
    /// t T
    /// ```
    T = 19,
    /// ```text
    /// y Y
    /// ```
    Y = 20,
    /// ```text
    /// u U
    /// ```
    U = 21,
    /// ```text
    /// i I
    /// ```
    I = 22,
    /// ```text
    /// o O
    /// ```
    O = 23,
    /// ```text
    /// p P
    /// ```
    P = 24,
    /// ```text
    /// [ {
    /// ```
    LeftBracket = 25,
    /// ```text
    /// ] }
    /// ```
    RightBracket = 26,

    // Row 3
    /// ```text
    /// Escape Menu
    /// ```
    Escape = 27,
    /// ```text
    /// a A
    /// ```
    A = 28,
    /// ```text
    /// s S
    /// ```
    S = 29,
    /// ```text
    /// d D
    /// ```
    D = 30,
    /// ```text
    /// f F
    /// ```
    F = 31,
    /// ```text
    /// g G
    /// ```
    G = 32,
    /// ```text
    /// h H
    /// ```
    H = 33,
    /// ```text
    /// j J
    /// ```
    J = 34,
    /// ```text
    /// k K
    /// ```
    K = 35,
    /// ```text
    /// l L
    /// ```
    L = 36,
    /// ```text
    /// ; :
    /// ```
    Semicolon = 37,
    /// ```text
    /// ' "
    /// ```
    Apostrophe = 38,
    /// ```text
    /// Enter Amend
    /// ```
    Enter = 39,

    // Row 4
    /// ```text
    /// Shift ShiftLk
    /// ```
    LeftShift = 40,
    /// ```text
    /// z Z
    /// ```
    Z = 41,
    /// ```text
    /// x X
    /// ```
    X = 42,
    /// ```text
    /// c C
    /// ```
    C = 43,
    /// ```text
    /// v V
    /// ```
    V = 44,
    /// ```text
    /// b B
    /// ```
    B = 45,
    /// ```text
    /// n N
    /// ```
    N = 46,
    /// ```text
    /// m M
    /// ```
    M = 47,
    /// ```text
    /// , <
    /// ```
    Comma = 48,
    /// ```text
    /// . >
    /// ```
    Period = 49,
    /// ```text
    /// / ?
    /// ```
    Slash = 50,
    /// ```text
    /// Up SelectUp
    /// ```
    Up = 51,
    /// ```text
    /// Shift ShiftLk
    /// ```
    RightShift = 52,

    // Row 5
    /// ```text
    /// LApp/LCtrl/LCmd LPrg/LOption/LAlt
    /// ```
    LeftApp = 53,
    /// ```text
    /// LPrg/LOption/LAlt LApp/LCtrl/LCmd
    /// ```
    LeftPrg = 54,
    /// ```text
    /// LEnv/LSystem LMeta
    /// ```
    LeftSys = 55,
    /// ```text
    /// Erase/Delete/Backspace Remove/Del
    /// ```
    Erase = 56,
    /// ```text
    /// Space Emoji
    /// ```
    Space = 57,
    /// ```text
    /// AltGr ShiftAltGr
    /// ```
    Graph = 58,
    /// ```text
    /// RPrg/ROption/RAlt RApp/RCtrl/RCmd
    /// ```
    RightPrg = 59,
    /// ```text
    /// RApp/RCtrl/RCmd RPrg/ROption/RAlt
    /// ```
    RightApp = 60,
    /// ```text
    /// Left SelectLeft
    /// ```
    Left = 61,
    /// ```text
    /// Down SelectDown
    /// ```
    Down = 62,
    /// ```text
    /// Right SelectRight
    /// ```
    Right = 63,

    // Row 1 (Function)
    /// ```text
    /// ¬ ¦
    /// ```
    Not = 64,
    /// ```text
    /// F1 Macro1
    /// ```
    F1 = 65,
    /// ```text
    /// F2 Macro2
    /// ```
    F2 = 66,
    /// ```text
    /// F3 Macro3
    /// ```
    F3 = 67,
    /// ```text
    /// F4 Macro4
    /// ```
    F4 = 68,
    /// ```text
    /// F5 Macro5
    /// ```
    F5 = 69,
    /// ```text
    /// F6 Macro6
    /// ```
    F6 = 70,
    /// ```text
    /// F7 Macro7
    /// ```
    F7 = 71,
    /// ```text
    /// F8 Macro8
    /// ```
    F8 = 72,
    /// ```text
    /// F9 Macro9
    /// ```
    F9 = 73,
    /// ```text
    /// F10 Macro10
    /// ```
    F10 = 74,
    /// ```text
    /// F11 Macro11
    /// ```
    F11 = 75,
    /// ```text
    /// F12 Macro12
    /// ```
    F12 = 76,
    /// ```text
    /// Power Virtualization
    /// ```
    Power = 77,

    // Row 2 (Function)
    /// ```text
    /// InputModeNext InputModePrev
    /// ```
    Input = 78,
    /// ```text
    /// F13 Macro13
    /// ```
    F13 = 79,
    /// ```text
    /// F14 Macro14
    /// ```
    F14 = 80,
    /// ```text
    /// F15 Macro15
    /// ```
    F15 = 81,
    /// ```text
    /// F16 Macro16
    /// ```
    F16 = 82,
    /// ```text
    /// F17 Macro17
    /// ```
    F17 = 83,
    /// ```text
    /// F18 Macro18
    /// ```
    F18 = 84,
    /// ```text
    /// F19 Macro19
    /// ```
    F19 = 85,
    /// ```text
    /// F20 Macro20
    /// ```
    F20 = 86,
    /// ```text
    /// F21 Macro21
    /// ```
    F21 = 87,
    /// ```text
    /// F22 Macro22
    /// ```
    F22 = 88,
    /// ```text
    /// F23 Macro23
    /// ```
    F23 = 89,
    /// ```text
    /// F24 Macro24
    /// ```
    F24 = 90,

    // Row 3 (Function)
    /// ```text
    /// MiddleClick ShiftMiddleClick
    /// ```
    MiddleClick = 91,
    /// ```text
    /// AlphaNumeric ShiftAlphaNumeric
    /// ```
    AlphaNumeric = 92,
    /// ```text
    /// KanjiHalfWidthFullWidth ShiftKanjiHalfWidthFullWidth
    /// ```
    Kanji = 93,
    /// ```text
    /// Convert ShiftConvert
    /// ```
    Convert = 94,
    /// ```text
    /// NonConvert ShiftNonConvert
    /// ```
    NonConvert = 95,
    /// ```text
    /// Hangul ShiftHangul
    /// ```
    Hangul = 96,
    /// ```text
    /// Hanja ShiftHanja
    /// ```
    Hanja = 97,
    /// ```text
    /// Junja ShiftJunja
    /// ```
    Junja = 98,
    /// ```text
    /// KanaHiraganaKatakana ShiftKanaHiraganaKatakana
    /// ```
    Kana = 99,
    /// ```text
    /// Final ShiftFinal
    /// ```
    Final = 100,
    /// ```text
    /// Change ShiftChange
    /// ```
    Change = 101,
    /// ```text
    /// Accept ShiftAccept
    /// ```
    Accept = 102,
    /// ```text
    /// RightClick ShiftRightClick
    /// ```
    RightClick = 103,

    // Row 4 (Function)
    /// ```text
    /// LeftClick ShiftLeftClick
    /// ```
    LeftClick = 104,
    /// ```text
    /// Pause ShiftPause
    /// ```
    Pause = 105,
    /// ```text
    /// Break ShiftBreak
    /// ```
    Break = 106,
    /// ```text
    /// Clear ShiftClear
    /// ```
    Clear = 107,
    /// ```text
    /// ScrollLock ShiftScrollLock
    /// ```
    ScrollLock = 108,
    /// ```text
    /// NumpadLock ShiftNumpadLock
    /// ```
    NumLock = 109,
    /// ```text
    /// SystemLock ShiftSystemLock
    /// ```
    SysLock = 110,
    /// ```text
    /// Menu ShiftMenu
    /// ```
    Menu = 111,
    /// ```text
    /// BackPage
    /// ```
    Back = 112,
    /// ```text
    /// ForwardPage
    /// ```
    Forward = 113,
    /// ```text
    /// REnv/RSystem RMeta
    /// ```
    RightSys = 114,
    /// ```text
    /// PageUp ShiftPageUp
    /// ```
    PageUp = 115,
    /// ```text
    /// Dpi ShiftDpi
    /// ```
    DpiClick = 116,

    // Row 5 (Function)
    /// ```text
    /// LeftAppLock ShiftLeftAppLock
    /// ```
    LeftAppLock = 117,
    /// ```text
    /// LeftPrgLock ShiftLeftPrgLock
    /// ```
    LeftPrgLock = 118,
    /// ```text
    /// SideClic ShiftSideClick
    /// ```
    SideClick = 119,
    /// ```text
    /// Remove/Delete/Del ShiftRemove/ShiftDelete/ShiftDel
    /// ```
    Remove = 120,
    /// ```text
    /// Compose ShiftCompose
    /// ```
    Compose = 121,
    /// ```text
    /// Insert ShiftInsert
    /// ```
    Insert = 122,
    /// ```text
    /// RightPrgLock ShiftRightPrgLock
    /// ```
    RightPrgLock = 123,
    /// ```text
    /// RightAppLock ShiftRightAppLock
    /// ```
    RightAppLock = 124,
    /// ```text
    /// PageLeft ShiftPageLeft
    /// ```
    PageHome = 125,
    /// ```text
    /// PageDown ShiftPageDown
    /// ```
    PageDown = 126,
    /// ```text
    /// PageRight ShiftPageRight
    /// ```
    PageEnd = 127,

    // Row 1 (System)
    /// Switch to previous language
    LangPrev = 128,
    /// Launch web browser
    LaunchWeb = 129,
    /// Launch email client
    LaunchEmail = 130,
    /// Launch calculator / terminal
    LaunchCalculator = 131,
    /// Launch finance tracker
    LaunchFinance = 132,
    /// Launch multimedia player
    LaunchPlayer = 133,
    /// Launch games
    LaunchGame = 134,
    /// Launch calendar
    LaunchCalendar = 135,
    /// Launch notes
    LaunchNotes = 136,
    /// Launch video editor / recorder
    LaunchVideo = 137,
    /// Launch audio editor / recorder
    LaunchAudio = 138,
    /// Zoom screen out (a11y)
    ZoomOut = 139,
    /// Zoom screen in (a11y)
    ZoomIn = 140,
    /// Zoom screen reset (a11y)
    ZoomReset = 141,

    // Row 2 (System)
    /// Switch to next language
    LangNext = 142,
    /// Toggle display settings
    Display = 143,
    /// Switch to workspace up
    WorkspaceUp = 144,
    /// Lower brightness level
    Dim = 145,
    /// Raise brightness level
    Brighten = 146,
    /// Launch system settings
    LaunchSettings = 147,
    /// Launch network settings
    LaunchConfigNet = 148,
    /// Launch multimedia (audio/video) settings
    LaunchConfigAV = 149,
    /// Take a video of the screen
    ScreenCapture = 150,
    /// Copy screen contents to clipboard
    ScreenCopy = 151,
    /// Save screen contents to file
    ScreenShot = 152,
    /// Rotate screen widdershins
    ScreenRotateWiddershins = 153,
    /// Rotate screen clockwise
    ScreenRotateClockwise = 154,

    // Row 3 (System)
    /// Mobile phone switch app key
    MobileSwitchApp = 155,
    /// Select workspace to change order
    SelectWorkspace = 156,
    /// Switch to workspace down
    WorkspaceDown = 157,
    /// Select program to switch its workspace
    SelectApp = 158,
    /// Alternate audio output
    SwitchSpeaker = 159,
    /// Alternate audio input
    SwitchMicrophone = 160,
    /// Silence the microphone
    MicrophoneMute = 161,
    /// Decrease microphone volume
    MicrophoneQuieter = 162,
    /// Increase microphone volume
    MicrophoneLouder = 163,
    /// Midi configuration
    Midi = 164,
    /// Controller/Joystick configuration
    Controller = 165,
    /// Phone configuration
    Phone = 166,
    /// Open a new window
    NewWindow = 167,

    // Row 4 (System)
    /// Mobile home screen key
    MobileHomeScreen = 168,
    /// Mute output volume
    SpeakerMute = 169,
    /// Output volume decrease
    SpeakerQuieter = 170,
    /// Output volume increase
    SpeakerLouder = 171,
    /// Rewind
    Rewind = 172,
    /// Stop/Eject
    StopEject = 173,
    /// Fast Forward
    FastForward = 174,
    /// Toggle dual-screen/mirroring
    Mirror = 175,
    /// Decrease exposure
    WebcamDim = 176,
    /// Increase exposure
    WebcamBrighten = 177,
    /// System help
    Help = 178,
    /// Split screen tiling, move window up
    TileUp = 179,
    /// Scroll up
    ScrollUp = 180,

    // Row 5 (System)
    /// Scroll left
    ScrollLeft = 181,
    /// Scroll down
    ScrollDown = 182,
    /// Scroll right
    ScrollRight = 183,
    /// Previous multimedia track
    Prev = 184,
    /// Play/pause multimedia track
    Play = 185,
    /// Next multimedia track
    Next = 186,
    /// Launch call button on mobile
    MobileLaunchCall = 187,
    /// Launch camera button on mobile
    MobileLaunchCamera = 188,
    /// Split screen tiling, move window left
    TileLeft = 189,
    /// Split screen tiling, move window down
    TileDown = 190,
    /// Split screen tiling, move window right
    TileRight = 191,

    // Row 1 (Numpad)
    /// FIXME
    Reserved192 = 192,
    /// FIXME
    Reserved193 = 193,
    /// FIXME
    Reserved194 = 194,
    /// FIXME
    Reserved195 = 195,
    /// FIXME
    Reserved196 = 196,
    /// FIXME
    Reserved197 = 197,
    /// FIXME
    Reserved198 = 198,
    /// FIXME
    Reserved199 = 199,
    /// FIXME
    Reserved200 = 200,
    /// FIXME
    Reserved201 = 201,
    /// FIXME
    Reserved202 = 202,
    /// FIXME
    Reserved203 = 203,
    /// FIXME
    Reserved204 = 204,
    /// FIXME
    Reserved205 = 205,

    // Row 2 (Numpad)
    /// Numpad tab
    NumTab = 206,
    /// Numpad divide
    NumDivide = 207,
    /// Numpad multiply
    NumMultiply = 208,
    /// Numpad subtract
    NumSubtract = 209,
    /// Numpad add
    NumAdd = 210,
    /// Numpad comma
    NumComma = 211,
    /// FIXME
    Reserved212 = 212,
    /// FIXME
    Reserved213 = 213,
    /// FIXME
    Reserved214 = 214,
    /// FIXME
    Reserved215 = 215,
    /// FIXME
    Reserved216 = 216,
    /// FIXME
    Reserved217 = 217,
    /// FIXME
    Reserved218 = 218,
    
    // Row 3 (Numpad)
    /// FIXME
    Reserved219 = 219,
    /// Numpad 0
    Num0 = 220,
    /// Numpad 1
    Num1 = 221,
    /// Numpad 2
    Num2 = 222,
    /// Numpad 3
    Num3 = 223,
    /// Numpad 4
    Num4 = 224,
    /// Numpad 5
    Num5 = 225,
    /// Numpad 6
    Num6 = 226,
    /// Numpad 7
    Num7 = 227,
    /// Numpad 8
    Num8 = 228,
    /// Numpad 9
    Num9 = 229,
    /// Numpad decimal point
    NumDecimalPoint = 230,
    /// Numpad enter
    NumEnter = 231,

    // Row 4 (Numpad)
    /// Mobile back key
    MobileActivityBack = 232,
    /// FIXME
    Reserved233 = 233,
    /// FIXME
    Reserved234 = 234,
    /// FIXME
    Reserved235 = 235,
    /// FIXME
    Reserved236 = 236,
    /// FIXME
    Reserved237 = 237,
    /// FIXME
    Reserved238 = 238,
    /// FIXME
    Reserved239 = 239,
    /// FIXME
    Reserved240 = 240,
    /// FIXME
    Reserved241 = 241,
    /// FIXME
    Reserved242 = 242,
    /// Numpad up arrow
    NumUp = 243,
    /// FIXME
    Reserved244 = 244,

    // Row 5 (Numpad)
    /// FIXME
    Reserved245 = 245,
    /// FIXME
    Reserved246 = 246,
    /// FIXME
    Reserved247 = 247,
    /// FIXME
    Reserved248 = 248,
    /// FIXME
    Reserved249 = 249,
    /// FIXME
    Reserved250 = 250,
    /// FIXME
    Reserved251 = 251,
    /// FIXME
    Reserved252 = 252,
    /// Numpad left arrow
    NumLeft = 253,
    /// Numpad down arrow
    NumDown = 254,
    /// Numpad right arrow
    NumRight = 255,
}

impl From<u8> for Key {
    fn from(other: u8) -> Self {
        // Safe because all variants are checked
        unsafe { std::mem::transmute(other) }
    }
}
