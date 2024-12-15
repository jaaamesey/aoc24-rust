use hashbrown::HashMap;

struct Block {
    id: usize,
    is_static: bool,
}

pub fn part1() {
    let (blocks, mut block_positions, mut pos, commands) = {
        let mut blocks = Vec::<Block>::new();
        let mut block_positions = HashMap::<(i32, i32), usize>::new();
        let lines = include_str!("./real_input.txt").lines();
        let mut commands = Vec::<(i32, i32)>::new();
        let mut initial_pos: Option<(i32, i32)> = None;
        let mut parsing_commands = false;
        for (y, row) in lines.enumerate() {
            if parsing_commands {
                row.chars().for_each(|char| {
                    commands.push(match char {
                        '^' => (-1, 0),
                        '>' => (0, 1),
                        '<' => (0, -1),
                        'v' => (1, 0),
                        _ => panic!(),
                    })
                });
                continue;
            }
            if row.len() == 0 {
                parsing_commands = true;
                continue;
            }
            for (x, char) in row.chars().enumerate() {
                if char == '.' {
                    continue;
                }
                if char == '@' {
                    initial_pos = Some((y.try_into().unwrap(), x.try_into().unwrap()));
                    continue;
                }
                let block = Block {
                    id: blocks.len(),
                    is_static: char == '#',
                };
                block_positions.insert((y.try_into().unwrap(), x.try_into().unwrap()), block.id);
                blocks.push(block);
            }
        }
        (blocks, block_positions, initial_pos.unwrap(), commands)
    };

    commands.iter().for_each(|dir| {
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        let mut blocks_to_move = Vec::<(i32, i32)>::new();
        let can_move = try_move_block(
            &blocks,
            &block_positions,
            *dir,
            new_pos,
            &mut blocks_to_move,
        );
        if !can_move {
            return;
        }
        let updates = blocks_to_move
            .iter()
            .map(|old_pos| {
                let id = block_positions.remove(old_pos).unwrap();
                ((old_pos.0 + dir.0, old_pos.1 + dir.1), id)
            })
            .collect::<Vec<_>>();
        updates.iter().for_each(|(k, v)| {
            block_positions.insert(*k, *v);
        });
        pos = new_pos;
    });

    let sum = block_positions
        .iter()
        .map(|(pos, id)| {
            let block = &blocks[*id];
            if block.is_static {
                return 0;
            }
            return 100 * pos.0 + pos.1;
        })
        .sum::<i32>();

    dbg!(sum);

    fn try_move_block(
        blocks: &Vec<Block>,
        block_positions: &HashMap<(i32, i32), usize>,
        dir: (i32, i32),
        block_position: (i32, i32),
        blocks_to_move: &mut Vec<(i32, i32)>,
    ) -> bool {
        if let Some(block_id) = block_positions.get(&block_position) {
            let block = blocks.get(*block_id).unwrap();
            blocks_to_move.push(block_position);
            if block.is_static {
                return false;
            } else {
                return try_move_block(
                    blocks,
                    block_positions,
                    dir,
                    (block_position.0 + dir.0, block_position.1 + dir.1),
                    blocks_to_move,
                );
            }
        } else {
            return true;
        }
    }
}

pub fn part2() {}
