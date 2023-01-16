use std::collections::{hash_map::Entry, HashMap};
use chrono::Local;
use rand::prelude::*;
use rand_seeder::Seeder;
use rand_pcg::Pcg64;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Member {
    pub name: &'static str,
    pub id: i32,
}

fn get_members() -> Vec<Member> {
    vec!["Vanini", "Gamerdinger", "Henriette", "Jon"]
        .iter()
        .enumerate()
        .map(|(id, name)| Member {
            name,
            id: id as i32,
        })
        .collect()
}

#[derive(Debug)]
pub enum Area {
    Kitchen,
    Bathroom,
    LivingRoom,
    Entrance,
    Outdoor,
    Everywhere,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Group {
    Bathroom,
    Trashs,
    WipeKitchen,
    Vacuum,
    Outdoor,
    Other,
}

#[derive(Debug)]
pub struct Task {
    pub area: Area,
    pub name: &'static str,
    pub difficulty: i32,

    // If you get a task from a group, it makes sense you get all of that same group
    pub group: Group
}

#[derive(Debug)]
pub struct CollectiveInformation {
    pub tasks: Vec<Task>,
    pub members: Vec<Member>,
    pub number_of_task: i32,
    pub num_members: i32,
    pub total_complexity: i32,
    pub max_complexity_per_member: i32,
}

// unsure yet how to store that
pub fn get_tasks_for_member() {
    let data = get_colletive_information();

    println!("{data:?}");
    
    // todo what if they want to clean on the monday?
    let date = Local::now().date_naive();
    let date_string = date.format("%Y/%m/%U").to_string();

    let mut used_task: HashSet<i32> = HashSet::new();

    data.members.into_iter().for_each(|member| {
        // Generates a custom seed based on
        // Name - Year/Month/Week
        // This way, the result will only vary from a week to another,
        // for each member.
        let mut rng: Pcg64 = Seeder::from(format!("{}{date_string}", member.name)).make_rng();
        let mut index = rng.gen_range(0..data.number_of_task);
        let mut current_tasks_difficulty = 0;
        let mut member_current_task: Vec<&Task> = Vec::new();
        
        println!("----{}---", member.name);
        while current_tasks_difficulty < data.max_complexity_per_member {
            while let Some(_) = used_task.get(&index) {
                index = rng.gen_range(0..data.number_of_task);
            }
            used_task.insert(index);
            let task = data.tasks.get(index as usize).unwrap();
            current_tasks_difficulty += task.difficulty;
            member_current_task.push(task);
    
            
            if task.group != Group::Other {
                // fetch all the tasks that belong to that group
                data.tasks.iter().enumerate().filter(|(_, x)| x.group == task.group).for_each(|(i, t)| {
                    current_tasks_difficulty += t.difficulty;
                    used_task.insert(i as i32);
                    member_current_task.push(t);
                });
            }
        }
        println!("{member_current_task:?}");
        println!("{current_tasks_difficulty:?}");
    });
}

fn get_colletive_information() -> CollectiveInformation {
    let mut tasks: Vec<Task> = Vec::new();
    let mut number_of_task = 0;
    let mut total_complexity = 0;

    for (area, name, difficulty, group) in vec![
        (Area::Bathroom, "Clean mirror", 1, Group::Bathroom),
        (Area::Bathroom, "Clean sink + tap", 1, Group::Bathroom),
        (Area::Bathroom, "Clean shower (Floor - Shower head)", 2, Group::Bathroom),
        (Area::Bathroom, "Wipe all surfaces", 1, Group::Bathroom),
        (Area::Bathroom, "Clean toilet", 4, Group::Bathroom),
        (Area::Bathroom, "Empty trash bin", 1, Group::Trashs),
        (Area::Bathroom, "Vacuum floor + wash", 2, Group::Vacuum),
        (Area::Kitchen, "Descale and clean Kettle ", 1, Group::Other),
        (Area::Kitchen, "Clean Toaster", 1, Group::Other),
        (Area::Kitchen, "Clean Oven + Trays ", 2, Group::Other),
        (Area::Kitchen, "Clean Sink", 1, Group::Other),
        (Area::Kitchen, "Clean micro", 1, Group::Other),
        (Area::Kitchen, "Clean Common shelves in fridge", 2, Group::Other),
        (Area::Kitchen, "Empty Trash + Bio Trash + clean bio bin", 4, Group::Trashs),
        (Area::Kitchen, "Empty Recycling + clean bins", 2, Group::Trashs),
        (Area::Kitchen, "Clean under sink", 1, Group::Trashs),
        (Area::Kitchen, "Clean the 3 vases (cloths and dish brush + onion + pot spoon, palette knife ect)", 1, Group::Other),
        (Area::Kitchen, "Kitchen counter area: Wipe all surfaces + panels", 2, Group::WipeKitchen),
        (Area::Kitchen, "Table area: Wipe all surfaces + panels", 1, Group::WipeKitchen),
        (Area::Kitchen, "Vacuum floor + wash", 3, Group::Vacuum),
        (Area::LivingRoom, "Wipe all surfaces Living room (incl. panels)", 1, Group::Other),
        (Area::LivingRoom, "Vacuum sofa and chair", 2, Group::Vacuum),
        (Area::LivingRoom, "Vacuum floor + wash", 2, Group::Vacuum),
        (Area::LivingRoom, "Clean shoe rack, wipe surfaces hall way (incl. panels)", 2, Group::Other),
        (Area::Kitchen, "Wash towels + Cloths (90 degrees)", 1, Group::Other),
        (Area::Outdoor, "Refund bottles and cans", 3, Group::Other),
        (Area::Outdoor, "Shopping (have a look + shoppinglist)", 2, Group::Other),
        (Area::Everywhere, "Water plants in common areas", 1, Group::Other),
    ] {
        let task = Task { area, name, group, difficulty };
        tasks.push(task);

        number_of_task += 1;
        total_complexity += difficulty;
    }

    let members = get_members();
    let num_members = members.len() as i32;

    CollectiveInformation {
        tasks,
        number_of_task,
        total_complexity,
        members,
        num_members,
        max_complexity_per_member: total_complexity / num_members,
    }
}


// ChatGPT Generated
// Since the list of tasks and their weight won't change
// we don't need to go full subset sum. A stupid brutforce will do
fn separate_into_slices(numbers: &[i32], x: i32) -> Vec<&[i32]> {
    let mut result = vec![];
    let mut current_sum = 0;
    let mut current_slice_start = 0;
    let mut current_slice_end;

    for (i, &num) in numbers.iter().enumerate() {
        current_sum += num;
        current_slice_end = i;

        if current_sum >= x {
            result.push(&numbers[current_slice_start..=current_slice_end]);
            current_slice_start = i + 1;
            current_sum = 0;
        }
    }
    if current_slice_start < numbers.len() {
        result.push(&numbers[current_slice_start..numbers.len()]);
    }
    result
}