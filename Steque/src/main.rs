// Copyright 2019 Mihir Wadekar and Chaitenya Gupta

mod lib;

#[test]
fn empty() {
  let steque = lib::Steque::<i32>::new();

  // Testing for an empty steque
  
  assert_eq!(steque.size(), 0);
  assert_eq!(steque.empty(), true);
}


#[test]
#[should_panic]
fn empty_front() {
  let mut steque = lib::Steque::<i32>::new();

  // Testing for an empty steque
  
  assert_eq!(steque.size(), 0);
  assert_eq!(steque.empty(), true);
  steque.front();
}


#[test]
#[should_panic]
fn empty_back() {
  let mut steque = lib::Steque::<i32>::new();

  // Testing for an empty steque
  
  assert_eq!(steque.size(), 0);
  assert_eq!(steque.empty(), true);
  steque.back();
}


#[test]
#[should_panic]
fn empty_pop() {
    let mut steque = lib::Steque::<i32>::new();

    // Testing for an empty steque
    
    assert_eq!(steque.size(), 0);
    assert_eq!(steque.empty(), true);
    steque.pop();
}


#[test]
fn push() {
    let mut steque = lib::Steque::<i32>::new();

    // Testing for the push function
    // by adding 3 values to the steque

    steque.push(23);
    steque.enqueue(42);
    steque.push(78);

    assert_eq!(steque.size(), 3);
    assert_eq!(steque.empty(), false);

    assert_eq!(steque.front(), 78);
    assert_eq!(steque.back(), 42);
}


#[test]
fn pop() {
    let mut steque = lib::Steque::<i32>::new();

    // Testing for the remove function
    // by removing 3 values from the steque

    steque.push(23);
    steque.push(42);
    steque.push(78);

    assert_eq!(steque.size(), 3);
    assert_eq!(steque.empty(), false);

    assert_eq!(steque.front(), 78);
    steque.pop();
    assert_eq!(steque.back(), 42);
    steque.pop();
    assert_eq!(steque.front(), 23);
    steque.pop();
}