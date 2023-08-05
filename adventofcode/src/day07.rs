//! https://adventofcode.com/2022/day/7

mod file_tree;
mod parser;

use crate::common::config::FileContentConfig;

use self::file_tree::FileTree;

pub fn run(config: FileContentConfig) -> Result<(), String> {
    println!("Day 7!");
    let tree = parser::parse_file_tree(&config.content)?;
    let (res1, root_size) = part1_solve(&tree)?;
    println!("  Part 1: sum = {}", res1);

    let res2 = part2_solve(&tree, root_size)?;
    println!("  Part 2: size = {}", res2);

    return Ok(());
}

fn part1_solve(tree: &FileTree) -> Result<(u32, u32), String> {
    println!("file tree is parsed:\n{}", tree);

    let max_size = 100_000;
    let mut small_dirs: Vec<(&str, u32)> = Vec::new();
    let root_size = tree
        .root
        .find_small_child_directories(max_size, &mut small_dirs);
    if root_size <= max_size {
        small_dirs.push(("/", root_size));
    }

    let total = small_dirs.iter().map(|(_, size)| size).sum::<u32>();

    return Ok((total, root_size));
}

fn part2_solve(tree: &FileTree, root_size: u32) -> Result<u32, String> {
    let space_for_update = 30_000_000;
    let space_available = 70_000_000 - root_size; // root_size must be <= 70_000_000

    if space_available < space_for_update {
        let need_to_free = space_for_update - space_available;
        let mut big_dirs: Vec<(&str, u32)> = Vec::new();
        tree.root
            .find_large_child_directories(need_to_free, &mut big_dirs);
        let dir_size_to_remove = big_dirs
            .into_iter()
            .map(|(_, size)| size)
            .min()
            .unwrap_or(0);
        return Ok(dir_size_to_remove);
        // println!("  Part 2: need to remove dir of size {:?}", dir_size_to_remove);
    } else {
        return Ok(0);
        // println!("  Part 2: anough space available !");
    }
}

#[cfg(test)]
mod test {
    use crate::day07::parser;

    const INPUT: &str = "$ cd /
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
    fn part1_example() {
        let tree = parser::parse_file_tree(INPUT).unwrap();
        let (res, ..) = super::part1_solve(&tree).unwrap();

        assert_eq!(res, 95437);
    }

    #[test]
    fn part2_example() {
        let tree = parser::parse_file_tree(INPUT).unwrap();
        let res = super::part2_solve(&tree, tree.root.size_recursive()).unwrap();

        assert_eq!(res, 24933642);
    }
}
