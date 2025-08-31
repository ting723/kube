use anyhow::Result;
use crossterm::event::{self, Event, KeyEventKind};
use std::time::Duration;

pub fn poll_events(timeout: Duration) -> Result<Option<Event>> {
    if event::poll(timeout)? {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                Ok(Some(Event::Key(key_event)))
            }
            Event::Mouse(mouse_event) => {
                // 传递所有鼠标事件，但在app中只处理滚轮事件
                // 这样可以保持文本选中复制功能
                Ok(Some(Event::Mouse(mouse_event)))
            }
            Event::Resize(width, height) => Ok(Some(Event::Resize(width, height))),
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}