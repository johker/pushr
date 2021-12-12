extern crate names;

use crate::push::instructions::InstructionCache;
use crate::push::item::Item;
use crate::push::state::PushState;
use crate::push::vector::{BoolVector, FloatVector, IntVector};
use names::Generator;
use rand::distributions::{Distribution, Standard, Uniform};
use rand::Rng;
use rand_distr::Normal;

/// Item types without list
pub enum ItemType {
    Boolean,
    Float,
    Instruction,
    Integer,
    Name,
    BoolVector,
    FloatVector,
    IntVector,
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
    pub fn random_code<'a>(
        push_state: &PushState<'a>,
        instructions: &InstructionCache,
        max_points: usize,
    ) -> Option<Item<'a>> {
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

    /// Returns a random boolean vector of given size and sparcity
    pub fn random_bool_vector(size: i32, sparsity: f32) -> Option<BoolVector> {
        if size < 0 || sparsity < 0.0 || sparsity > 1.0 {
            None
        } else {
            let mut rng = rand::thread_rng();
            // default = false when less than half of the bits should be active
            // sparcity = portion of non-default values
            let default = sparsity > 0.5;
            let sparsity = (100.0 * f32::min(sparsity, 1.0 - sparsity)).round() / 100.0;
            let mut bool_vector = vec![default; size as usize];
            let num_active_bits = (sparsity * size as f32) as i32;
            for _i in 1..num_active_bits + 1 {
                loop {
                    let rand_idx = rng.gen_range(0..size - 1) as usize;
                    // Flip bit if it is still default, select other index otherwise
                    if bool_vector[rand_idx] == default {
                        bool_vector[rand_idx] = !default;
                        break;
                    }
                }
            }
            Some(BoolVector::new(bool_vector))
        }
    }

    /// Returns a random float vector. Its elements are independent and identically distributed
    /// random variables drawn from the normal distribution with given mean and standard
    /// deviation.
    pub fn random_float_vector(size: i32, mean: f32, stddev: f32) -> Option<FloatVector> {
        if size < 0 || stddev < 0.0 {
            None
        } else {
            let mut float_vector = Vec::with_capacity(size as usize);
            let mut r = rand::thread_rng();
            let n = Normal::new(mean, stddev).unwrap();
            for _i in 0..size {
                float_vector.push(n.sample(&mut r));
            }
            Some(FloatVector::new(float_vector))
        }
    }

    /// Returns a random integer vector. Its elements are independent and identically distributed
    /// random variables drawn from the uniform distribution with given min and max values.
    pub fn random_int_vector(size: i32, min: i32, max: i32) -> Option<IntVector> {
        if size < 0 || max <= min {
            None
        } else {
            let mut int_vector = Vec::with_capacity(size as usize);
            let mut r = rand::thread_rng();
            for _i in 0..size {
                int_vector.push(r.gen_range(min..max));
            }
            Some(IntVector::new(int_vector))
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
    pub fn random_code_with_size<'a>(
        push_state: &PushState<'a>,
        instructions: &InstructionCache,
        points: usize,
    ) -> Item<'a> {
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
                ItemType::BoolVector => {
                    let sparsity = rng.gen_range(0.0..1.0);
                    let size = rng.gen_range(0..push_state.configuration.max_random_integer);
                    Item::boolvec(CodeGenerator::random_bool_vector(size, sparsity).unwrap())
                }
                ItemType::FloatVector => {
                    let size = rng.gen_range(0..push_state.configuration.max_random_integer);
                    let mean = rng.gen_range(
                        push_state.configuration.min_random_float
                            ..push_state.configuration.max_random_float,
                    );
                    let stddev = rng.gen_range(0.0..push_state.configuration.max_random_float);
                    Item::floatvec(CodeGenerator::random_float_vector(size, mean, stddev).unwrap())
                }
                ItemType::IntVector => {
                    let size = rng.gen_range(0..push_state.configuration.max_random_integer);
                    Item::intvec(
                        CodeGenerator::random_int_vector(
                            size,
                            push_state.configuration.min_random_integer,
                            push_state.configuration.max_random_integer,
                        )
                        .unwrap(),
                    )
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
    use crate::push::instructions::InstructionSet;

    #[test]
    fn random_bool_vector_is_generated() {
        let test_size = 100;
        let test_sparsity = vec![0.0, 0.12, 0.5, 0.85, 1.0];
        for test_sp in test_sparsity {
            if let Some(rand_bool_vector) = CodeGenerator::random_bool_vector(test_size, test_sp) {
                assert_eq!(rand_bool_vector.values.len(), test_size as usize);
                assert_eq!(
                    rand_bool_vector
                        .values
                        .iter()
                        .filter(|&n| *n == true)
                        .count(),
                    (test_sp * test_size as f32) as usize
                );
            } else {
                assert!(false, "Expected to get bool vector");
            }
        }
    }

    #[test]
    fn random_float_vector_is_generated() {
        let test_size = 100;
        let test_mean = 0.5;
        let test_stddev = 0.01;
        if let Some(rand_vector) =
            CodeGenerator::random_float_vector(test_size, test_mean, test_stddev)
        {
            assert_eq!(rand_vector.values.len(), test_size as usize);
        } else {
            assert!(false, "Expected to get int vector");
        }
    }

    #[test]
    fn random_int_vector_is_generated() {
        let test_size = 100;
        let test_min = 5;
        let test_max = 11;
        if let Some(rand_vector) = CodeGenerator::random_int_vector(test_size, test_min, test_max) {
            assert_eq!(rand_vector.values.len(), test_size as usize);
        } else {
            assert!(false, "Expected to get int vector");
        }
    }

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
