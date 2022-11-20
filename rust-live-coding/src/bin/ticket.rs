use tokio::sync::{Semaphore, SemaphorePermit};

// 获取许可
#[derive(Debug)]
pub struct Ticket<'a> {
    permit: SemaphorePermit<'a>,
}

impl<'a> Ticket<'a> {
    pub fn new(permit: SemaphorePermit<'a>) -> Self {
        Self { permit }
    }
}

impl<'a> Drop for Ticket<'a> {
    fn drop(&mut self) {
        println!("delete ticket")
    }
}


pub struct Museum {
    remaining_tickets: Semaphore,
}

impl Museum {
    // 初始化最大容量
    pub fn new(total: usize) -> Self {
        Self { remaining_tickets: Semaphore::new(total) }
    }

    pub fn get_ticket(&self) -> Option<Ticket<'_>> {
        match self.remaining_tickets.try_acquire() {
            Ok(ticket) => Some(Ticket::new(ticket)),
            Err(err) => None
        }
    }

    pub fn tickets(&self) -> usize {
        self.remaining_tickets.available_permits()
    }
}


// 实现一个有限队列
fn main() {
    let museum = Museum::new(50);
    let ticket = museum.get_ticket().unwrap();
    assert_eq!(museum.tickets(), 49);
    let _tickets: Vec<Ticket> = (0..49).map(|_i| museum.get_ticket().unwrap()).collect();
    assert_eq!(museum.tickets(), 0);

    assert!(museum.get_ticket().is_none());

    drop(ticket);
    {
        let ticket = museum.get_ticket().unwrap();
        println!("got ticket: {:?}", ticket);
    }
    println!("!!!!");
}