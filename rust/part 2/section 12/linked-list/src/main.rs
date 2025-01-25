// #[derive(Debug)]
// struct LinkedList {
//     head: Pointer,
// }
//
// #[derive(Debug)]
// struct Node {
//     element: i32,
//     next: Pointer,
// }
//
// type Pointer = Option<Box<Node>>;
//
// impl LinkedList {
//     fn new() -> LinkedList {
//         LinkedList { head: None }
//     }
//
//     fn add(&mut self, element: i32) {
//         //Does not work
//         // match self.head {
//         //     None => {
//         //         let new_node = Some(Box::new(Node {
//         //             element,
//         //             next: None,
//         //         }));
//         //         self.head = new_node;
//         //     }
//         //     Some(prev_node) => {
//         //         let new_node = Some(Box::new(Node{element, next: Some(prev_node)}));
//         //         self.head = new_node;
//         //     }
//         // }
//
//         let pre_node = self.head.take();
//         let new_head = Some(Box::new(Node {
//             element,
//             next: pre_node,
//         }));
//         self.head = new_head;
//     }
//
//     fn remove(&mut self) -> Option<i32> {
//         match self.head.take() {
//             Some(pre_head)=>{
//                 self.head = pre_head.next;
//                 Some(pre_head.element)
//             }
//             None => None,
//         }
//     }
//
//     fn print(&self) {
//         let mut list_traversal = &self.head;
//         while !list_traversal.is_none() {
//             println!("{}", list_traversal.as_ref().unwrap().element);
//             list_traversal = &list_traversal.as_ref().unwrap().next;
//         }
//     }
// }
//
// fn main() {
//     let mut list = LinkedList::new();
//     list.add(1);
//     list.add(10);
//     list.add(5);
//     list.add(2);
//     list.add(303);
//     // let list = LinkedList {
//     //     head: Some(Box::new(Node {
//     //         element: 1,
//     //         next: Some(Box::new(Node {
//     //             element: 2,
//     //             next: None,
//     //         })),
//     //     })),
//     // };
//     //
//     // println!("{:#?}", &list);
//     // println!("{:#?}", &list.remove());
//     list.print();
//     // list.remove();
//     // println!("{:#?}", &list.head.take().unwrap());
// }

//Problem 1: Implement a peek method on the Linklist.
// The signature of the function is given in the code below.
// This method will return the head value if it exist.

// #[derive(Debug)]
// struct Linklist {
//     head: pointer,
// }
//
// #[derive(Debug)]
// struct Node {
//     element: i32,
//     next: pointer,
// }
// type pointer = Option<Box<Node>>;
//
// impl Linklist {
//     fn new() -> Linklist {
//         Linklist { head: None }
//     }
//
//     fn add(&mut self, element: i32) {
//         let previous_head = self.head.take();
//         let new_head = Some(Box::new(Node {
//             element: element,
//             next: previous_head,
//         }));
//         self.head = new_head;
//     }
//
//     fn remove(&mut self) -> Option<i32> {
//         match self.head.take() {
//             Some(previous_head) => {
//                 self.head = previous_head.next;
//                 Some(previous_head.element)
//             }
//             None => None,
//         }
//     }
//
//     fn peek(&self) -> Option<i32> {
//         match &self.head {
//             Some(h) => Some(h.element),
//             None => None,
//         }
//     }
//
//     fn print(&self) {
//         let mut list_traversal = &self.head;
//         while !list_traversal.is_none() {
//             println!("{:?}", list_traversal.as_ref().unwrap().element);
//             list_traversal = &list_traversal.as_ref().unwrap().next;
//         }
//     }
// }
// fn main() {
//     let mut list = Linklist::new();
//     list.add(5);
//     list.add(7);
//     list.add(10);
//     list.add(15);
//     list.add(20);
//
//     println!("{:?}", list.peek());
// }

//Problem 2: We want to change the linked list implementation by making the element part of
// the node as generic rather then concrete i32. Make approperiate changes to the code below.
// For printing, T should have the trait bound of  std::fmt::Debug and
// for the peek to work, T must also have the trait bound of std::marker::Copy

// #[derive(Debug)]
// struct Linklist<T> {
//     // This line needs a fix
//     head: pointer<T>, // This line needs a fix
// }
//
// #[derive(Debug)]
// struct Node<T> {
//     element: T,
//     next: pointer<T>, // This line needs a fix
// }
// type pointer<T> = Option<Box<Node<T>>>; // This line needs a fix
//
// impl<T: std::fmt::Debug + std::marker::Copy> Linklist<T> {
//     // This line needs a fix
//     fn new() -> Linklist<T> {
//         // This line needs a fix
//         Linklist { head: None }
//     }
//
//     fn add(&mut self, element: T) {
//         // This line needs a fix
//         let previous_head = self.head.take();
//         let new_head = Some(Box::new(Node {
//             element: element,
//             next: previous_head,
//         }));
//         self.head = new_head;
//     }
//
//     fn remove(&mut self) -> Option<T> {
//         // This line needs a fix
//         match self.head.take() {
//             Some(previous_head) => {
//                 self.head = previous_head.next;
//                 Some(previous_head.element)
//             }
//             None => None,
//         }
//     }
//
//     fn peek(&self) -> Option<T> {
//         // This line needs a fix
//         match &self.head {
//             Some(h) => Some(h.element),
//             None => None,
//         }
//     }
//
//     fn print(&self) {
//         let mut list_traversal = &self.head;
//         while !list_traversal.is_none() {
//             println!("{:?}", list_traversal.as_ref().unwrap().element);
//             list_traversal = &list_traversal.as_ref().unwrap().next;
//         }
//     }
// }
// fn main() {
//     let mut list = Linklist::new();
//     list.add(5);
//     list.add(7);
//     list.add(10);
//     list.add(15);
//     list.add(20);
//
//     println!("{:?}", list.peek());
// }

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct DoublyLinkedList {
    head: Pointer,
    tail: Pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: Pointer,
    previous: Pointer,
}

impl Node {
    fn new(element: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            element,
            next: None,
            previous: None,
        }))
    }
}
type Pointer = Option<Rc<RefCell<Node>>>;

impl DoublyLinkedList {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    fn add(&mut self, element: i32) {
        let new_head = Node::new(element);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().previous = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }

            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn add_back(&mut self, element: i32) {
        let new_tail = Node::new(element);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().previous = Some(old_tail.clone());
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    fn remove(&mut self) -> Option<i32> {
        if self.head.is_none() {
            println!("The List is empty!");
            None
        } else {
            let removed_val = self.head.as_ref().unwrap().borrow().element;
            self.head
                .take()
                .map(|old_head| match old_head.borrow_mut().next.take() {
                    Some(old_head) => {
                        old_head.borrow_mut().previous = None;
                        self.head = Some(old_head);
                        self.head.clone()
                    }
                    None => {
                        self.tail = None;
                        println!("The List is empty after removal!");
                        None
                    }
                });
            Some(removed_val)
        }
    }

    fn remove_back(&mut self) -> Option<i32> {
        if self.tail.is_none() {
            println!("The List is empty!");
            None
        }else{
            let removed_val = self.tail.as_ref().unwrap().borrow().element;
            self.tail.take().map(|old_tail| {
                match old_tail.borrow_mut().previous.take() {
                    Some(old_tail) => {
                        old_tail.borrow_mut().next = None;
                        self.tail= Some(old_tail);
                        self.tail.clone()
                    }
                    None=>{
                        self.tail = None;
                        println!("The List is empty after removal!");
                        None
                    }
                }
            });
            Some(removed_val)
        }
    }

    fn print(&self) {
        let mut traversal = self.head.clone();
        while !traversal.is_none() {
            println!("traversal element: {}", traversal.as_ref().unwrap().borrow().element);
            traversal = traversal.unwrap().borrow().next.clone();
        }
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();
    list.add(1);
    list.add(2);
    list.add(3);
    list.add(4);
    list.print();
    list.remove_back();
    list.remove_back();
    println!("<<<---->>>");
    list.print();
}
