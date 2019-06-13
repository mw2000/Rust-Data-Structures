// Copyright 2019 Mihir Wadekar and Chaitenya Gupta

mod lib;

#[test]
fn empty() {
    let stack = lib::Stack::<i32>::new();

    // Testing for an empty stack
    
    assert_eq!(stack.size(), 0);
    assert_eq!(stack.empty(), true);
}


#[test]
#[should_panic]
fn empty_top() {
    let mut stack = lib::Stack::<i32>::new();

    // Testing for an empty stack
    
    assert_eq!(stack.size(), 0);
    assert_eq!(stack.empty(), true);
    stack.top();
}


#[test]
#[should_panic]
fn empty_pop() {
    let mut stack = lib::Stack::<i32>::new();

    // Testing for an empty stack
    
    assert_eq!(stack.size(), 0);
    assert_eq!(stack.empty(), true);
    stack.pop();
}


#[test]
fn push() {
    let mut stack = lib::Stack::<i32>::new();

    // Testing for the append function
    // by adding 3 values to the stack

    stack.push(23);
    stack.push(42);
    stack.push(78);

    assert_eq!(stack.size(), 3);
    assert_eq!(stack.empty(), false);

    assert_eq!(stack.top(), 78);
}


#[test]
fn pop() {
    let mut stack = lib::Stack::<i32>::new();

    // Testing for the append function
    // by adding 3 values to the stack

    stack.push(23);
    stack.push(42);
    stack.push(78);

    assert_eq!(stack.size(), 3);
    assert_eq!(stack.empty(), false);

    assert_eq!(stack.top(), 78);
    stack.pop();
    assert_eq!(stack.top(), 42);
    stack.pop();
    assert_eq!(stack.top(), 23);
    stack.pop();
}