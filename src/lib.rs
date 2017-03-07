extern crate csv;
extern crate rustc_serialize;

use std::hash::Hash;
use std::fs::File;
use std::collections::{HashMap, HashSet};
use rustc_serialize::Decodable;

#[test]
fn it_works() {
    let reader = csv::Reader::from_file("./data/movielens.csv").unwrap().delimiter(b' ');
    let data_handler: BasicDataHandler<usize, usize> = BasicDataHandler::new(reader);
    println!("{:?}", data_handler.get_user_ratings(326));
}

pub trait DataHandler<U, I> {
    fn get_user_ids(&self) -> &HashSet<U>;
    fn get_item_ids(&self) -> &HashSet<I>;
    fn get_user_ratings(&self, user_id: U) -> HashMap<I, f64>;
    fn get_item_ratings(&self, item_id: I) -> HashMap<U, f64>;
    fn get_rating(&self, user_id: U, item_id: I) -> f64;
    fn get_num_users(&self) -> usize;
    fn get_num_items(&self) -> usize;
    fn add_rating(&mut self, user_id: U, item_id: I, rating: f64);
    fn remove_rating(&mut self, user_id: U, item_id: I);
}

pub struct BasicDataHandler<U: Eq + Hash + Decodable + Clone, I: Eq + Hash + Decodable + Clone> {
    user_ids: HashSet<U>,
    item_ids: HashSet<I>,
    ratings: HashMap<(U, I), f64>
}

impl<U: Eq + Hash + Decodable + Clone, I: Eq + Hash + Decodable + Clone> DataHandler<U, I> for BasicDataHandler<U, I> {
    fn get_user_ids(&self) -> &HashSet<U> {
        &self.user_ids
    }
    fn get_item_ids(&self) -> &HashSet<I> {
        &self.item_ids
    }
    fn get_user_ratings(&self, user_id: U) -> HashMap<I, f64> {
        self.ratings.iter().filter_map(|(k, v)| {
            if k.0 == user_id {
                return Some((k.1.clone(), *v));
            }
            return None;
        }).collect()
    }
    fn get_item_ratings(&self, item_id: I) -> HashMap<U, f64> {
        self.ratings.iter().filter_map(|(k, v)| {
            if k.1 == item_id {
                return Some((k.0.clone(), *v));
            }
            return None;
        }).collect()
    }
    fn get_rating(&self, user_id: U, item_id: I) -> f64 {
        *self.ratings.get(&(user_id, item_id)).unwrap_or(&-1.0)
    }
    fn get_num_users(&self) -> usize {
        self.user_ids.len()
    }
    fn get_num_items(&self) -> usize {
        self.item_ids.len()
    }
    fn add_rating(&mut self, user_id: U, item_id: I, rating: f64) {
        self.ratings.insert((user_id, item_id), rating);
    }
    fn remove_rating(&mut self, user_id: U, item_id: I) {
        self.ratings.remove(&(user_id, item_id));
    }
}

impl<U: Eq + Hash + Decodable + Clone, I: Eq + Hash + Decodable + Clone> BasicDataHandler<U, I> {
    pub fn new(mut reader: csv::Reader<File>) -> BasicDataHandler<U, I> {
        let mut user_ids: HashSet<U> = HashSet::new();
        let mut item_ids: HashSet<I> = HashSet::new();
        let mut ratings: HashMap<(U, I), f64> = HashMap::new();
            
        for row in reader.decode() {
            let (user_id, item_id, rating): (U, I, f64) = row.unwrap();
            user_ids.insert(user_id.clone());
            item_ids.insert(item_id.clone());
            ratings.insert((user_id, item_id), rating);
        }
        BasicDataHandler {
            user_ids: user_ids,
            item_ids: item_ids,
            ratings: ratings
        }
    }
}