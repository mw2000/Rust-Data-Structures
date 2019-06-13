// Copyright 2019 Mihir Wadekar and Chaitenya Gupta

mod lib;

#[test]
fn empty() {
    let queue = lib::Queue::<i32>::new();

    // Testing for an empty queue
    
    assert_eq!(queue.size(), 0);
    assert_eq!(queue.empty(), true);
}


#[test]
#[should_panic]
fn empty_top() {
    let mut queue = lib::Queue::<i32>::new();

    // Testing for an empty queue
    
    assert_eq!(queue.size(), 0);
    assert_eq!(queue.empty(), true);
    queue.top();
}


#[test]
#[should_panic]
fn empty_pop() {
    let mut queue = lib::Queue::<i32>::new();

    // Testing for an empty queue
    
    assert_eq!(queue.size(), 0);
    assert_eq!(queue.empty(), true);
    queue.pop();
}


#[test]
fn push() {
    let mut queue = lib::Queue::<i32>::new();

    // Testing for the append function
    // by adding 3 values to the queue

    queue.push(23);
    queue.push(42);
    queue.push(78);

    assert_eq!(queue.size(), 3);
    assert_eq!(queue.empty(), false);

    assert_eq!(queue.top(), 23);
}


#[test]
fn pop() {
    let mut queue = lib::Queue::<i32>::new();

    // Testing for the append function
    // by adding 3 values to the queue

    queue.push(23);
    queue.push(42);
    queue.push(78);

    assert_eq!(queue.size(), 3);
    assert_eq!(queue.empty(), false);

    assert_eq!(queue.top(), 23);
    queue.pop();
    assert_eq!(queue.top(), 42);
    queue.pop();
    assert_eq!(queue.top(), 78);
    queue.pop();
}