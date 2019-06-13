// Copyright 2019 Mihir Wadekar and Chaitenya Gupta

mod lib;

#[test]
fn empty() {
    let list = lib::List::<i32>::new();

    // Testing for an empty list
    
    assert_eq!(list.size(), 0);
    assert_eq!(list.empty(), true);
}


#[test]
fn append() {
    let mut list = lib::List::<i32>::new();

    // Testing for the append function
    // by adding 3 values to the list

    list.append(23);
    list.append(42);
    list.append(78);

    assert_eq!(list.size(), 3);
    assert_eq!(list.empty(), false);

    assert_eq!(list[0], 23);
    assert_eq!(list[1], 42);
    assert_eq!(list[2], 78);
}


#[test]
fn get() {
    let mut list = lib::List::<i32>::new();

    // Testing for the append function
    // by adding 3 values to the list

    list.append(23);
    list.append(42);
    list.append(78);

    assert_eq!(list.size(), 3);
    assert_eq!(list.empty(), false);

    assert_eq!(list.get(0), 23);
    assert_eq!(list.get(1), 42);
    assert_eq!(list.get(2), 78);
}


#[test]
fn find() {
    let mut list = lib::List::<i32>::new();

    // Testing for the append function
    // by adding 3 values to the list

    list.append(23);
    list.append(42);
    list.append(78);

    assert_eq!(list.size(), 3);
    assert_eq!(list.empty(), false);

    assert_eq!(list.find(23), 0);
    assert_eq!(list.find(42), 1);
    assert_eq!(list.find(78), 2);
    assert_eq!(list.find(109), -1);
}


#[test]
fn insertion() {
    let mut list = lib::List::<i32>::new();

    // Testing for the append function by
    // inserting 3 values in the list

    list.insert(23, 0);
    list.insert(42, 1);
    list.insert(61, 2);
    list.insert(78, 1);
    list.insert(97, 2);
    list.insert(109, 3);


    assert_eq!(list.size(), 6);
    assert_eq!(list.empty(), false);

    assert_eq!(list[0], 23);
    assert_eq!(list[1], 78);
    assert_eq!(list[2], 97);
    assert_eq!(list[3], 109);
    assert_eq!(list[4], 42);
    assert_eq!(list[5], 61);
}


#[test]
fn removal() {
    let mut list = lib::List::<i32>::new();

    // Testing for the remove function by inserting 
    // 6 values in the list and removing 3

    list.insert(23, 0);
    list.insert(42, 1);
    list.insert(61, 2);
    list.insert(78, 1);
    list.insert(97, 2);
    list.insert(109, 3);

    list.remove(4);
    list.remove(3);
    list.remove(2);


    assert_eq!(list.size(), 3);
    assert_eq!(list.empty(), false);

    assert_eq!(list[0], 23);
    assert_eq!(list[1], 78);
    assert_eq!(list[2], 61);
}