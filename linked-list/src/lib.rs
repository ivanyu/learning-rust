//#![feature(nll)]

use std::fmt::Debug;

#[derive(Debug)]
struct LinkedList<T>
where
    T: PartialEq + Debug,
{
    head: Option<Box<Node<T>>>,
    length: usize,
}

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq + Debug,
{
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: PartialEq + Debug,
{
    fn new_box(value: T) -> Box<Node<T>> {
        Box::new(Node::<T> { value, next: None })
    }
}

#[allow(dead_code)]
impl<T> LinkedList<T>
where
    T: PartialEq + Debug,
{
    fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            length: 0,
        }
    }

    fn iter(&self) -> LinkedListIterator<T> {
        self.into_iter()
    }

    fn append_before_nll(&mut self, value: T) {
        let mut current_link: &mut Option<Box<Node<T>>> = &mut self.head;

        loop {
            let tmp_link = current_link;
            match tmp_link {
                Some(ref mut current_box) => {
                    current_link = &mut current_box.next;
                }
                None => {
                    current_link = tmp_link;
                    break;
                }
            }
        }
        *current_link = Some(Node::new_box(value));
        self.length += 1;
    }

    fn append(&mut self, value: T) {
        let mut current_link: &mut Option<Box<Node<T>>> = &mut self.head;

        while let Some(ref mut current_box) = current_link {
            current_link = &mut current_box.next;
        }

        *current_link = Some(Node::new_box(value));
        self.length += 1;
    }

    fn remove_using_ref(&mut self, value: T) -> bool {
        let mut curr_link: &mut Option<Box<Node<T>>> = &mut self.head;
        while let Some(ref mut next_node_ref) = curr_link {
            if next_node_ref.value == value {
                let next_link: Option<Box<Node<T>>> = next_node_ref.next.take();
                *curr_link = next_link;
                self.length -= 1;
                return true;
            } else {
                curr_link = &mut curr_link.as_mut().unwrap().next;
            }
        }
        false
    }

    fn remove_using_move(&mut self, value: T) -> bool {
        let mut curr_link: &mut Option<Box<Node<T>>> = &mut self.head;
        while let Some(next_node) = curr_link.take() {
            if next_node.value == value {
                *curr_link = next_node.next;
                self.length -= 1;
                return true;
            } else {
                curr_link.replace(next_node);
                curr_link = &mut curr_link.as_mut().unwrap().next;
            }
        }
        false
    }
}

impl<'a, T: 'a> IntoIterator for &'a LinkedList<T>
where
    T: PartialEq + Debug,
{
    type Item = &'a T;
    type IntoIter = LinkedListIterator<'a, T>;

    fn into_iter(self: &'a LinkedList<T>) -> <Self as IntoIterator>::IntoIter {
        LinkedListIterator {
            curr_link: &self.head,
        }
    }
}

struct LinkedListIterator<'a, T>
where
    T: PartialEq + Debug,
{
    curr_link: &'a Option<Box<Node<T>>>,
}

impl<'a, T> Iterator for LinkedListIterator<'a, T>
where
    T: PartialEq + Debug,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.curr_link {
            Some(ref node) => {
                self.curr_link = &node.next;
                Some(&node.value)
            }

            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list_length() {
        assert_eq!(LinkedList::<i32>::new().length, 0);
    }

    #[test]
    fn test_empty_list_iterator() {
        let list = LinkedList::<i32>::new();
        assert_eq!((&list).iter().count(), 0);
    }

    #[test]
    fn test_append_before_nll() {
        let mut list = LinkedList::<i32>::new();

        list.append_before_nll(1);
        assert_eq!(list.length, 1);
        let v = to_vec(&list);
        assert_eq!(v, vec![1]);

        list.append_before_nll(2);
        assert_eq!(list.length, 2);
        let v = to_vec(&list);
        assert_eq!(v, vec![1, 2]);

        list.append_before_nll(3);
        assert_eq!(list.length, 3);
        let v = to_vec(&list);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_append() {
        let mut list = LinkedList::<i32>::new();

        list.append(1);
        assert_eq!(list.length, 1);
        let v = to_vec(&list);
        assert_eq!(v, vec![1]);

        list.append(2);
        assert_eq!(list.length, 2);
        let v = to_vec(&list);
        assert_eq!(v, vec![1, 2]);

        list.append(3);
        assert_eq!(list.length, 3);
        let v = to_vec(&list);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_remove_using_ref() {
        test_remove(&LinkedList::<i32>::remove_using_ref);
    }

    #[test]
    fn test_remove_using_move() {
        test_remove(&LinkedList::<i32>::remove_using_move);
    }

    fn test_remove(remove_fn: &Fn(&mut LinkedList<i32>, i32) -> bool) {
        let mut list = LinkedList::<i32>::new();
        list.append(1);
        list.append(2);
        list.append(3);
        list.append(4);
        list.append(5);

        assert_eq!(remove_fn(&mut list, 5), true);
        assert_eq!(list.length, 4);
        let v = to_vec(&list);
        assert_eq!(v, vec![1, 2, 3, 4]);

        assert_eq!(remove_fn(&mut list, 1), true);
        assert_eq!(list.length, 3);
        let v = to_vec(&list);
        assert_eq!(v, vec![2, 3, 4]);

        assert_eq!(remove_fn(&mut list, 3), true);
        assert_eq!(list.length, 2);
        let v = to_vec(&list);
        assert_eq!(v, vec![2, 4]);

        assert_eq!(remove_fn(&mut list, 2), true);
        assert_eq!(list.length, 1);
        let v = to_vec(&list);
        assert_eq!(v, vec![4]);

        assert_eq!(remove_fn(&mut list, 4), true);
        assert_eq!(list.length, 0);
        let v = to_vec(&list);
        assert_eq!(v, vec![]);
    }

    fn to_vec<T>(list: &LinkedList<T>) -> Vec<T>
    where
        T: Debug + PartialEq + Clone,
    {
        list.iter().map(|x| x.clone()).collect::<Vec<_>>()
    }
}
