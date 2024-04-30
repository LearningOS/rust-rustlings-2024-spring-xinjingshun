/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/
// I AM DONE
// 思路：使用队列实现栈的功能。使用两个队列 q1 和 q2，其中 q1 用于存储栈的元素，而 q2 则用于辅助操作。
#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::new(),
            q2: Queue::new(),
        }
    }

    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
    }

    /// 从栈中弹出一个元素。如果栈为空，则返回错误信息；否则返回栈顶元素。
    pub fn pop(&mut self) -> Result<T, &str> {
        // 检查栈是否为空，如果是，则返回错误信息
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }

        // 从 q1 中逐个弹出元素，直到 q1 中只剩下最后一个元素
        while let Some(elem) = self.q1.dequeue().ok() {
            // 如果 q1 中还有元素，将其加入到 q2 中
            if self.q1.is_empty() {
                // 当 q1 只剩下最后一个元素时，交换 q1 和 q2 的角色
                std::mem::swap(&mut self.q1, &mut self.q2);
                // 返回栈顶元素
                return Ok(elem);
            } else {
                self.q2.enqueue(elem);
            }
        }

        // 如果程序执行到这里，表示栈为空，这应该是不可达的
        // 在循环中使用了 unreachable!() 宏，这表示如果程序执行到了循环外部的这个位置，说明栈应该是空的，因为所有的元素都已经被弹出了。
        unreachable!("Stack is empty")
    }


    pub fn is_empty(&self) -> bool {
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut s = MyStack::new();
        assert_eq!(s.pop(), Err("Stack is empty"));

        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
