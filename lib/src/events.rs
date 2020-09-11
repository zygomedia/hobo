//! everything that has to do with HTML event handling

use crate::{prelude::*, Element, storage::Storage};

pub enum EventHandler {
	MouseEvent(Closure<dyn FnMut(web_sys::MouseEvent) + 'static>),
	KeyboardEvent(Closure<dyn FnMut(web_sys::KeyboardEvent) + 'static>),
	Event(Closure<dyn FnMut(web_sys::Event) + 'static>),
	FocusEvent(Closure<dyn FnMut(web_sys::FocusEvent) + 'static>),
	TouchEvent(Closure<dyn FnMut(web_sys::TouchEvent) + 'static>),

	// AnimationEvent
	// AnimationPlaybackEvent
	// DeviceMotionEvent
	// DeviceOrientationEvent
	// DeviceProximityEvent
	// DragEvent
	// ErrorEvent
	// ExtendableEvent
	// ExtendableMessageEvent
	// FetchEvent
	// AudioProcessingEvent
	// FontFaceSetLoadEvent
	// GamepadAxisMoveEvent
	// GamepadButtonEvent
	// GamepadEvent
	// GpuUncapturedErrorEvent
	// HashChangeEvent
	// IdbVersionChangeEvent
	// ImageCaptureErrorEvent
	// InputEvent
	// BeforeUnloadEvent
	// MediaEncryptedEvent
	// MediaKeyError
	// MediaKeyMessageEvent
	// MediaQueryListEvent
	// MediaRecorderErrorEvent
	// MediaStreamEvent
	// MediaStreamTrackEvent
	// MessageEvent
	// MidiConnectionEvent
	// MidiMessageEvent
	// BlobEvent
	// MouseScrollEvent
	// MutationEvent
	// NotificationEvent
	// OfflineAudioCompletionEvent
	// PageTransitionEvent
	// PaymentMethodChangeEvent
	// PaymentRequestUpdateEvent
	// PointerEvent
	// PopStateEvent
	// ClipboardEvent
	// PopupBlockedEvent
	// PresentationConnectionAvailableEvent
	// PresentationConnectionCloseEvent
	// ProgressEvent
	// PromiseRejectionEvent
	// PushEvent
	// RtcDataChannelEvent
	// RtcPeerConnectionIceEvent
	// RtcTrackEvent
	// RtcdtmfToneChangeEvent
	// CloseEvent
	// ScrollAreaEvent
	// SecurityPolicyViolationEvent
	// SpeechRecognitionError
	// SpeechRecognitionEvent
	// SpeechSynthesisErrorEvent
	// SpeechSynthesisEvent
	// StorageEvent
	// TcpServerSocketEvent
	// TcpSocketErrorEvent
	// TcpSocketEvent
	// CompositionEvent
	// TimeEvent
	// TrackEvent
	// TransitionEvent
	// UiEvent
	// UserProximityEvent
	// WebGlContextEvent
	// WheelEvent
	// XrInputSourceEvent
	// XrInputSourcesChangeEvent
	// CustomEvent
	// XrReferenceSpaceEvent
	// XrSessionEvent
	// DeviceLightEvent
}

macro_rules! generate_events {
	($($event_kind:ident, $name:ident, $f:ident);+$(;)*) => {paste::item!{
		impl Element {
			$(
				pub fn [<add_ $f>](self, mut f: impl FnMut(web_sys::$event_kind) + 'static) {
					if WORLD.is_dead(self.entity) { return; }
					if let Some(target) = WORLD.storage::<web_sys::EventTarget>().get(self.entity) {
						let handler = Closure::wrap(Box::new(move |e| f(e)) as Box<dyn FnMut(web_sys::$event_kind) + 'static>);
						target.add_event_listener_with_callback(web_str::$name(), handler.as_ref().unchecked_ref()).expect("can't add event listener");
						WORLD.storage_mut::<Vec<EventHandler>>().get_mut_or(self.entity, Vec::new).push(EventHandler::$event_kind(handler));
					}
				}

				pub fn $f(self, f: impl FnMut(web_sys::$event_kind) + 'static) -> Self { self.[<add_ $f>](f); self }
			)+
		}
	}};
}

generate_events! {
	MouseEvent,    click,       on_click;
	MouseEvent,    contextmenu, on_context_menu;
	MouseEvent,    dblclick,    on_dbl_click;
	MouseEvent,    mousedown,   on_mouse_down;
	MouseEvent,    mouseenter,  on_mouse_enter;
	MouseEvent,    mouseleave,  on_mouse_leave;
	MouseEvent,    mousemove,   on_mouse_move;
	MouseEvent,    mouseover,   on_mouse_over;
	MouseEvent,    mouseout,    on_mouse_out;
	MouseEvent,    mouseup,     on_mouse_up;
	KeyboardEvent, keydown,     on_key_down;
	KeyboardEvent, keyup,       on_key_up;
	Event,         change,      on_change;
	Event,         scroll,      on_scroll;
	FocusEvent,    blur,        on_blur;
	FocusEvent,    focus,       on_focus;
	TouchEvent,    touchstart,  on_touch_start;
	TouchEvent,    touchend,    on_touch_end;
	TouchEvent,    touchmove,   on_touch_move;
	TouchEvent,    touchcancel, on_touch_cancel;
}
