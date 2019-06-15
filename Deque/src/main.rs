// Copyright 2019 Mihir Wadekar and Chaitenya Gupta

mod lib;

#[test]
fn empty() {
  let deque = lib::Deque::<i32>::new();

  // Testing for an empty deque
  
  assert_eq!(deque.size(), 0);
  assert_eq!(deque.empty(), true);
}


#[test]
#[should_panic]
fn empty_front() {
  let mut deque = lib::Deque::<i32>::new();

  // Testing for an empty deque
  
  assert_eq!(deque.size(), 0);
  assert_eq!(deque.empty(), true);
  deque.front();
}


#[test]
#[should_panic]
fn empty_back() {
  let mut deque = lib::Deque::<i32>::new();

  // Testing for an empty deque
  
  assert_eq!(deque.size(), 0);
  assert_eq!(deque.empty(), true);
  deque.back();
}


#[test]
#[should_panic]
fn empty_pop_front() {
    let mut deque = lib::Deque::<i32>::new();

    // Testing for an empty deque
    
    assert_eq!(deque.size(), 0);
    assert_eq!(deque.empty(), true);
    deque.pop_front();
}


#[test]
#[should_panic]
fn empty_pop_back() {
    let mut deque = lib::Deque::<i32>::new();

    // Testing for an empty deque
    
    assert_eq!(deque.size(), 0);
    assert_eq!(deque.empty(), true);
    deque.pop_back();
}


#[test]
fn push() {
    let mut deque = lib::Deque::<i32>::new();

    // Testing for the push function
    // by adding 3 values to the deque

    deque.push_front(23);
    deque.push_back(42);
    deque.push_front(78);

    assert_eq!(deque.size(), 3);
    assert_eq!(deque.empty(), false);

    assert_eq!(deque.front(), 78);
    assert_eq!(deque.back(), 42);
}


#[test]
fn pop() {
    let mut deque = lib::Deque::<i32>::new();

    // Testing for the remove function
    // by removing 3 values from the deque

    deque.push_back(23);
    deque.push_back(42);
    deque.push_back(78);

    assert_eq!(deque.size(), 3);
    assert_eq!(deque.empty(), false);

    assert_eq!(deque.front(), 23);
    deque.pop_front();
    assert_eq!(deque.back(), 78);
    deque.pop_back();
    assert_eq!(deque.front(), 42);
    deque.pop_front();
}