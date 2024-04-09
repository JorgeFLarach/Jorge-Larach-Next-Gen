use std::time::Instant;

fn main() {}

struct Coll {
    list: Vec<i32>
}

fn copy_vector(v: Vec<i32>) {}
fn borrow_vector(v: &Vec<i32>) {}
fn copy_struct(coll: Coll) {}
fn borrow_struct(coll: &Coll) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn benchmark_struct_comparison() {
        let s = Coll { list: vec![1; 1000000]};
        
        let borrow_start = Instant::now();
        borrow_struct(&s);
        let borrow_duration = borrow_start.elapsed();
    
        
        let copy_start = Instant::now();
        copy_struct(s);
        let copy_duration = copy_start.elapsed();
    
        println!("borrow struct: {:?}", borrow_duration);
        println!("copy struct: {:?}", copy_duration);

        assert!(borrow_duration < copy_duration);
    }

    #[test]
    fn benchmark_vec_comparison() {
        let v = vec![1; 1000000];

        let borrow_start = Instant::now();
        borrow_vector(&v);
        let borrow_duration = borrow_start.elapsed();
    
        
        let copy_start = Instant::now();
        copy_vector(v);
        let copy_duration = copy_start.elapsed();
        
        println!("borrow struct: {:?}", borrow_duration);
        println!("copy struct: {:?}", copy_duration);

        assert!(borrow_duration < copy_duration);

    }
}