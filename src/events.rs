use anyhow::Result;
use crossterm::event::{self, Event, KeyEventKind, MouseEventKind};
use std::time::Duration;

pub fn poll_events(timeout: Duration) -> Result<Option<Event>> {
    if event::poll(timeout)? {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                Ok(Some(Event::Key(key_event)))
            }
            Event::Mouse(mouse_event) => {
                // 只处理滚轮事件，其他鼠标事件忽略
                match mouse_event.kind {
                    MouseEventKind::ScrollUp | MouseEventKind::ScrollDown => {
                        Ok(Some(Event::Mouse(mouse_event)))
                    }
                    _ => Ok(None), // 忽略点击、移动等事件
                }
            }
            Event::Resize(width, height) => Ok(Some(Event::Resize(width, height))),
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}