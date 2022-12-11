use crate::formater::*;

#[derive(Debug, PartialEq)]
enum Item {
    File(String, Int),
    Folder(String, Vec<Item>),
}

impl PartialEq<str> for Item {
    fn eq(&self, other: &str) -> bool {
        match self {
            Self::File(l0, _) => l0 == other,
            Self::Folder(l0, _) => l0 == other,
        }
    }
}
impl Item {
    pub fn new_file(name: &str, size: Int) -> Self {
        Self::File(name.into(), size)
    }
    pub fn new_folder(name: &str) -> Self {
        Self::Folder(name.into(), Vec::new())
    }
    pub fn find_path(&mut self, path: &Vec<&str>) -> &mut Self {
        let mut cur = self;
        for i in path {
            if let Item::Folder(_, content) = cur {
                cur = content.iter_mut().filter(|x| x == i).next().unwrap()
            }
        }
        cur
    }
    pub fn add_into_folder(&mut self, other: Self) {
        if let Self::Folder(_, content) = self {
            content.push(other)
        }
    }
    pub fn get_content_iter(&self) -> impl Iterator<Item = &Item> {
        if let Item::Folder(_, content) = self {
            content.iter()
        } else {
            [].iter()
        }
    }
    pub fn run_on_all<F, T>(&self, func: &F, init: T) -> T
    where
        F: Fn(&Item, T) -> T,
    {
        let mut temp = func(self, init);
        for i in self.get_content_iter() {
            temp = i.run_on_all(func, temp);
        }
        temp
    }
    pub fn size(&self) -> usize {
        match self {
            Item::Folder(_, content) => content.iter().map(Item::size).sum(),
            Item::File(_, size) => *size as usize,
        }
    }
}

pub const FILE: usize = 7;
type Int = usize;
type Data = Item;

pub fn run(filename: Option<String>) -> Result<(), std::io::Error> {
    print_single_parse(FILE, filename, parse_input, part_1, part_2)
}

fn parse_input(input: &str) -> Data {
    let mut out = Item::Folder("/".to_string(), Vec::new());
    let mut path: Vec<&str> = Vec::new();
    let mut cursor = out.find_path(&path);
    let mut iter = input.lines();
    iter.next();
    for line in iter {
        if let Some(command) = line.strip_prefix("$ ") {
            if let Some(arg) = command.strip_prefix("cd ") {
                if arg == ".." {
                    path.pop();
                } else {
                    path.push(arg);
                }
                cursor = out.find_path(&path);
            } //ignore ls for now
        } else {
            let new = match line.split_once(' ') {
                Some(("dir", name)) => Item::new_folder(name),
                Some((size, name)) => Item::new_file(name, size.parse().unwrap()),
                None => panic!("invalid folder content"),
            };
            cursor.add_into_folder(new);
        }
    }
    out
}

fn part_1(data: &Data) -> usize {
    data.run_on_all(
        &|item, count| {
            if let Item::Folder(_, _) = item {
                let size = item.size();
                if size <= 100000 {
                    count + size
                } else {
                    count
                }
            } else {
                count
            }
        },
        0,
    )
}

fn part_2(data: &Data) -> Int {
    let total = 70000000;
    let need_free = 30000000;
    let used_space = data.size();
    let need_to_free = need_free - (total - used_space);

    data.run_on_all(
        &|item, (needed, closest)| {
            if let Item::Folder(_, _) = item {
                let size = item.size();
                if size <= closest && size >= needed {
                    (needed, size)
                } else {
                    (needed, closest)
                }
            } else {
                (needed, closest)
            }
        },
        (need_to_free, used_space),
    )
    .1
}

#[cfg(test)]
mod tests {
    use crate::day_07::{parse_input, part_1, part_2, Item, FILE};
    use crate::formater::read_file;

    static EXEMPLE_1: &'static str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_example() {
        let exemple = Item::Folder(
            "/".to_string(),
            vec![
                Item::Folder(
                    "a".to_string(),
                    vec![
                        Item::Folder("e".to_string(), vec![Item::File("i".to_string(), 584)]),
                        Item::File("f".to_string(), 29116),
                        Item::File("g".to_string(), 2557),
                        Item::File("h.lst".to_string(), 62596),
                    ],
                ),
                Item::File("b.txt".to_string(), 14848514),
                Item::File("c.dat".to_string(), 8504156),
                Item::Folder(
                    "d".to_string(),
                    vec![
                        Item::File("j".to_string(), 4060174),
                        Item::File("d.log".to_string(), 8033020),
                        Item::File("d.ext".to_string(), 5626152),
                        Item::File("k".to_string(), 7214296),
                    ],
                ),
            ],
        );
        assert_eq!(parse_input(EXEMPLE_1), exemple);
        assert_eq!(part_1(&exemple), 95437);
        assert_eq!(part_2(&exemple), 24933642);
    }
    #[test]
    fn test() {
        let data = parse_input(&read_file(FILE));
        assert_eq!(part_1(&data).to_string(), format!("{}", 1517599));
        assert_eq!(part_2(&data).to_string(), format!("{}", 2481982));
    }
}
