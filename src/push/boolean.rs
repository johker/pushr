use crate::push::atoms::{Atom, PushType};
use crate::push::state::PushState;
use rand::Rng;

//
// ------------------ Type: BOOLEAN ---------------------
//

pub fn boolean_eq(push_state: &mut PushState) {
    if let Some(pv) = push_state.bool_stack.pop_vec(2) {
        push_state.bool_stack.push(pv[0] == pv[1]);
    }
}

pub fn boolean_and(push_state: &mut PushState) {
    if let Some(pv) = push_state.bool_stack.pop_vec(2) {
        push_state.bool_stack.push(pv[0] && pv[1]);
    }
}

pub fn boolean_or(push_state: &mut PushState) {
    if let Some(pv) = push_state.bool_stack.pop_vec(2) {
        push_state.bool_stack.push(pv[0] || pv[1]);
    }
}

pub fn boolean_def(push_state: &mut PushState) {
    if let Some(name) = push_state.name_stack.pop() {
        if let Some(bval) = push_state.bool_stack.pop() {
            push_state.name_bindings.insert(
                name,
                Atom::Literal {
                    push_type: PushType::PushBoolType { val: bval },
                },
            );
        }
    }
}

pub fn boolean_dup(push_state: &mut PushState) {
    if let Some(pv) = push_state.bool_stack.observe_vec(1) {
        push_state.bool_stack.push(pv[0]);
    }
}

pub fn boolean_flush(push_state: &mut PushState) {
    push_state.bool_stack.flush();
}

pub fn boolean_from_float(push_state: &mut PushState) {
    if let Some(pv) = push_state.float_stack.observe_vec(1) {
        // TODO: Float comparison?
        let x = pv[0] == 0.0;
        push_state.bool_stack.push(x);
    }
}

pub fn boolean_from_integer(push_state: &mut PushState) {
    if let Some(pv) = push_state.int_stack.observe_vec(1) {
        let x = pv[0] == 0;
        push_state.bool_stack.push(x);
    }
}

pub fn boolean_not(push_state: &mut PushState) {
    if let Some(pv) = push_state.bool_stack.pop() {
        push_state.bool_stack.push(!pv);
    }
}

pub fn boolean_pop(push_state: &mut PushState) {
    push_state.bool_stack.pop();
}

pub fn boolean_rand(push_state: &mut PushState) {
    let mut rng = rand::thread_rng();
    let bval = rng.gen_range(0..2) == 1;
    push_state.bool_stack.push(bval);
}
