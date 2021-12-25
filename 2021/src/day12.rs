use std::cell::{RefCell, RefMut, UnsafeCell};
use std::collections::{HashMap, HashSet};
//With your submarine's subterranean subsystems subsisting suboptimally, the only way you're getting out of this cave anytime soon is by finding a path yourself. Not just a path - the only way to know if you've found the best path is to find all of them.

//Fortunately, the sensors are still mostly working, and so you build a rough map of the remaining caves (your puzzle input). For example:

//start-A
//start-b
//A-c
//A-b
//b-d
//A-end
//b-end

//This is a list of how all of the caves are connected. You start in the cave named start, and your destination is the cave named end. An entry like b-d means that cave b is connected to cave d - that is, you can move between them.

//So, the above cave system looks roughly like this:

//start
///   \
//c--A-----b--d
//\   /
//end

//Your goal is to find the number of distinct paths that start at start, end at end, and don't visit small caves more than once. There are two types of caves: big caves (written in uppercase, like A) and small caves (written in lowercase, like b). It would be a waste of time to visit any small cave more than once, but big caves are large enough that it might be worth visiting them multiple times. So, all paths you find should visit small caves at most once, and can visit big caves any number of times.

//Given these rules, there are 10 paths through this example cave system:

//start,A,b,A,c,A,end
//start,A,b,A,end
//start,A,b,end
//start,A,c,A,b,A,end
//start,A,c,A,b,end
//start,A,c,A,end
//start,A,end
//start,b,A,c,A,end
//start,b,A,end
//start,b,end

//(Each line in the above list corresponds to a single path; the caves visited by that path are listed in the order they are visited and separated by commas.)

//Note that in this cave system, cave d is never visited by any path: to do so, cave b would need to be visited twice (once on the way to cave d and a second time when returning from cave d), and since cave b is small, this is not allowed.

//Here is a slightly larger example:

//dc-end
//HN-start
//start-kj
//dc-start
//dc-HN
//LN-dc
//HN-end
//kj-sa
//kj-HN
//kj-dc

//The 19 paths through it are as follows:

//start,HN,dc,HN,end
//start,HN,dc,HN,kj,HN,end
//start,HN,dc,end
//start,HN,dc,kj,HN,end
//start,HN,end
//start,HN,kj,HN,dc,HN,end
//start,HN,kj,HN,dc,end
//start,HN,kj,HN,end
//start,HN,kj,dc,HN,end
//start,HN,kj,dc,end
//start,dc,HN,end
//start,dc,HN,kj,HN,end
//start,dc,end
//start,dc,kj,HN,end
//start,kj,HN,dc,HN,end
//start,kj,HN,dc,end
//start,kj,HN,end
//start,kj,dc,HN,end
//start,kj,dc,end

//Finally, this even larger example has 226 paths through it:

//fs-end
//he-DX
//fs-he
//start-DX
//pj-DX
//end-zg
//zg-sl
//zg-pj
//pj-he
//RW-he
//fs-DX
//pj-RW
//zg-RW
//start-pj
//he-WI
//zg-he
//pj-fs
//start-RW

//How many paths through this cave system are there that visit small caves at most once?
#[derive(Debug)]
struct Node<'a> {
    small: bool,
    name: &'static str,
    edges: UnsafeCell<Vec<&'a Node<'a>>>,
}

impl<'a> Node<'a> {
    fn new(name: &'static str, small: bool, mut arena: RefMut<'a, Vec<Node<'a>>>) -> &'a Node<'a> {
        let len = arena.len();
        arena.push(Node {
            name,
            small,
            edges: UnsafeCell::new(Vec::new()),
        });
        unsafe { &mut *arena.as_mut_ptr().add(len) }
    }

    fn explore(
        &'a self,
        path: String,
        name: &'static str,
        seen: &mut HashSet<&'static str>,
    ) -> Vec<String> {
        if seen.contains(&self.name) || seen.contains(name) {
            return vec![];
        }
        if self.small {
            seen.insert(self.name);
        }
        if self.name == name {
            return vec![format!("{},{}", path, name)];
        }
        let paths: Vec<String> = unsafe {
            (*self.edges.get())
                .iter()
                .cloned()
                .flat_map(|n| n.explore(format!("{},{}", path, self.name), name, &mut seen.clone()))
                .collect()
        };

        paths
    }

    fn explore2(
        &'a self,
        path: String,
        start: &'static str,
        end: &'static str,
        twice: bool,
        seen: &mut HashSet<&'static str>,
    ) -> Vec<String> {
        let already_visited = seen.contains(&self.name);
        if (already_visited && twice) || seen.contains(end) {
            return vec![];
        }
        if self.small {
            seen.insert(self.name);
        }
        if self.name == end {
            return vec![format!("{},{}", path, end)];
        }
        let paths: Vec<String> = unsafe {
            (*self.edges.get())
                .iter()
                .cloned()
                .flat_map(|n| {
                    if n.name == start {
                        return vec![];
                    }
                    n.explore2(
                        format!("{},{}", path, self.name),
                        start,
                        end,
                        already_visited || twice,
                        &mut seen.clone(),
                    )
                })
                .collect()
        };

        paths
    }
}

pub fn find_cave_exits() {
    let arena: RefCell<Vec<Node<'_>>> = RefCell::new(Vec::with_capacity(CAVE_CONNECTIONS.len()));
    for connection in CAVE_CONNECTIONS.iter() {
        let (opening, exit) = match connection.split("-").collect::<Vec<&str>>()[..] {
            [first, second, ..] => (first, second),
            _ => unreachable!(),
        };
        unsafe {
            let opening = if let Some(n) = arena
                .as_ptr()
                .as_ref()
                .unwrap()
                .iter()
                .find(|n| n.name == opening)
            {
                n
            } else {
                Node::new(
                    opening,
                    opening.as_bytes()[0].is_ascii_lowercase(),
                    arena.borrow_mut(),
                )
            };
            let exit = if let Some(n) = arena
                .as_ptr()
                .as_ref()
                .unwrap()
                .iter()
                .find(|n| n.name == exit)
            {
                n
            } else {
                Node::new(
                    exit,
                    exit.as_bytes()[0].is_ascii_lowercase(),
                    arena.borrow_mut(),
                )
            };
            (*opening.edges.get()).push(exit);
            (*exit.edges.get()).push(opening);
        }
    }
    unsafe {
        if let Some(n) = arena
            .as_ptr()
            .as_ref()
            .unwrap()
            .iter()
            .find(|n| n.name == "start")
        {
            let end = n.explore(String::new(), "end", &mut HashSet::new());
            println!("{:?}", end);
            println!("{}", end.len());
        }
    }
}
//After reviewing the available paths, you realize you might have time to visit a single small cave twice. Specifically, big caves can be visited any number of times, a single small cave can be visited at most twice, and the remaining small caves can be visited at most once. However, the caves named start and end can only be visited exactly once each: once you leave the start cave, you may not return to it, and once you reach the end cave, the path must end immediately.

//Now, the 36 possible paths through the first example above are:

//start,A,b,A,b,A,c,A,end
//start,A,b,A,b,A,end
//start,A,b,A,b,end
//start,A,b,A,c,A,b,A,end
//start,A,b,A,c,A,b,end
//start,A,b,A,c,A,c,A,end
//start,A,b,A,c,A,end
//start,A,b,A,end
//start,A,b,d,b,A,c,A,end
//start,A,b,d,b,A,end
//start,A,b,d,b,end
//start,A,b,end
//start,A,c,A,b,A,b,A,end
//start,A,c,A,b,A,b,end
//start,A,c,A,b,A,c,A,end
//start,A,c,A,b,A,end
//start,A,c,A,b,d,b,A,end
//start,A,c,A,b,d,b,end
//start,A,c,A,b,end
//start,A,c,A,c,A,b,A,end
//start,A,c,A,c,A,b,end
//start,A,c,A,c,A,end
//start,A,c,A,end
//start,A,end
//start,b,A,b,A,c,A,end
//start,b,A,b,A,end
//start,b,A,b,end
//start,b,A,c,A,b,A,end
//start,b,A,c,A,b,end
//start,b,A,c,A,c,A,end
//start,b,A,c,A,end
//start,b,A,end
//start,b,d,b,A,c,A,end
//start,b,d,b,A,end
//start,b,d,b,end
//start,b,end

//The slightly larger example above now has 103 paths through it, and the even larger example now has 3509 paths through it.

//Given these new rules, how many paths through this cave system are there?

pub fn find_cave_exits2() {
    let arena: RefCell<Vec<Node<'_>>> = RefCell::new(Vec::with_capacity(CAVE_CONNECTIONS.len()));
    for connection in CAVE_CONNECTIONS.iter() {
        let (opening, exit) = match connection.split("-").collect::<Vec<&str>>()[..] {
            [first, second, ..] => (first, second),
            _ => unreachable!(),
        };
        unsafe {
            let opening = if let Some(n) = arena
                .as_ptr()
                .as_ref()
                .unwrap()
                .iter()
                .find(|n| n.name == opening)
            {
                n
            } else {
                Node::new(
                    opening,
                    opening.as_bytes()[0].is_ascii_lowercase(),
                    arena.borrow_mut(),
                )
            };
            let exit = if let Some(n) = arena
                .as_ptr()
                .as_ref()
                .unwrap()
                .iter()
                .find(|n| n.name == exit)
            {
                n
            } else {
                Node::new(
                    exit,
                    exit.as_bytes()[0].is_ascii_lowercase(),
                    arena.borrow_mut(),
                )
            };
            (*opening.edges.get()).push(exit);
            (*exit.edges.get()).push(opening);
        }
    }
    unsafe {
        if let Some(n) = arena
            .as_ptr()
            .as_ref()
            .unwrap()
            .iter()
            .find(|n| n.name == "start")
        {
            let end = n.explore2(String::new(), "start", "end", false, &mut HashSet::new());
            println!("{:?}", end);
            println!("{}", end.len());
        }
    }
}
const BCAVE_CONNECTIONS: [&str; 7] = ["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];
const ACAVE_CONNECTIONS: [&str; 10] = [
    "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa", "kj-HN",
    "kj-dc",
];
const CAVE_CONNECTIONS: [&str; 24] = [
    "by-TW", "start-TW", "fw-end", "QZ-end", "JH-by", "ka-start", "ka-by", "end-JH", "QZ-cv",
    "vg-TI", "by-fw", "QZ-by", "JH-ka", "JH-vg", "vg-fw", "TW-cv", "QZ-vg", "ka-TW", "ka-QZ",
    "JH-fw", "vg-hu", "cv-start", "by-cv", "ka-cv",
];
