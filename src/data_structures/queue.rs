// // // FIFO
// // pub trait QueueTrait<'a, T> {
// //     fn new(
// //         head: &'a Option<&'a mut QueueType<'a, T>>,
// //         tail: &'a Option<&'a mut QueueType<'a, T>>,
// //         length: usize,
// //     ) -> Queue<'a, T>;
// //     fn peak(&self) -> &Option<&mut QueueType<T>>;
// //     fn push(&mut self, queue_item: &'a mut Option<&'a mut QueueType<'a, T>>);
// // }

// // pub struct QueueType<'a, T: 'a> {
// //     pub value: T,
// //     pub next: &'a mut Option<&'a mut QueueType<'a, T>>,
// // }

// // impl<'a, T: 'a> QueueType<'a, T> {
// //     pub fn new(value: T, next: &'a mut Option<&'a mut QueueType<'a, T>>) -> Self {
// //         Self { value, next }
// //     }
// // }

// // pub struct Queue<'b, T: 'b> {
// //     pub head: &'b Option<&'b mut QueueType<'b, T>>,
// //     pub tail: &'b Option<&'b mut QueueType<'b, T>>,
// //     pub length: usize,
// // }

// // impl<'a, T: 'a> QueueTrait<'a, T> for Queue<'a, T> {
// //     fn new(
// //         head: &'a Option<&'a mut QueueType<'a, T>>,
// //         tail: &'a Option<&'a mut QueueType<'a, T>>,
// //         length: usize,
// //     ) -> Self {
// //         Self { head, tail, length }
// //     }

// //     fn peak(&self) -> &Option<&mut QueueType<T>> {
// //         self.head
// //     }

// //     fn push(&mut self, queue_item: &'a mut Option<&'a mut QueueType<'a, T>>) {
// //         if self.head.is_none() {
// //             self.head = queue_item;
// //         }
// //         match self.tail {
// //             Some(tail) => {
// //                 tail.next = queue_item;
// //                 self.tail = queue_item;
// //                 //tail.next = queue_item;
// //                 // self.tail = queue_item;
// //             }
// //             None => todo!(),
// //         }
// //     }
// // }

// // #[cfg(test)]
// // mod queue_test {
// //     use super::*;

// //     #[test]
// //     fn peak_test() {
// //         // let mut new_queue: Queue<i32> = Queue::new();
// //         // let new_value = &QueueType::new(48);
// //         // new_queue.head = Some(new_value);
// //         // assert_eq!(48, new_queue.peak().unwrap().value);
// //     }
// // }

// pub struct QueueType<'a, T> {
//     data: T,
//     next: &'a T,
// }

// impl<'a, T> QueueType<'a, T> {
//     pub fn new(data: T, next: &'a T) -> Self {
//         Self { data, next }
//     }
// }

// pub struct Queue<'a, T> {
//     pub head: &'a Option<QueueType<'a, T>>,
//     pub tail: &'a Option<QueueType<'a, T>>,
//     pub length: usize,
// }

// impl<'a, T> Queue<'a, T> {
//     pub fn new() -> Self {
//         Self {
//             head: &None,
//             tail: &None,
//             length: 0,
//         }
//     }

//     pub fn peak(&self) -> &Option<QueueType<'a, T>> {
//         &self.head
//     }

//     pub fn add_to_queue(&mut self, queue_item: Option<QueueType<'a, T>>) {
//         if self.head.is_none() {
//             self.head = &queue_item;
//         }

//         if self.tail.is_none() {
//             self.tail.unwrap().next =
//         }
//     }
// }
