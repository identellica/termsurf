//! CEF and Chromium support a wide range of command-line switches. This file
//! only contains command-line switches specific to the cefclient application.
//! View CEF/Chromium documentation or search for *_switches.cc files in the
//! Chromium source code to identify other existing command-line switches.

pub const MULTI_THREADED_MESSAGE_LOOP: &str = "multi-threaded-message-loop";
pub const EXTERNAL_MESSAGE_PUMP: &str = "external-message-pump";
pub const CACHE_PATH: &str = "cache-path";
pub const URL: &str = "url";
pub const OFF_SCREEN_RENDERING_ENABLED: &str = "off-screen-rendering-enabled";
pub const OFF_SCREEN_FRAME_RATE: &str = "off-screen-frame-rate";
pub const TRANSPARENT_PAINTING_ENABLED: &str = "transparent-painting-enabled";
pub const SHOW_UPDATE_RECT: &str = "show-update-rect";
pub const FAKE_SCREEN_BOUNDS: &str = "fake-screen-bounds";
pub const SHARED_TEXTURE_ENABLED: &str = "shared-texture-enabled";
pub const EXTERNAL_BEGIN_FRAME_ENABLED: &str = "external-begin-frame-enabled";
pub const MOUSE_CURSOR_CHANGE_DISABLED: &str = "mouse-cursor-change-disabled";
pub const OFFLINE: &str = "offline";
pub const FILTER_CHROME_COMMANDS: &str = "filter-chrome-commands";
pub const REQUEST_CONTEXT_PER_BROWSER: &str = "request-context-per-browser";
pub const REQUEST_CONTEXT_SHARED_CACHE: &str = "request-context-shared-cache";
pub const BACKGROUND_COLOR: &str = "background-color";
pub const ENABLE_GPU: &str = "enable-gpu";
pub const FILTER_URL: &str = "filter-url";
pub const USE_VIEWS: &str = "use-views";
pub const USE_NATIVE: &str = "use-native";
pub const HIDE_FRAME: &str = "hide-frame";
pub const HIDE_CONTROLS: &str = "hide-controls";
pub const HIDE_OVERLAYS: &str = "hide-overlays";
pub const ALWAYS_ON_TOP: &str = "always-on-top";
pub const HIDE_TOP_MENU: &str = "hide-top-menu";
pub const SSL_CLIENT_CERTIFICATE: &str = "ssl-client-certificate";
pub const CRL_SETS_PATH: &str = "crl-sets-path";
pub const NO_ACTIVATE: &str = "no-activate";
pub const SHOW_CHROME_TOOLBAR: &str = "show-chrome-toolbar";
pub const INITIAL_SHOW_STATE: &str = "initial-show-state";
pub const USE_DEFAULT_POPUP: &str = "use-default-popup";
pub const USE_CLIENT_DIALOGS: &str = "use-client-dialogs";
pub const USE_TEST_HTTP_SERVER: &str = "use-test-http-server";
pub const SHOW_WINDOW_BUTTONS: &str = "show-window-buttons";
pub const USE_WINDOW_MODAL_DIALOG: &str = "use-window-modal-dialog";
pub const USE_BOTTOM_CONTROLS: &str = "use-bottom-controls";
pub const HIDE_PIP_FRAME: &str = "hide-pip-frame";
pub const MOVE_PIP_ENABLED: &str = "move-pip-enabled";
pub const HIDE_CHROME_BUBBLES: &str = "hide-chrome-bubbles";
pub const HIDE_WINDOW_ON_CLOSE: &str = "hide-window-on-close";
pub const ACCEPTS_FIRST_MOUSE: &str = "accepts-first-mouse";
pub const USE_ALLOY_STYLE: &str = "use-alloy-style";
pub const USE_CHROME_STYLE_WINDOW: &str = "use-chrome-style-window";
pub const SHOW_OVERLAY_BROWSER: &str = "show-overlay-browser";
pub const USE_ANGLE: &str = "use-angle";
pub const OZONE_PLATFORM: &str = "ozone-platform";
