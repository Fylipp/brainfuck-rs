use ::std;
use super::BrainfuckMachine;
use super::super::constants::*;

#[test]
fn new() {
    let instr: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let bfm = BrainfuckMachine::new_dummy(instr);

    assert_eq!(0, bfm.ptr);
    assert_eq!(0, bfm.cells.iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(0, bfm.ip);
    assert_eq!(0, bfm.stack.len());
    assert_eq!(false, bfm.done);
}

#[test]
fn incr_ptr() {
    let instr: &[u8] = &[INCR_PTR];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();

    assert_eq!(1, bfm.ptr);
    assert_eq!(0, bfm.cells.iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(1, bfm.ip);
    assert_eq!(0, bfm.stack.len());
    assert_eq!(true, bfm.done);
}

#[test]
fn incr_ptr_wrap() {
    let instr: &[u8] = &[DECR_PTR, INCR_PTR];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();

    assert_eq!(0, bfm.ptr);
    assert_eq!(0, bfm.cells.iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(2, bfm.ip);
    assert_eq!(0, bfm.stack.len());
    assert_eq!(true, bfm.done);
}

#[test]
fn decr_ptr() {
    let instr: &[u8] = &[INCR_PTR, DECR_PTR];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();

    assert_eq!(0, bfm.ptr);
    assert_eq!(0, bfm.cells.iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(2, bfm.ip);
    assert_eq!(0, bfm.stack.len());
    assert_eq!(true, bfm.done);
}

#[test]
fn decr_ptr_wrap() {
    let instr: &[u8] = &[DECR_PTR];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();

    assert_eq!(std::u16::MAX, bfm.ptr);
    assert_eq!(0, bfm.cells.iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(1, bfm.ip);
    assert_eq!(0, bfm.stack.len());
    assert_eq!(true, bfm.done);
}

#[test]
fn incr_val_at_0() {
    let instr: &[u8] = &[INCR_VAL];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();

    assert_eq!(0, bfm.ptr);
    assert_eq!(1, bfm.cells[0]);
    assert_eq!(0, bfm.cells[1..].iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(1, bfm.ip);
    assert_eq!(0, bfm.stack.len());
    assert_eq!(true, bfm.done);
}

#[test]
fn incr_val_at_2() {
    let instr: &[u8] = &[INCR_PTR, INCR_PTR, INCR_VAL];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();

    assert_eq!(2, bfm.ptr);
    assert_eq!(0, bfm.cells[0]);
    assert_eq!(0, bfm.cells[1]);
    assert_eq!(1, bfm.cells[2]);
    assert_eq!(0, bfm.cells[3..].iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(3, bfm.ip);
    assert_eq!(0, bfm.stack.len());
    assert_eq!(true, bfm.done);
}

#[test]
fn incr_val_wrap() {
    let instr: &[u8] = &[DECR_VAL, INCR_VAL];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();

    assert_eq!(0, bfm.ptr);
    assert_eq!(0, bfm.cells[0..].iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(2, bfm.ip);
    assert_eq!(0, bfm.stack.len());
    assert_eq!(true, bfm.done);
}

#[test]
fn decr_val_at_0() {
    let instr: &[u8] = &[INCR_VAL, INCR_VAL, DECR_VAL];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();

    assert_eq!(0, bfm.ptr);
    assert_eq!(1, bfm.cells[0]);
    assert_eq!(0, bfm.cells[1..].iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(3, bfm.ip);
    assert_eq!(0, bfm.stack.len());
    assert_eq!(true, bfm.done);
}

#[test]
fn decr_val_at_2() {
    let instr: &[u8] = &[INCR_PTR, INCR_PTR, INCR_VAL, INCR_VAL, DECR_VAL];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();

    assert_eq!(2, bfm.ptr);
    assert_eq!(0, bfm.cells[0]);
    assert_eq!(0, bfm.cells[1]);
    assert_eq!(1, bfm.cells[2]);
    assert_eq!(0, bfm.cells[3..].iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(5, bfm.ip);
    assert_eq!(0, bfm.stack.len());
    assert_eq!(true, bfm.done);
}

#[test]
fn decr_val_wrap() {
    let instr: &[u8] = &[DECR_VAL];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();

    assert_eq!(0, bfm.ptr);
    assert_eq!(std::u8::MAX, bfm.cells[0]);
    assert_eq!(0, bfm.cells[1..].iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(1, bfm.ip);
    assert_eq!(0, bfm.stack.len());
    assert_eq!(true, bfm.done);
}

#[test]
fn start_loop_at_0() {
    let instr: &[u8] = &[INCR_VAL, LOOP_START];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();

    assert_eq!(0, bfm.ptr);
    assert_eq!(1, bfm.cells[0]);
    assert_eq!(0, bfm.cells[1..].iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(2, bfm.ip);
    assert_eq!(1, bfm.stack.len());
    assert_eq!(1, bfm.stack[0]);
    assert_eq!(true, bfm.done);
}

#[test]
fn start_loop_at_offset() {
    let instr: &[u8] = &[INCR_PTR, INCR_PTR, INCR_VAL, LOOP_START];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();

    assert_eq!(2, bfm.ptr);
    assert_eq!(0, bfm.cells[0]);
    assert_eq!(0, bfm.cells[1]);
    assert_eq!(1, bfm.cells[2]);
    assert_eq!(0, bfm.cells[3..].iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(4, bfm.ip);
    assert_eq!(1, bfm.stack.len());
    assert_eq!(3, bfm.stack[0]);
    assert_eq!(true, bfm.done);
}

#[test]
fn start_loop_false() {
    let instr: &[u8] = &[LOOP_START, INCR_VAL, INCR_PTR, LOOP_END, INCR_VAL, INCR_PTR];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();

    assert_eq!(1, bfm.ptr);
    assert_eq!(1, bfm.cells[0]);
    assert_eq!(0, bfm.cells[1..].iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(6, bfm.ip);
    assert_eq!(0, bfm.stack.len());
    assert_eq!(true, bfm.done);
}

#[test]
fn start_nested_loop_false() {
    let instr: &[u8] = &[INCR_VAL, LOOP_START, INCR_PTR, LOOP_START, INCR_VAL, LOOP_END, LOOP_END];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();

    assert_eq!(1, bfm.ptr);
    assert_eq!(1, bfm.cells[0]);
    assert_eq!(0, bfm.cells[1..].iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(7, bfm.ip);
    assert_eq!(0, bfm.stack.len());
    assert_eq!(true, bfm.done);
}

#[test]
#[should_panic]
fn start_loop_false_no_end() {
    let instr: &[u8] = &[LOOP_START, INCR_VAL, INCR_PTR, INCR_VAL, INCR_PTR];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();

    assert_eq!(1, bfm.ptr);
    assert_eq!(0, bfm.cells[0..].iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(6, bfm.ip);
    assert_eq!(0, bfm.stack.len());
    assert_eq!(true, bfm.done);
}

#[test]
fn end_loop_false() {
    let instr: &[u8] = &[LOOP_START, LOOP_END];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();

    assert_eq!(0, bfm.ptr);
    assert_eq!(0, bfm.cells[0..].iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(2, bfm.ip);
    assert_eq!(0, bfm.stack.len());
    assert_eq!(true, bfm.done);
}

#[test]
#[should_panic]
fn end_loop_false_no_start() {
    let instr: &[u8] = &[LOOP_END];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();
}

#[test]
fn end_loop_true() {
    let instr: &[u8] = &[INCR_VAL, INCR_VAL, INCR_VAL, LOOP_START, INCR_PTR, INCR_VAL, DECR_PTR, DECR_VAL, LOOP_END];

    let mut bfm = BrainfuckMachine::new_dummy(instr);
    bfm.execute_all();

    assert_eq!(0, bfm.ptr);
    assert_eq!(0, bfm.cells[0]);
    assert_eq!(3, bfm.cells[1]);
    assert_eq!(0, bfm.cells[2..].iter().filter(|cell| **cell != 0).count());
    assert_eq!(instr, bfm.instr);
    assert_eq!(9, bfm.ip);
    assert_eq!(0, bfm.stack.len());
    assert_eq!(true, bfm.done);
}
