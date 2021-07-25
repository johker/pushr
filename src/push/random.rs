extern crate names;

use crate::push::instructions::InstructionCache;
use crate::push::item::Item;
use crate::push::state::PushState;
use names::Generator;
use rand::distributions::{Distribution, Standard, Uniform};
use rand::Rng;

pub enum ItemType {
    Boolean,
    Float,
    Instruction,
    Integer,
    List,
    Name,
}

impl Distribution<ItemType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ItemType {
        match rng.gen_range(0..=5) {
            0 => ItemType::Boolean,
            1 => ItemType::Float,
            2 => ItemType::Instruction,
            3 => ItemType::Integer,
            4 => ItemType::List,
            _ => ItemType::Name,
        }
    }
}

pub struct CodeGenerator {}

impl CodeGenerator {
    pub fn random_code(
        push_state: &PushState,
        instructions: &InstructionCache,
        max_points: usize,
    ) -> Item {
        let mut rng = rand::thread_rng();
        let actual_points = Uniform::from(1..max_points).sample(&mut rng);
        CodeGenerator::random_code_with_size(push_state, instructions, actual_points)
    }

    pub fn random_code_with_size(
        push_state: &PushState,
        instructions: &InstructionCache,
        points: usize,
    ) -> Item {
        let number_instructions = instructions.list.len();
        let mut generator = Generator::default();
        if points == 1 {
            let mut rng = rand::thread_rng();
            let item_type: ItemType = rand::random();
            match item_type {
                ItemType::Boolean => Item::bool(rng.gen::<bool>()),
                ItemType::Float => Item::float(rng.gen::<f32>()),
                ItemType::Instruction => {
                    let instruction_idx = rng.gen_range(0..number_instructions);
                    let selected_instruction =
                        instructions.list.get(instruction_idx).unwrap().clone();
                    Item::instruction(selected_instruction)
                }
                ItemType::Integer => Item::int(rng.gen::<i32>()),
                ItemType::List => Item::empty_list(),
                ItemType::Name => {
                    let rand_name;
                    let name_size = push_state.name_bindings.len();
                    let pnew_name = push_state.configuration.new_erc_name_probability;
                    let n_total = 10000;
                    let n_event_new_name = (pnew_name * n_total as f32) as u32;
                    if name_size == 0 || rng.gen_range(0..n_total) < n_event_new_name {
                        rand_name = generator.next().unwrap();
                    } else {
                        let name_idx = rng.gen_range(0..name_size);
                        let names: Vec<&str> = push_state.name_bindings.keys().cloned().collect();
                        rand_name = names[name_idx].to_string();
                    }
                    Item::name(rand_name)
                }
            }
        } else {
            Item::noop()
        }
    }
}
