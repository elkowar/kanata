//! Platform specific code for OS key code mappings.

use kanata_keyberon::key_code::KeyCode;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use rustc_hash::FxHashMap as HashMap;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

mod mappings;
pub use mappings::*;

static CUSTOM_STRS_TO_OSCODES: Lazy<Mutex<HashMap<String, OsCode>>> = Lazy::new(|| {
    let mut mappings = HashMap::default();
    add_default_str_osc_mappings(&mut mappings);
    mappings.shrink_to_fit();
    Mutex::new(mappings)
});

/// Replaces the stateful custom `String` to `OsCode` mapping in this module with the input
/// mapping.
///
/// This will change how `str_to_oscode` behaves. One could imagine that a new `struct` could be
/// created and `str_to_oscode` would become a method on that struct instead of a standalone
/// function. I'm too lazy to do that right now and based on how `keys` is used right now, it
/// should not be a problem. A potential immediate issue that comes to mind is concurrent tests
/// that have `defcustomkeys`.
pub fn replace_custom_str_oscode_mapping(mapping: &HashMap<String, OsCode>) {
    let mut local_mapping = CUSTOM_STRS_TO_OSCODES.lock();
    local_mapping.clear();
    local_mapping.extend(mapping.iter().map(|kv| (kv.0.clone(), *kv.1)));
    add_default_str_osc_mappings(&mut local_mapping);
    local_mapping.shrink_to_fit();
}

/// Clears the stateful custom `String` to `OsCode` mapping in this module.
pub fn clear_custom_str_oscode_mapping() {
    let mut local_mapping = CUSTOM_STRS_TO_OSCODES.lock();
    local_mapping.clear();
}

/// Used for backwards compatibility. If there is hardcoded key name in `str_to_oscode` that would
/// be useful to remap via `defcustomkeys`, then it should be moved into here. This is so that the
/// key name can be remapped while also working for older configurations that already use it.
fn add_default_str_osc_mappings(mapping: &mut HashMap<String, OsCode>) {
    let default_mappings = [
        ("+", OsCode::KEY_KPPLUS),
        ("[", OsCode::KEY_LEFTBRACE),
        ("]", OsCode::KEY_RIGHTBRACE),
        ("{", OsCode::KEY_LEFTBRACE),
        ("}", OsCode::KEY_RIGHTBRACE),
        ("/", OsCode::KEY_SLASH),
        (";", OsCode::KEY_SEMICOLON),
        ("`", OsCode::KEY_GRAVE),
        ("=", OsCode::KEY_EQUAL),
        ("-", OsCode::KEY_MINUS),
        ("'", OsCode::KEY_APOSTROPHE),
        (",", OsCode::KEY_COMMA),
        (".", OsCode::KEY_DOT),
        ("\\", OsCode::KEY_BACKSLASH),
    ];
    for dm in default_mappings {
        mapping.entry(dm.0.into()).or_insert(dm.1);
    }
}

/// Convert a `&str` to an `OsCode`.
///
/// kmonad's str to key mapping is found here as a reference:
/// https://github.com/kmonad/kmonad/blob/master/src/KMonad/Keyboard/Keycode.hs
///
/// Do your best to keep the str side a maximum character length of 4 so that configuration file
/// can stay clean.
#[rustfmt::skip]
pub fn str_to_oscode(s: &str) -> Option<OsCode> {
    Some(match s {
        "grv" => OsCode::KEY_GRAVE,
        "1" => OsCode::KEY_1,
        "2" => OsCode::KEY_2,
        "3" => OsCode::KEY_3,
        "4" => OsCode::KEY_4,
        "5" => OsCode::KEY_5,
        "6" => OsCode::KEY_6,
        "7" => OsCode::KEY_7,
        "8" => OsCode::KEY_8,
        "9" => OsCode::KEY_9,
        "0" => OsCode::KEY_0,
        "min" => OsCode::KEY_MINUS,
        "eql" => OsCode::KEY_EQUAL,
        "bspc" | "bks" => OsCode::KEY_BACKSPACE,
        "tab" => OsCode::KEY_TAB,
        "q" => OsCode::KEY_Q,
        "w" => OsCode::KEY_W,
        "e" => OsCode::KEY_E,
        "r" => OsCode::KEY_R,
        "t" => OsCode::KEY_T,
        "y" => OsCode::KEY_Y,
        "u" => OsCode::KEY_U,
        "i" => OsCode::KEY_I,
        "o" => OsCode::KEY_O,
        "p" => OsCode::KEY_P,
        "lbrc" => OsCode::KEY_LEFTBRACE,
        "rbrc" => OsCode::KEY_RIGHTBRACE,
        "bksl" | "yen" | "¥" => OsCode::KEY_BACKSLASH,
        "caps" => OsCode::KEY_CAPSLOCK,
        "a" => OsCode::KEY_A,
        "s" => OsCode::KEY_S,
        "d" => OsCode::KEY_D,
        "f" => OsCode::KEY_F,
        "g" => OsCode::KEY_G,
        "h" => OsCode::KEY_H,
        "j" => OsCode::KEY_J,
        "k" => OsCode::KEY_K,
        "l" => OsCode::KEY_L,
        "scln" => OsCode::KEY_SEMICOLON,
        "apo" | "apos" => OsCode::KEY_APOSTROPHE,
        "ret" | "return" | "ent" | "enter" => OsCode::KEY_ENTER,
        "lshift" | "lshft" | "lsft" | "shft" | "sft" => OsCode::KEY_LEFTSHIFT,
        "z" => OsCode::KEY_Z,
        "x" => OsCode::KEY_X,
        "c" => OsCode::KEY_C,
        "v" => OsCode::KEY_V,
        "b" => OsCode::KEY_B,
        "n" => OsCode::KEY_N,
        "m" => OsCode::KEY_M,
        "comm" => OsCode::KEY_COMMA,
        "kp=" | "clr" => OsCode::KEY_CLEAR,
        // The kp<etc> keys are also known as the numpad keys. E.g. below is numpad enter.
        "kp0" => OsCode::KEY_KP0,
        "kp1" => OsCode::KEY_KP1,
        "kp2" => OsCode::KEY_KP2,
        "kp3" => OsCode::KEY_KP3,
        "kp4" => OsCode::KEY_KP4,
        "kp5" => OsCode::KEY_KP5,
        "kp6" => OsCode::KEY_KP6,
        "kp7" => OsCode::KEY_KP7,
        "kp8" => OsCode::KEY_KP8,
        "kp9" => OsCode::KEY_KP9,
        "kprt" => OsCode::KEY_KPENTER,
        "kp/" => OsCode::KEY_KPSLASH,
        "kp+" => OsCode::KEY_KPPLUS,
        "kp*" => OsCode::KEY_KPASTERISK,
        "kp-" => OsCode::KEY_KPMINUS,
        "kp." => OsCode::KEY_KPDOT,
        "ssrq" | "sys" => OsCode::KEY_SYSRQ,
        // Typically the Non-US backslash, near the left shift key
        "102d" | "lsgt" | "nubs" | "nonusbslash" => OsCode::KEY_102ND,
        "scrlck" | "slck" => OsCode::KEY_SCROLLLOCK,
        "pause" | "break" | "brk" => OsCode::KEY_PAUSE,
        "wkup" => OsCode::KEY_WAKEUP,
        "esc" => OsCode::KEY_ESC,
        "rshift" | "rshft" | "rsft" => OsCode::KEY_RIGHTSHIFT,
        "lctrl" | "lctl" | "ctl" => OsCode::KEY_LEFTCTRL,
        "lalt" | "alt" => OsCode::KEY_LEFTALT,
        "spc" => OsCode::KEY_SPACE,
        "ralt" => OsCode::KEY_RIGHTALT,
        "comp" | "cmps" | "cmp" | "menu" | "apps" => OsCode::KEY_COMPOSE,
        // Also known as Windows, GUI, Comand, Super
        "lmeta" | "lmet" | "met" => OsCode::KEY_LEFTMETA,
        "rmeta" | "rmet" => OsCode::KEY_RIGHTMETA,
        "rctrl" | "rctl" => OsCode::KEY_RIGHTCTRL,
        "del" => OsCode::KEY_DELETE,
        "ins" => OsCode::KEY_INSERT,
        "bck" => OsCode::KEY_BACK,
        "fwd" => OsCode::KEY_FORWARD,
        "pgup" => OsCode::KEY_PAGEUP,
        "pgdn" => OsCode::KEY_PAGEDOWN,
        "up" => OsCode::KEY_UP,
        "down" => OsCode::KEY_DOWN,
        "lft" | "left" => OsCode::KEY_LEFT,
        "rght" => OsCode::KEY_RIGHT,
        "home" => OsCode::KEY_HOME,
        "end" => OsCode::KEY_END,
        "nlck" | "nlk" => OsCode::KEY_NUMLOCK,
        "mute" => OsCode::KEY_MUTE,
        "volu" => OsCode::KEY_VOLUMEUP,
        "voldwn" | "vold" => OsCode::KEY_VOLUMEDOWN,
        "brup" | "bru" => OsCode::KEY_BRIGHTNESSUP,
        "brdown" | "brdwn" | "brdn" => OsCode::KEY_BRIGHTNESSDOWN,
        "blup" => OsCode::KEY_KBDILLUMUP,
        "bldn" => OsCode::KEY_KBDILLUMDOWN,
        "next" => OsCode::KEY_NEXTSONG,
        "pp" => OsCode::KEY_PLAYPAUSE,
        "prev" => OsCode::KEY_PREVIOUSSONG,
        "f1" => OsCode::KEY_F1,
        "f2" => OsCode::KEY_F2,
        "f3" => OsCode::KEY_F3,
        "f4" => OsCode::KEY_F4,
        "f5" => OsCode::KEY_F5,
        "f6" => OsCode::KEY_F6,
        "f7" => OsCode::KEY_F7,
        "f8" => OsCode::KEY_F8,
        "f9" => OsCode::KEY_F9,
        "f10" => OsCode::KEY_F10,
        "f11" => OsCode::KEY_F11,
        "f12" => OsCode::KEY_F12,
        "f13" => OsCode::KEY_F13,
        "f14" => OsCode::KEY_F14,
        "f15" => OsCode::KEY_F15,
        "f16" => OsCode::KEY_F16,
        "f17" => OsCode::KEY_F17,
        "f18" => OsCode::KEY_F18,
        "f19" => OsCode::KEY_F19,
        "f20" => OsCode::KEY_F20,
        "f21" => OsCode::KEY_F21,
        "f22" => OsCode::KEY_F22,
        "f23" => OsCode::KEY_F23,
        "f24" => OsCode::KEY_F24,
        #[cfg(target_os = "windows")]
        "kana" | "katakana" | "katakanahiragana" => OsCode::KEY_HANGEUL,
        #[cfg(target_os = "linux")]
        "kana" | "katakanahiragana" => OsCode::KEY_KATAKANAHIRAGANA,
        #[cfg(target_os = "linux")]
        "hiragana" => OsCode::KEY_HIRAGANA,
        #[cfg(target_os = "linux")]
        "katakana" => OsCode::KEY_KATAKANA,
        "cnv" | "conv" | "henk" | "hnk" | "henkan" => OsCode::KEY_HENKAN,
        "ncnv" | "mhnk" | "muhenkan" => OsCode::KEY_MUHENKAN,
        "ro" => OsCode::KEY_RO,

        #[cfg(target_os = "linux")]
        "prtsc" | "prnt" => OsCode::KEY_SYSRQ,
        #[cfg(target_os = "windows")]
        "prtsc" | "prnt" => OsCode::KEY_PRINT,

        "mlft" | "mouseleft" => OsCode::BTN_LEFT,
        "mrgt" | "mouseright" => OsCode::BTN_RIGHT,
        "mmid" | "mousemid" => OsCode::BTN_MIDDLE,
        "mfwd" | "mouseforward" => OsCode::BTN_EXTRA,
        "mbck" | "mousebackward" => OsCode::BTN_SIDE,

        "hmpg" | "homepage" => OsCode::KEY_HOMEPAGE,
        "mdia" | "media" => OsCode::KEY_MEDIA,
        "mail" => OsCode::KEY_MAIL,
        "email" => OsCode::KEY_EMAIL,
        "calc" => OsCode::KEY_CALC,

        // NOTE: these are linux-only right now due to missing the mappings in windows.rs
        #[cfg(tagret_os = "linux")]
        "plyr" | "player" => OsCode::KEY_PLAYER,
        #[cfg(tagret_os = "linux")]
        "powr" | "power" => OsCode::KEY_POWER,
        #[cfg(tagret_os = "linux")]
        "zzz" | "sleep" => OsCode::KEY_SLEEP,

        _ => {
            let custom_mappings = CUSTOM_STRS_TO_OSCODES.lock();
            match custom_mappings.get(s) {
                Some(osc) => *osc,
                None => return None,
            }
        }
    })
}

/// This is a shameless copy of evdev_rs::enums::EV_KEY.
/// I've added the Copy trait and I'll be able
/// to added my own Impl(s) to it
#[repr(u16)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OsCode {
    KEY_RESERVED = 0,
    KEY_ESC = 1,
    KEY_1 = 2,
    KEY_2 = 3,
    KEY_3 = 4,
    KEY_4 = 5,
    KEY_5 = 6,
    KEY_6 = 7,
    KEY_7 = 8,
    KEY_8 = 9,
    KEY_9 = 10,
    KEY_0 = 11,
    KEY_MINUS = 12,
    KEY_EQUAL = 13,
    KEY_BACKSPACE = 14,
    KEY_TAB = 15,
    KEY_Q = 16,
    KEY_W = 17,
    KEY_E = 18,
    KEY_R = 19,
    KEY_T = 20,
    KEY_Y = 21,
    KEY_U = 22,
    KEY_I = 23,
    KEY_O = 24,
    KEY_P = 25,
    KEY_LEFTBRACE = 26,
    KEY_RIGHTBRACE = 27,
    KEY_ENTER = 28,
    KEY_LEFTCTRL = 29,
    KEY_A = 30,
    KEY_S = 31,
    KEY_D = 32,
    KEY_F = 33,
    KEY_G = 34,
    KEY_H = 35,
    KEY_J = 36,
    KEY_K = 37,
    KEY_L = 38,
    KEY_SEMICOLON = 39,
    KEY_APOSTROPHE = 40,
    KEY_GRAVE = 41,
    KEY_LEFTSHIFT = 42,
    KEY_BACKSLASH = 43,
    KEY_Z = 44,
    KEY_X = 45,
    KEY_C = 46,
    KEY_V = 47,
    KEY_B = 48,
    KEY_N = 49,
    KEY_M = 50,
    KEY_COMMA = 51,
    KEY_DOT = 52,
    KEY_SLASH = 53,
    KEY_RIGHTSHIFT = 54,
    KEY_KPASTERISK = 55,
    KEY_LEFTALT = 56,
    KEY_SPACE = 57,
    KEY_CAPSLOCK = 58,
    KEY_F1 = 59,
    KEY_F2 = 60,
    KEY_F3 = 61,
    KEY_F4 = 62,
    KEY_F5 = 63,
    KEY_F6 = 64,
    KEY_F7 = 65,
    KEY_F8 = 66,
    KEY_F9 = 67,
    KEY_F10 = 68,
    KEY_NUMLOCK = 69,
    KEY_SCROLLLOCK = 70,
    KEY_KP7 = 71,
    KEY_KP8 = 72,
    KEY_KP9 = 73,
    KEY_KPMINUS = 74,
    KEY_KP4 = 75,
    KEY_KP5 = 76,
    KEY_KP6 = 77,
    KEY_KPPLUS = 78,
    KEY_KP1 = 79,
    KEY_KP2 = 80,
    KEY_KP3 = 81,
    KEY_KP0 = 82,
    KEY_KPDOT = 83,
    KEY_84 = 84,
    KEY_ZENKAKUHANKAKU = 85,
    KEY_102ND = 86,
    KEY_F11 = 87,
    KEY_F12 = 88,
    KEY_RO = 89,
    KEY_KATAKANA = 90,
    KEY_HIRAGANA = 91,
    KEY_HENKAN = 92,
    KEY_KATAKANAHIRAGANA = 93,
    KEY_MUHENKAN = 94,
    KEY_KPJPCOMMA = 95,
    KEY_KPENTER = 96,
    KEY_RIGHTCTRL = 97,
    KEY_KPSLASH = 98,
    KEY_SYSRQ = 99,
    KEY_RIGHTALT = 100,
    KEY_LINEFEED = 101,
    KEY_HOME = 102,
    KEY_UP = 103,
    KEY_PAGEUP = 104,
    KEY_LEFT = 105,
    KEY_RIGHT = 106,
    KEY_END = 107,
    KEY_DOWN = 108,
    KEY_PAGEDOWN = 109,
    KEY_INSERT = 110,
    KEY_DELETE = 111,
    KEY_MACRO = 112,
    KEY_MUTE = 113,
    KEY_VOLUMEDOWN = 114,
    KEY_VOLUMEUP = 115,
    KEY_POWER = 116,
    KEY_KPEQUAL = 117,
    KEY_KPPLUSMINUS = 118,
    KEY_PAUSE = 119,
    KEY_SCALE = 120,
    KEY_KPCOMMA = 121,
    KEY_HANGEUL = 122,
    KEY_HANJA = 123,
    KEY_YEN = 124,
    KEY_LEFTMETA = 125,
    KEY_RIGHTMETA = 126,
    KEY_COMPOSE = 127,
    KEY_STOP = 128,
    KEY_AGAIN = 129,
    KEY_PROPS = 130,
    KEY_UNDO = 131,
    KEY_FRONT = 132,
    KEY_COPY = 133,
    KEY_OPEN = 134,
    KEY_PASTE = 135,
    KEY_FIND = 136,
    KEY_CUT = 137,
    KEY_HELP = 138,
    KEY_MENU = 139,
    KEY_CALC = 140,
    KEY_SETUP = 141,
    KEY_SLEEP = 142,
    KEY_WAKEUP = 143,
    KEY_FILE = 144,
    KEY_SENDFILE = 145,
    KEY_DELETEFILE = 146,
    KEY_XFER = 147,
    KEY_PROG1 = 148,
    KEY_PROG2 = 149,
    KEY_WWW = 150,
    KEY_MSDOS = 151,
    KEY_COFFEE = 152,
    KEY_ROTATE_DISPLAY = 153,
    KEY_CYCLEWINDOWS = 154,
    KEY_MAIL = 155,
    KEY_BOOKMARKS = 156,
    KEY_COMPUTER = 157,
    KEY_BACK = 158,
    KEY_FORWARD = 159,
    KEY_CLOSECD = 160,
    KEY_EJECTCD = 161,
    KEY_EJECTCLOSECD = 162,
    KEY_NEXTSONG = 163,
    KEY_PLAYPAUSE = 164,
    KEY_PREVIOUSSONG = 165,
    KEY_STOPCD = 166,
    KEY_RECORD = 167,
    KEY_REWIND = 168,
    KEY_PHONE = 169,
    KEY_ISO = 170,
    KEY_CONFIG = 171,
    KEY_HOMEPAGE = 172,
    KEY_REFRESH = 173,
    KEY_EXIT = 174,
    KEY_MOVE = 175,
    KEY_EDIT = 176,
    KEY_SCROLLUP = 177,
    KEY_SCROLLDOWN = 178,
    KEY_KPLEFTPAREN = 179,
    KEY_KPRIGHTPAREN = 180,
    KEY_NEW = 181,
    KEY_REDO = 182,
    KEY_F13 = 183,
    KEY_F14 = 184,
    KEY_F15 = 185,
    KEY_F16 = 186,
    KEY_F17 = 187,
    KEY_F18 = 188,
    KEY_F19 = 189,
    KEY_F20 = 190,
    KEY_F21 = 191,
    KEY_F22 = 192,
    KEY_F23 = 193,
    KEY_F24 = 194,
    KEY_195 = 195,
    KEY_196 = 196,
    KEY_197 = 197,
    KEY_198 = 198,
    KEY_199 = 199,
    KEY_PLAYCD = 200,
    KEY_PAUSECD = 201,
    KEY_PROG3 = 202,
    KEY_PROG4 = 203,
    KEY_DASHBOARD = 204,
    KEY_SUSPEND = 205,
    KEY_CLOSE = 206,
    KEY_PLAY = 207,
    KEY_FASTFORWARD = 208,
    KEY_BASSBOOST = 209,
    KEY_PRINT = 210,
    KEY_HP = 211,
    KEY_CAMERA = 212,
    KEY_SOUND = 213,
    KEY_QUESTION = 214,
    KEY_EMAIL = 215,
    KEY_CHAT = 216,
    KEY_SEARCH = 217,
    KEY_CONNECT = 218,
    KEY_FINANCE = 219,
    KEY_SPORT = 220,
    KEY_SHOP = 221,
    KEY_ALTERASE = 222,
    KEY_CANCEL = 223,
    KEY_BRIGHTNESSDOWN = 224,
    KEY_BRIGHTNESSUP = 225,
    KEY_MEDIA = 226,
    KEY_SWITCHVIDEOMODE = 227,
    KEY_KBDILLUMTOGGLE = 228,
    KEY_KBDILLUMDOWN = 229,
    KEY_KBDILLUMUP = 230,
    KEY_SEND = 231,
    KEY_REPLY = 232,
    KEY_FORWARDMAIL = 233,
    KEY_SAVE = 234,
    KEY_DOCUMENTS = 235,
    KEY_BATTERY = 236,
    KEY_BLUETOOTH = 237,
    KEY_WLAN = 238,
    KEY_UWB = 239,
    KEY_UNKNOWN = 240,
    KEY_VIDEO_NEXT = 241,
    KEY_VIDEO_PREV = 242,
    KEY_BRIGHTNESS_CYCLE = 243,
    KEY_BRIGHTNESS_AUTO = 244,
    KEY_DISPLAY_OFF = 245,
    KEY_WWAN = 246,
    KEY_RFKILL = 247,
    KEY_MICMUTE = 248,
    KEY_249 = 249,
    KEY_250 = 250,
    KEY_251 = 251,
    KEY_252 = 252,
    KEY_253 = 253,
    KEY_254 = 254,
    KEY_255 = 255,
    BTN_0 = 256,
    BTN_1 = 257,
    BTN_2 = 258,
    BTN_3 = 259,
    BTN_4 = 260,
    BTN_5 = 261,
    BTN_6 = 262,
    BTN_7 = 263,
    BTN_8 = 264,
    BTN_9 = 265,
    KEY_266 = 266,
    KEY_267 = 267,
    KEY_268 = 268,
    KEY_269 = 269,
    KEY_270 = 270,
    KEY_271 = 271,
    BTN_LEFT = 272,
    BTN_RIGHT = 273,
    BTN_MIDDLE = 274,
    BTN_SIDE = 275,
    BTN_EXTRA = 276,
    BTN_FORWARD = 277,
    BTN_BACK = 278,
    BTN_TASK = 279,
    KEY_280 = 280,
    KEY_281 = 281,
    KEY_282 = 282,
    KEY_283 = 283,
    KEY_284 = 284,
    KEY_285 = 285,
    KEY_286 = 286,
    KEY_287 = 287,
    BTN_TRIGGER = 288,
    BTN_THUMB = 289,
    BTN_THUMB2 = 290,
    BTN_TOP = 291,
    BTN_TOP2 = 292,
    BTN_PINKIE = 293,
    BTN_BASE = 294,
    BTN_BASE2 = 295,
    BTN_BASE3 = 296,
    BTN_BASE4 = 297,
    BTN_BASE5 = 298,
    BTN_BASE6 = 299,
    KEY_300 = 300,
    KEY_301 = 301,
    KEY_302 = 302,
    BTN_DEAD = 303,
    BTN_SOUTH = 304,
    BTN_EAST = 305,
    BTN_C = 306,
    BTN_NORTH = 307,
    BTN_WEST = 308,
    BTN_Z = 309,
    BTN_TL = 310,
    BTN_TR = 311,
    BTN_TL2 = 312,
    BTN_TR2 = 313,
    BTN_SELECT = 314,
    BTN_START = 315,
    BTN_MODE = 316,
    BTN_THUMBL = 317,
    BTN_THUMBR = 318,
    KEY_319 = 319,
    BTN_TOOL_PEN = 320,
    BTN_TOOL_RUBBER = 321,
    BTN_TOOL_BRUSH = 322,
    BTN_TOOL_PENCIL = 323,
    BTN_TOOL_AIRBRUSH = 324,
    BTN_TOOL_FINGER = 325,
    BTN_TOOL_MOUSE = 326,
    BTN_TOOL_LENS = 327,
    BTN_TOOL_QUINTTAP = 328,
    BTN_STYLUS3 = 329,
    BTN_TOUCH = 330,
    BTN_STYLUS = 331,
    BTN_STYLUS2 = 332,
    BTN_TOOL_DOUBLETAP = 333,
    BTN_TOOL_TRIPLETAP = 334,
    BTN_TOOL_QUADTAP = 335,
    BTN_GEAR_DOWN = 336,
    BTN_GEAR_UP = 337,
    KEY_338 = 338,
    KEY_339 = 339,
    KEY_340 = 340,
    KEY_341 = 341,
    KEY_342 = 342,
    KEY_343 = 343,
    KEY_344 = 344,
    KEY_345 = 345,
    KEY_346 = 346,
    KEY_347 = 347,
    KEY_348 = 348,
    KEY_349 = 349,
    KEY_350 = 350,
    KEY_351 = 351,
    KEY_OK = 352,
    KEY_SELECT = 353,
    KEY_GOTO = 354,
    KEY_CLEAR = 355,
    KEY_POWER2 = 356,
    KEY_OPTION = 357,
    KEY_INFO = 358,
    KEY_TIME = 359,
    KEY_VENDOR = 360,
    KEY_ARCHIVE = 361,
    KEY_PROGRAM = 362,
    KEY_CHANNEL = 363,
    KEY_FAVORITES = 364,
    KEY_EPG = 365,
    KEY_PVR = 366,
    KEY_MHP = 367,
    KEY_LANGUAGE = 368,
    KEY_TITLE = 369,
    KEY_SUBTITLE = 370,
    KEY_ANGLE = 371,
    KEY_FULL_SCREEN = 372,
    KEY_MODE = 373,
    KEY_KEYBOARD = 374,
    KEY_ASPECT_RATIO = 375,
    KEY_PC = 376,
    KEY_TV = 377,
    KEY_TV2 = 378,
    KEY_VCR = 379,
    KEY_VCR2 = 380,
    KEY_SAT = 381,
    KEY_SAT2 = 382,
    KEY_CD = 383,
    KEY_TAPE = 384,
    KEY_RADIO = 385,
    KEY_TUNER = 386,
    KEY_PLAYER = 387,
    KEY_TEXT = 388,
    KEY_DVD = 389,
    KEY_AUX = 390,
    KEY_MP3 = 391,
    KEY_AUDIO = 392,
    KEY_VIDEO = 393,
    KEY_DIRECTORY = 394,
    KEY_LIST = 395,
    KEY_MEMO = 396,
    KEY_CALENDAR = 397,
    KEY_RED = 398,
    KEY_GREEN = 399,
    KEY_YELLOW = 400,
    KEY_BLUE = 401,
    KEY_CHANNELUP = 402,
    KEY_CHANNELDOWN = 403,
    KEY_FIRST = 404,
    KEY_LAST = 405,
    KEY_AB = 406,
    KEY_NEXT = 407,
    KEY_RESTART = 408,
    KEY_SLOW = 409,
    KEY_SHUFFLE = 410,
    KEY_BREAK = 411,
    KEY_PREVIOUS = 412,
    KEY_DIGITS = 413,
    KEY_TEEN = 414,
    KEY_TWEN = 415,
    KEY_VIDEOPHONE = 416,
    KEY_GAMES = 417,
    KEY_ZOOMIN = 418,
    KEY_ZOOMOUT = 419,
    KEY_ZOOMRESET = 420,
    KEY_WORDPROCESSOR = 421,
    KEY_EDITOR = 422,
    KEY_SPREADSHEET = 423,
    KEY_GRAPHICSEDITOR = 424,
    KEY_PRESENTATION = 425,
    KEY_DATABASE = 426,
    KEY_NEWS = 427,
    KEY_VOICEMAIL = 428,
    KEY_ADDRESSBOOK = 429,
    KEY_MESSENGER = 430,
    KEY_DISPLAYTOGGLE = 431,
    KEY_SPELLCHECK = 432,
    KEY_LOGOFF = 433,
    KEY_DOLLAR = 434,
    KEY_EURO = 435,
    KEY_FRAMEBACK = 436,
    KEY_FRAMEFORWARD = 437,
    KEY_CONTEXT_MENU = 438,
    KEY_MEDIA_REPEAT = 439,
    KEY_10CHANNELSUP = 440,
    KEY_10CHANNELSDOWN = 441,
    KEY_IMAGES = 442,
    KEY_443 = 443,
    KEY_444 = 444,
    KEY_445 = 445,
    KEY_446 = 446,
    KEY_447 = 447,
    KEY_DEL_EOL = 448,
    KEY_DEL_EOS = 449,
    KEY_INS_LINE = 450,
    KEY_DEL_LINE = 451,
    KEY_452 = 452,
    KEY_453 = 453,
    KEY_454 = 454,
    KEY_455 = 455,
    KEY_456 = 456,
    KEY_457 = 457,
    KEY_458 = 458,
    KEY_459 = 459,
    KEY_460 = 460,
    KEY_461 = 461,
    KEY_462 = 462,
    KEY_463 = 463,
    KEY_FN = 464,
    KEY_FN_ESC = 465,
    KEY_FN_F1 = 466,
    KEY_FN_F2 = 467,
    KEY_FN_F3 = 468,
    KEY_FN_F4 = 469,
    KEY_FN_F5 = 470,
    KEY_FN_F6 = 471,
    KEY_FN_F7 = 472,
    KEY_FN_F8 = 473,
    KEY_FN_F9 = 474,
    KEY_FN_F10 = 475,
    KEY_FN_F11 = 476,
    KEY_FN_F12 = 477,
    KEY_FN_1 = 478,
    KEY_FN_2 = 479,
    KEY_FN_D = 480,
    KEY_FN_E = 481,
    KEY_FN_F = 482,
    KEY_FN_S = 483,
    KEY_FN_B = 484,
    KEY_485 = 485,
    KEY_486 = 486,
    KEY_487 = 487,
    KEY_488 = 488,
    KEY_489 = 489,
    KEY_490 = 490,
    KEY_491 = 491,
    KEY_492 = 492,
    KEY_493 = 493,
    KEY_494 = 494,
    KEY_495 = 495,
    KEY_496 = 496,
    KEY_BRL_DOT1 = 497,
    KEY_BRL_DOT2 = 498,
    KEY_BRL_DOT3 = 499,
    KEY_BRL_DOT4 = 500,
    KEY_BRL_DOT5 = 501,
    KEY_BRL_DOT6 = 502,
    KEY_BRL_DOT7 = 503,
    KEY_BRL_DOT8 = 504,
    KEY_BRL_DOT9 = 505,
    KEY_BRL_DOT10 = 506,
    KEY_507 = 507,
    KEY_508 = 508,
    KEY_509 = 509,
    KEY_510 = 510,
    KEY_511 = 511,
    KEY_NUMERIC_0 = 512,
    KEY_NUMERIC_1 = 513,
    KEY_NUMERIC_2 = 514,
    KEY_NUMERIC_3 = 515,
    KEY_NUMERIC_4 = 516,
    KEY_NUMERIC_5 = 517,
    KEY_NUMERIC_6 = 518,
    KEY_NUMERIC_7 = 519,
    KEY_NUMERIC_8 = 520,
    KEY_NUMERIC_9 = 521,
    KEY_NUMERIC_STAR = 522,
    KEY_NUMERIC_POUND = 523,
    KEY_NUMERIC_A = 524,
    KEY_NUMERIC_B = 525,
    KEY_NUMERIC_C = 526,
    KEY_NUMERIC_D = 527,
    KEY_CAMERA_FOCUS = 528,
    KEY_WPS_BUTTON = 529,
    KEY_TOUCHPAD_TOGGLE = 530,
    KEY_TOUCHPAD_ON = 531,
    KEY_TOUCHPAD_OFF = 532,
    KEY_CAMERA_ZOOMIN = 533,
    KEY_CAMERA_ZOOMOUT = 534,
    KEY_CAMERA_UP = 535,
    KEY_CAMERA_DOWN = 536,
    KEY_CAMERA_LEFT = 537,
    KEY_CAMERA_RIGHT = 538,
    KEY_ATTENDANT_ON = 539,
    KEY_ATTENDANT_OFF = 540,
    KEY_ATTENDANT_TOGGLE = 541,
    KEY_LIGHTS_TOGGLE = 542,
    KEY_543 = 543,
    BTN_DPAD_UP = 544,
    BTN_DPAD_DOWN = 545,
    BTN_DPAD_LEFT = 546,
    BTN_DPAD_RIGHT = 547,
    KEY_548 = 548,
    KEY_549 = 549,
    KEY_550 = 550,
    KEY_551 = 551,
    KEY_552 = 552,
    KEY_553 = 553,
    KEY_554 = 554,
    KEY_555 = 555,
    KEY_556 = 556,
    KEY_557 = 557,
    KEY_558 = 558,
    KEY_559 = 559,
    KEY_ALS_TOGGLE = 560,
    KEY_ROTATE_LOCK_TOGGLE = 561,
    KEY_562 = 562,
    KEY_563 = 563,
    KEY_564 = 564,
    KEY_565 = 565,
    KEY_566 = 566,
    KEY_567 = 567,
    KEY_568 = 568,
    KEY_569 = 569,
    KEY_570 = 570,
    KEY_571 = 571,
    KEY_572 = 572,
    KEY_573 = 573,
    KEY_574 = 574,
    KEY_575 = 575,
    KEY_BUTTONCONFIG = 576,
    KEY_TASKMANAGER = 577,
    KEY_JOURNAL = 578,
    KEY_CONTROLPANEL = 579,
    KEY_APPSELECT = 580,
    KEY_SCREENSAVER = 581,
    KEY_VOICECOMMAND = 582,
    KEY_ASSISTANT = 583,
    KEY_KBD_LAYOUT_NEXT = 584,
    KEY_585 = 585,
    KEY_586 = 586,
    KEY_587 = 587,
    KEY_588 = 588,
    KEY_589 = 589,
    KEY_590 = 590,
    KEY_591 = 591,
    KEY_BRIGHTNESS_MIN = 592,
    KEY_BRIGHTNESS_MAX = 593,
    KEY_594 = 594,
    KEY_595 = 595,
    KEY_596 = 596,
    KEY_597 = 597,
    KEY_598 = 598,
    KEY_599 = 599,
    KEY_600 = 600,
    KEY_601 = 601,
    KEY_602 = 602,
    KEY_603 = 603,
    KEY_604 = 604,
    KEY_605 = 605,
    KEY_606 = 606,
    KEY_607 = 607,
    KEY_KBDINPUTASSIST_PREV = 608,
    KEY_KBDINPUTASSIST_NEXT = 609,
    KEY_KBDINPUTASSIST_PREVGROUP = 610,
    KEY_KBDINPUTASSIST_NEXTGROUP = 611,
    KEY_KBDINPUTASSIST_ACCEPT = 612,
    KEY_KBDINPUTASSIST_CANCEL = 613,
    KEY_RIGHT_UP = 614,
    KEY_RIGHT_DOWN = 615,
    KEY_LEFT_UP = 616,
    KEY_LEFT_DOWN = 617,
    KEY_ROOT_MENU = 618,
    KEY_MEDIA_TOP_MENU = 619,
    KEY_NUMERIC_11 = 620,
    KEY_NUMERIC_12 = 621,
    KEY_AUDIO_DESC = 622,
    KEY_3D_MODE = 623,
    KEY_NEXT_FAVORITE = 624,
    KEY_STOP_RECORD = 625,
    KEY_PAUSE_RECORD = 626,
    KEY_VOD = 627,
    KEY_UNMUTE = 628,
    KEY_FASTREVERSE = 629,
    KEY_SLOWREVERSE = 630,
    KEY_DATA = 631,
    KEY_ONSCREEN_KEYBOARD = 632,
    KEY_633 = 633,
    KEY_634 = 634,
    KEY_635 = 635,
    KEY_636 = 636,
    KEY_637 = 637,
    KEY_638 = 638,
    KEY_639 = 639,
    KEY_640 = 640,
    KEY_641 = 641,
    KEY_642 = 642,
    KEY_643 = 643,
    KEY_644 = 644,
    KEY_645 = 645,
    KEY_646 = 646,
    KEY_647 = 647,
    KEY_648 = 648,
    KEY_649 = 649,
    KEY_650 = 650,
    KEY_651 = 651,
    KEY_652 = 652,
    KEY_653 = 653,
    KEY_654 = 654,
    KEY_655 = 655,
    KEY_656 = 656,
    KEY_657 = 657,
    KEY_658 = 658,
    KEY_659 = 659,
    KEY_660 = 660,
    KEY_661 = 661,
    KEY_662 = 662,
    KEY_663 = 663,
    KEY_664 = 664,
    KEY_665 = 665,
    KEY_666 = 666,
    KEY_667 = 667,
    KEY_668 = 668,
    KEY_669 = 669,
    KEY_670 = 670,
    KEY_671 = 671,
    KEY_672 = 672,
    KEY_673 = 673,
    KEY_674 = 674,
    KEY_675 = 675,
    KEY_676 = 676,
    KEY_677 = 677,
    KEY_678 = 678,
    KEY_679 = 679,
    KEY_680 = 680,
    KEY_681 = 681,
    KEY_682 = 682,
    KEY_683 = 683,
    KEY_684 = 684,
    KEY_685 = 685,
    KEY_686 = 686,
    KEY_687 = 687,
    KEY_688 = 688,
    KEY_689 = 689,
    KEY_690 = 690,
    KEY_691 = 691,
    KEY_692 = 692,
    KEY_693 = 693,
    KEY_694 = 694,
    KEY_695 = 695,
    KEY_696 = 696,
    KEY_697 = 697,
    KEY_698 = 698,
    KEY_699 = 699,
    KEY_700 = 700,
    KEY_701 = 701,
    KEY_702 = 702,
    KEY_703 = 703,
    BTN_TRIGGER_HAPPY1 = 704,
    BTN_TRIGGER_HAPPY2 = 705,
    BTN_TRIGGER_HAPPY3 = 706,
    BTN_TRIGGER_HAPPY4 = 707,
    BTN_TRIGGER_HAPPY5 = 708,
    BTN_TRIGGER_HAPPY6 = 709,
    BTN_TRIGGER_HAPPY7 = 710,
    BTN_TRIGGER_HAPPY8 = 711,
    BTN_TRIGGER_HAPPY9 = 712,
    BTN_TRIGGER_HAPPY10 = 713,
    BTN_TRIGGER_HAPPY11 = 714,
    BTN_TRIGGER_HAPPY12 = 715,
    BTN_TRIGGER_HAPPY13 = 716,
    BTN_TRIGGER_HAPPY14 = 717,
    BTN_TRIGGER_HAPPY15 = 718,
    BTN_TRIGGER_HAPPY16 = 719,
    BTN_TRIGGER_HAPPY17 = 720,
    BTN_TRIGGER_HAPPY18 = 721,
    BTN_TRIGGER_HAPPY19 = 722,
    BTN_TRIGGER_HAPPY20 = 723,
    BTN_TRIGGER_HAPPY21 = 724,
    BTN_TRIGGER_HAPPY22 = 725,
    BTN_TRIGGER_HAPPY23 = 726,
    BTN_TRIGGER_HAPPY24 = 727,
    BTN_TRIGGER_HAPPY25 = 728,
    BTN_TRIGGER_HAPPY26 = 729,
    BTN_TRIGGER_HAPPY27 = 730,
    BTN_TRIGGER_HAPPY28 = 731,
    BTN_TRIGGER_HAPPY29 = 732,
    BTN_TRIGGER_HAPPY30 = 733,
    BTN_TRIGGER_HAPPY31 = 734,
    BTN_TRIGGER_HAPPY32 = 735,
    BTN_TRIGGER_HAPPY33 = 736,
    BTN_TRIGGER_HAPPY34 = 737,
    BTN_TRIGGER_HAPPY35 = 738,
    BTN_TRIGGER_HAPPY36 = 739,
    BTN_TRIGGER_HAPPY37 = 740,
    BTN_TRIGGER_HAPPY38 = 741,
    BTN_TRIGGER_HAPPY39 = 742,
    BTN_TRIGGER_HAPPY40 = 743,
    BTN_MAX = 744,
    KEY_MAX = 767,
}

impl TryFrom<usize> for OsCode {
    type Error = ();
    fn try_from(item: usize) -> Result<Self, Self::Error> {
        match Self::from_u16(item as u16) {
            Some(kc) => Ok(kc),
            _ => Err(()),
        }
    }
}

impl From<u32> for OsCode {
    fn from(item: u32) -> Self {
        Self::from_u16(item as u16).unwrap_or_else(|| panic!("Invalid KeyCode: {item}"))
    }
}

impl From<u16> for OsCode {
    fn from(item: u16) -> Self {
        Self::from_u16(item).unwrap_or_else(|| panic!("Invalid KeyCode: {item}"))
    }
}

impl From<OsCode> for usize {
    fn from(item: OsCode) -> Self {
        item.as_u16() as usize
    }
}

impl From<OsCode> for u32 {
    fn from(item: OsCode) -> Self {
        item.as_u16() as u32
    }
}

impl From<OsCode> for u16 {
    fn from(item: OsCode) -> Self {
        item.as_u16()
    }
}

impl From<&OsCode> for KeyCode {
    fn from(item: &OsCode) -> KeyCode {
        (*item).into()
    }
}
impl From<&KeyCode> for OsCode {
    fn from(item: &KeyCode) -> Self {
        (*item).into()
    }
}

// ------------------ KeyValue --------------------

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum KeyValue {
    Release = 0,
    Press = 1,
    Repeat = 2,
}

impl From<i32> for KeyValue {
    fn from(item: i32) -> Self {
        match item {
            0 => Self::Release,
            1 => Self::Press,
            2 => Self::Repeat,
            _ => unreachable!(),
        }
    }
}

impl From<bool> for KeyValue {
    fn from(up: bool) -> Self {
        match up {
            true => Self::Release,
            false => Self::Press,
        }
    }
}

impl From<KeyValue> for bool {
    fn from(val: KeyValue) -> Self {
        matches!(val, KeyValue::Release)
    }
}
#[derive(Debug, Clone, Copy)]
pub struct KeyEvent {
    pub code: OsCode,
    pub value: KeyValue,
}

#[cfg(not(all(feature = "interception_driver", target_os = "windows")))]
impl KeyEvent {
    pub fn new(code: OsCode, value: KeyValue) -> Self {
        Self { code, value }
    }
}
