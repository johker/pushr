extern crate names;

use crate::prush::instructions::InstructionCache;
use crate::prush::item::Item;
use crate::prush::state::PushState;
use names::Generator;
use rand::distributions::{Distribution, Standard, Uniform};
use rand::Rng;

/// Item types without list
pub enum ItemType {
    Boolean,
    Float,
    Instruction,
    Integer,
    Name,
}

impl Distribution<ItemType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ItemType {
        match rng.gen_range(0..=5) {
            0 => ItemType::Boolean,
            1 => ItemType::Float,
            2 => ItemType::Instruction,
            3 => ItemType::Integer,
            _ => ItemType::Name,
        }
    }
}

pub struct CodeGenerator {}

impl CodeGenerator {
    /// Returns random code of random size but smaller than max_points
    pub fn random_code(
        push_state: &PushState,
        instructions: &InstructionCache,
        max_points: usize,
    ) -> Option<Item> {
        if max_points > 0 {
            let mut rng = rand::thread_rng();
            let actual_points = Uniform::from(1..max_points).sample(&mut rng);
            Some(CodeGenerator::random_code_with_size(
                push_state,
                instructions,
                actual_points,
            ))
        } else {
            None
        }
    }

    /// Returns random float value within the bounds given by configuration
    pub fn random_float(push_state: &PushState) -> Option<f32> {
        let mut rng = rand::thread_rng();
        if push_state.configuration.min_random_float < push_state.configuration.max_random_float {
            Some(rng.gen_range(
                push_state.configuration.min_random_float
                    ..push_state.configuration.max_random_float,
            ))
        } else {
            None
        }
    }

    /// Returns random integer value within the bounds given by configuration
    pub fn random_integer(push_state: &PushState) -> Option<i32> {
        let mut rng = rand::thread_rng();
        if push_state.configuration.min_random_integer < push_state.configuration.max_random_integer
        {
            Some(rng.gen_range(
                push_state.configuration.min_random_integer
                    ..push_state.configuration.max_random_integer,
            ))
        } else {
            None
        }
    }

    /// Returns a random name that is not being used yet
    pub fn new_random_name() -> String {
        let mut generator = Generator::default();
        let rand_name = generator.next().unwrap();
        // TODO: Check for collisions?
        return rand_name;
    }

    /// Selects a random item from the name bindings or a new
    /// name if there is not name binding yet.
    pub fn existing_random_name(push_state: &PushState) -> String {
        let name_size = push_state.name_bindings.len();
        if name_size == 0 {
            CodeGenerator::new_random_name()
        } else {
            let mut rng = rand::thread_rng();
            let name_idx = rng.gen_range(0..name_size);
            let names: Vec<String> = push_state.name_bindings.keys().cloned().collect();
            names[name_idx].to_string()
        }
    }

    /// Return random code of size points
    pub fn random_code_with_size(
        push_state: &PushState,
        instructions: &InstructionCache,
        points: usize,
    ) -> Item {
        let number_instructions = instructions.list.len();
        if points == 1 {
            let mut rng = rand::thread_rng();
            let item_type: ItemType = rand::random();
            match item_type {
                ItemType::Boolean => Item::bool(rng.gen::<bool>()),
                ItemType::Float => Item::float(rng.gen::<f32>()),
                ItemType::Instruction => {
                    if number_instructions > 0 {
                        let instruction_idx = rng.gen_range(0..number_instructions);
                        let selected_instruction =
                            instructions.list.get(instruction_idx).unwrap().clone();
                        Item::instruction(selected_instruction)
                    } else {
                        Item::noop()
                    }
                }
                ItemType::Integer => Item::int(rng.gen::<i32>()),
                ItemType::Name => {
                    let rand_name;
                    let pnew_name = push_state.configuration.new_erc_name_probability;
                    let n_total = 10000;
                    let n_event_new_name = (pnew_name * n_total as f32) as u32;
                    if rng.gen_range(0..n_total) < n_event_new_name {
                        rand_name = CodeGenerator::new_random_name();
                    } else {
                        rand_name = CodeGenerator::existing_random_name(push_state);
                    }
                    Item::name(rand_name)
                }
            }
        } else {
            let mut item_distribution: Vec<usize> = vec![];
            CodeGenerator::decompose(&mut item_distribution, points - 1);
            let mut items_this_level: Vec<Item> = Vec::with_capacity(item_distribution.len());
            for i in 0..item_distribution.len() {
                items_this_level.push(CodeGenerator::random_code_with_size(
                    push_state,
                    instructions,
                    item_distribution[i],
                ));
            }
            Item::list(items_this_level)
        }
    }

    /// Returns a vector of random size whose elements sum up to
    /// remaining_item
    pub fn decompose(elements: &mut Vec<usize>, remaining_items: usize) {
        if remaining_items == 1 {
            elements.push(1);
            return;
        }
        let mut rng = rand::thread_rng();
        let items_this_level = rng.gen_range(1..remaining_items) as usize;
        elements.push(items_this_level);
        CodeGenerator::decompose(elements, remaining_items - items_this_level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prush::instructions::InstructionSet;

    #[test]
    fn random_code_is_generated() {
        let push_state = PushState::new();
        let test_size = 1034;
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();
        let instructions = instruction_set.cache();
        let random_item = CodeGenerator::random_code(&push_state, &instructions, test_size);
        assert!(Item::size(&random_item.unwrap()) <= test_size);
    }

    #[test]
    fn random_code_with_size_is_generated() {
        let push_state = PushState::new();
        let test_size = 235;
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();
        let instructions = instruction_set.cache();
        let random_item =
            CodeGenerator::random_code_with_size(&push_state, &instructions, test_size);
        assert_eq!(Item::size(&random_item), test_size);
    }

    #[test]
    fn decompose_generates_valid_distribution() {
        let test_size = 11;
        let mut test_distribution: Vec<usize> = vec![];
        CodeGenerator::decompose(&mut test_distribution, test_size);
        assert_eq!(test_distribution.iter().sum::<usize>(), test_size);
    }
}
