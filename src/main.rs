
use std::fs::read_to_string;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::ptr::hash;

#[derive(Serialize, Deserialize,PartialEq, Clone, )]
struct Bid {
    items: HashSet<u16>,
    value: i32
}

fn load_bids(path: &str) -> Vec<Bid>{
    let f = read_to_string(path).unwrap();
    let bids : Vec<Bid> = serde_json::from_str(&f).unwrap();
    return bids
}
fn wdp(bids: &Vec<Bid>) -> Vec<usize>{

    let mut f = 0; // highest value sum
    let mut best : Vec<usize> = vec![];
    for b in 0..bids.len(){

        let mut path = vec![];
        path.push(b);
        let mut available_bids = prune_bids(&path, (0..bids.len()).collect() , bids);
        let mut p_sum = bids[b].value;
        //let bar = ProgressBar::new(available_bids.len() as u64);
        '_inner: while available_bids.len() > 0 {
            //bar.inc(1);
            path.push(available_bids[0].clone());
            p_sum += bids[available_bids[0]].value;
            available_bids = prune_bids(&path, available_bids, bids);

            let a_sum = bid_sum(&available_bids, bids);
            if (p_sum + a_sum) < f {
                break '_inner
            }
            if p_sum > f {
                f = p_sum;
                best = path.clone();

            }
        }


    }
    return best
}

fn prune_bids(path:&Vec<usize>, list_to_check:Vec<usize>, bids:&Vec<Bid>) -> Vec<usize>{
    let mut not_sold : Vec<usize> = vec![];
    let mut sold: HashSet<u16> = HashSet::new();
    for b in path{
        sold.extend(bids[*b].items.clone())
    }
    for b in list_to_check{

        if ! sold.intersection(&bids[b].items).next().is_some(){
            not_sold.push(b)
        }
    }
    return not_sold;

}




fn bid_sum(bids_idx: &Vec<usize>, bids : &Vec<Bid>) -> i32{
    let mut sum = 0;
    for b in bids_idx {
        sum += bids[*b].value;
    }
    return sum
}


/*

fn check_valid(bids: &Vec<Bid>) -> bool{
    for (i,b) in bids.iter().enumerate(){
        for j in 0..b.items.len() {
            for k in i..bids.len() {
                if bids[k].items.contains(&b.items[j]){
                    return false
                }
            }
        }
    }
    return true
} */
fn main() {

    let mut times : Vec<f32> = vec![];
    let mut results : Vec<i32> = vec![];
    for i in 1..19{
        let s = format!("bids/bids{:0>2}-ID.json", i);
        let mut bids = load_bids(&s);
        bids.sort_by(|a, b| b.value.cmp(&a.value));
        println!("Loaded and sorted bids{:0>2}.json", i);
        let now = Instant::now();
        let w = wdp(&bids.clone());
        let elapsed_time = now.elapsed();
        let sum = bid_sum(&w, &bids);
        println!("Running slow_function() took {} seconds.", elapsed_time.as_secs_f32());
        println!("{}", sum);
        times.push(elapsed_time.as_secs_f32());
        results.push(sum);
    }

    println!("{:?}", times);
    println!("{:?}", results);




}