/*
	queue
	This question requires you to use queues to implement the functionality of the stack
*/

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

pub struct myStack<T>
{
    q1:Queue<T>,
    q2:Queue<T>,
    is_q1: bool,
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1:Queue::<T>::new(),
            q2:Queue::<T>::new(),
            is_q1:true
        }
    }

    pub fn push(&mut self, elem: T) {
        let q = self.cur_queue();
        q.enqueue(elem);
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        if self.is_empty() {
            return Err("Stack is empty")
        }

        let mut last = Err("Not Found");
        while let Ok(first) = self.cur_queue().dequeue() {
            if !self.is_empty() {
                self.cur_backup_queue().enqueue(first);
            } else {
                last = Ok(first)
            }
        }

        self.switch_queue();

        return last;
    }

    pub fn is_empty(&self) -> bool {
        self.cur_queue_ro().is_empty()
    }

    fn switch_queue(&mut self) {
        self.is_q1 = !self.is_q1;
    }

    fn cur_queue_ro(&self) -> &Queue<T> {
        if self.is_q1 {
            return &self.q1
        } else {
            return &self.q2
        }
    }

    fn cur_queue(&mut self) -> &mut Queue<T> {
        if self.is_q1 {
            return &mut self.q1
        } else {
            return &mut self.q2
        }
    }

    fn cur_backup_queue(&mut self) -> &mut Queue<T> {
        if !self.is_q1 {
            return &mut self.q1
        } else {
            return &mut self.q2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue(){
        let mut s = myStack::<i32>::new();
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
