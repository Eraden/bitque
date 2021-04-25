use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;

use futures::StreamExt;
use seed::prelude::*;
use seed::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Default)]
pub struct Distinct {
    state: Rc<RefCell<HashSet<&'static str>>>,
    locks: Rc<RefCell<HashMap<&'static str, bool>>>,
}

impl Distinct {
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(Default::default())),
            locks: Rc::new(RefCell::new(Default::default())),
        }
    }

    pub fn keyup_wih_reset<Cb>(&self, selector: &'static str, submit_after_frames: usize, cb: Cb)
    where
        Cb: Fn(web_sys::KeyboardEvent) + 'static,
    {
        if self.state.borrow().contains(selector) {
            return;
        }
        self.state.borrow_mut().insert(selector);

        let callback = Rc::new(cb);
        let locks = self.locks.clone();

        on("keyup", move |ev| {
            let active = match seed::document().active_element() {
                None => {
                    return;
                }
                Some(el) => el,
            };
            if !active.class_name().contains(selector) {
                return;
            }
            let locks = locks.clone();
            if locks.borrow().get(selector).cloned().unwrap_or_default() {
                return;
            }
            locks.borrow_mut().insert(selector, true);
            let callback = callback.clone();

            keyup_wih_reset(
                selector,
                move || {
                    let locks = locks.clone();
                    locks.borrow_mut().remove(selector);
                    let callback = callback.clone();
                    callback(ev);
                },
                submit_after_frames,
            );
        });
    }
}

impl Clone for Distinct {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            locks: self.locks.clone(),
        }
    }
}

pub fn distinct() -> Distinct {
    Distinct::new()
}

pub fn keydown<Cb>(cb: Cb)
where
    Cb: FnMut(web_sys::KeyboardEvent) + 'static,
{
    on("keydown", cb);
}

fn keyup_wih_reset<Cb>(selector: &'static str, cb: Cb, submit_after_frames: usize)
where
    Cb: FnOnce() + 'static,
{
    use std::sync::atomic::Ordering;
    let (up_sender, mut up_receiver) = ::futures::channel::mpsc::unbounded();
    let n: Rc<AtomicUsize> = Rc::new(AtomicUsize::new(submit_after_frames));

    let reset = n.clone();
    spawn_local(async move {
        while up_receiver.next().await.is_some() {
            reset.store(submit_after_frames, Ordering::Relaxed);
        }
    });

    spawn_local(async move {
        loop {
            if n.load(Ordering::Relaxed) == 0 {
                break;
            }
            wait_frame().await;
            let _ = n.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| Some(v - 1));
        }

        cb();
    });

    on("keyup", move |_ev| {
        let active = match seed::document().active_element() {
            None => return,
            Some(el) => el,
        };
        if !active.class_name().contains(selector) {
            return;
        }
        let _ = up_sender.unbounded_send(());
    });
}

pub fn keypress<Cb>(cb: Cb)
where
    Cb: FnMut(web_sys::KeyboardEvent) + 'static,
{
    on("keypress", cb);
}

pub fn on<Cb>(event: &str, cb: Cb)
where
    Cb: FnMut(web_sys::KeyboardEvent) + 'static,
{
    let handler = Closure::wrap(Box::new(cb) as Box<dyn FnMut(_)>);
    seed::window()
        .add_event_listener_with_callback(event, handler.as_ref().unchecked_ref())
        .expect("Failed to mount global key handler");
    handler.forget();
}

pub async fn wait_frame() -> f64 {
    let (sender, mut receiver) = ::futures::channel::mpsc::unbounded();
    let handler = Closure::wrap(Box::new(move |f| {
        let _ = sender.unbounded_send(f);
    }) as Box<dyn FnMut(f64)>);
    let _ = seed::window().request_animation_frame(handler.as_ref().unchecked_ref());
    handler.forget();
    receiver.next().await.unwrap_or_default()
}
