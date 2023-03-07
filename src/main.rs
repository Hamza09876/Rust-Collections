use std::collections::{VecDeque, LinkedList, HashMap, BTreeMap, HashSet, BTreeSet, BinaryHeap};

fn main() {
    // Create a vector
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    println!("Vector: {:?}", vec);

    // Create a deque
    let mut deque = VecDeque::new();
    deque.push_front(1);
    deque.push_back(2);
    println!("Deque: {:?}", deque);

    // Create a linked list
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    println!("Linked List: {:?}", list);

    // Create a HashMap
    let mut map = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 2);
    map.insert("orange", 5);
    println!("HashMap: {:?}", map);

    // Create a BTreeMap
    let mut btree_map = BTreeMap::new();
    btree_map.insert("apple", 3);
    btree_map.insert("banana", 2);
    btree_map.insert("orange", 5);
    println!("BTreeMap: {:?}", btree_map);
     // Create a HashSet
     let mut set = HashSet::new();
     set.insert("apple");
     set.insert("banana");
     set.insert("orange");
     println!("HashSet: {:?}", set);
 
     // Create a BTreeSet
     let mut btree_set = BTreeSet::new();
     btree_set.insert("apple");
     btree_set.insert("banana");
     btree_set.insert("orange");
     println!("BTreeSet: {:?}", btree_set);
     // Create a BinaryHeap
    let mut heap = BinaryHeap::new();
    heap.push(3);
    heap.push(1);
    heap.push(4);
    heap.push(1);
    heap.push(5);

    // Print the heap
    while let Some(elem) = heap.pop() {
        println!("{}", elem);
    }
}