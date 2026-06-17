# Logs

## PT-pt / `CTRL+~`

When hitting `CTRL+~`, I saw the following logs:

```
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(37) key_utf32=0 key_utf8="" key_sym=XK_Control_L key_index=0
[dead-key-debug] wl_keyboard key event keycode=KeyCode(37) keysym=Control_L key=control_l key_char=None pre_edit_text=None
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(51) key_utf32=28 key_utf8="\u{1c}" key_sym=XK_dead_tilde key_index=1
[dead-key-debug] wl_keyboard key event keycode=KeyCode(51) keysym=dead_tilde key=\ key_char=None pre_edit_text=None
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(51) key_utf32=28 key_utf8="\u{1c}" key_sym=XK_dead_tilde key_index=1
[dead-key-debug] wl_keyboard press before compose keycode=KeyCode(51) keysym=dead_tilde modifiers=Modifiers { control: true, alt: false, shift: false, platform: false, function: false } key=\ key_char=None pre_edit_text=Some("~")
[dead-key-debug] keydown keysym=dead_tilde key=\ key_char=None pre_edit_text=Some("~")
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(51) key_utf32=28 key_utf8="\u{1c}" key_sym=XK_dead_tilde key_index=1
[dead-key-debug] wl_keyboard key event keycode=KeyCode(51) keysym=dead_tilde key=\ key_char=None pre_edit_text=Some("~")
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(51) key_utf32=28 key_utf8="\u{1c}" key_sym=XK_dead_tilde key_index=1
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(37) key_utf32=0 key_utf8="" key_sym=XK_Control_L key_index=0
[dead-key-debug] wl_keyboard key event keycode=KeyCode(37) keysym=Control_L key=control_l key_char=None pre_edit_text=Some("~")
```

## PT-pt / `CTRL+\`

```
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(37) key_utf32=0 key_utf8="" key_sym=XK_Control_L key_index=0
[dead-key-debug] wl_keyboard key event keycode=KeyCode(37) keysym=Control_L key=control_l key_char=None pre_edit_text=None
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(49) key_utf32=28 key_utf8="\u{1c}" key_sym=XK_backslash key_index=1
[dead-key-debug] wl_keyboard key event keycode=KeyCode(49) keysym=backslash key=\ key_char=None pre_edit_text=None
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(49) key_utf32=28 key_utf8="\u{1c}" key_sym=XK_backslash key_index=1
[dead-key-debug] compose status=Nothing keysym=backslash key=\ key_char=None pre_edit_text=None
[dead-key-debug] keydown keysym=backslash key=\ key_char=None pre_edit_text=None
[dead-key-debug] text_input_v3 Done serial=44 ime_pre_edit=None pre_edit_text=None composing=false
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(49) key_utf32=28 key_utf8="\u{1c}" key_sym=XK_backslash key_index=1
[dead-key-debug] wl_keyboard key event keycode=KeyCode(49) keysym=backslash key=\ key_char=None pre_edit_text=None
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(49) key_utf32=28 key_utf8="\u{1c}" key_sym=XK_backslash key_index=1
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(37) key_utf32=0 key_utf8="" key_sym=XK_Control_L key_index=0
[dead-key-debug] wl_keyboard key event keycode=KeyCode(37) keysym=Control_L key=control_l key_char=None pre_edit_text=None
```

## EN-us / `CTRL+~`

```
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(37) key_utf32=0 key_utf8="" key_sym=XK_Control_L key_index=0
[dead-key-debug] wl_keyboard key event keycode=KeyCode(37) keysym=Control_L key=control_l key_char=None pre_edit_text=None
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(51) key_utf32=28 key_utf8="\u{1c}" key_sym=XK_backslash key_index=0
[dead-key-debug] wl_keyboard key event keycode=KeyCode(51) keysym=backslash key=\ key_char=None pre_edit_text=None
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(51) key_utf32=28 key_utf8="\u{1c}" key_sym=XK_backslash key_index=0
[dead-key-debug] compose status=Nothing keysym=backslash key=\ key_char=None pre_edit_text=None
[dead-key-debug] keydown keysym=backslash key=\ key_char=None pre_edit_text=None
[dead-key-debug] text_input_v3 Done serial=54 ime_pre_edit=None pre_edit_text=None composing=false
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(51) key_utf32=28 key_utf8="\u{1c}" key_sym=XK_backslash key_index=0
[dead-key-debug] wl_keyboard key event keycode=KeyCode(51) keysym=backslash key=\ key_char=None pre_edit_text=None
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(51) key_utf32=28 key_utf8="\u{1c}" key_sym=XK_backslash key_index=0
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(37) key_utf32=0 key_utf8="" key_sym=XK_Control_L key_index=0
[dead-key-debug] wl_keyboard key event keycode=KeyCode(37) keysym=Control_L key=control_l key_char=None pre_edit_text=None
```

## EN-us / `CTRL+\`

```
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(37) key_utf32=0 key_utf8="" key_sym=XK_Control_L key_index=0
[dead-key-debug] wl_keyboard key event keycode=KeyCode(37) keysym=Control_L key=control_l key_char=None pre_edit_text=None
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(49) key_utf32=0 key_utf8="\0" key_sym=XK_grave key_index=0
[dead-key-debug] wl_keyboard key event keycode=KeyCode(49) keysym=grave key=` key_char=None pre_edit_text=None
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(49) key_utf32=0 key_utf8="\0" key_sym=XK_grave key_index=0
[dead-key-debug] compose status=Nothing keysym=grave key=` key_char=None pre_edit_text=None
[dead-key-debug] keydown keysym=grave key=` key_char=None pre_edit_text=None
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(49) key_utf32=0 key_utf8="\0" key_sym=XK_grave key_index=0
[dead-key-debug] wl_keyboard key event keycode=KeyCode(49) keysym=grave key=` key_char=None pre_edit_text=None
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(49) key_utf32=0 key_utf8="\0" key_sym=XK_grave key_index=0
[dead-key-debug] text_input_v3 Done serial=58 ime_pre_edit=None pre_edit_text=None composing=false
[dead-key-debug] keystroke_from_kxb keycode=KeyCode(37) key_utf32=0 key_utf8="" key_sym=XK_Control_L key_index=0
[dead-key-debug] wl_keyboard key event keycode=KeyCode(37) keysym=Control_L key=control_l key_char=None pre_edit_text=None
```
