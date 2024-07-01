use std::collections::VecDeque;

// 定义事件类型枚举
#[derive(PartialEq, Eq)]
pub enum Event {
    BUTTON,
    SystemShow,
    Dht11Show
}

// 事件队列结构体
pub struct EventQueue {
    queue: VecDeque<Event>,
}

impl EventQueue {
    // 创建一个新的事件队列
    pub fn new() -> Self {
        EventQueue {
            queue: VecDeque::new(),
        }
    }

    // 向队列添加事件
    pub fn push(&mut self, event: Event) {
        self.queue.push_back(event);
    }

    // 从队列取出一个事件
    pub fn pop(&mut self) -> Option<Event> {
        self.queue.pop_front()
    }
}