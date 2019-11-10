use std::io;
 
fn main()
{
    let n: usize;
    let q: usize;
    {
        let mut buf = String::new();
        let _ = io::stdin().read_line(&mut buf);
 
        let n_q: Vec<_> = buf.split_whitespace().collect();
        n = n_q[0].parse().unwrap();
        q = n_q[1].parse().unwrap();
    }
 
    let mut query = Vec::new();
 
    for _ in 0..q
    {
        let mut buf = String::new();
        let _ = io::stdin().read_line(&mut buf);
 
        let qu: Vec<_> = buf.split_whitespace()
            .map(|i| i.parse().unwrap()).collect();
        query.push(qu);
    }
 
    query.reverse();
 
    for i in 1..(n+1)
    {
        let mut found = false;
 
        for q in query.iter()
        {
            if q[0] <= i && i <= q[1]
            {
                println!("{}", q[2]);
                found = true;
                break;
            }
        }
 
        if found == false
        {
            println!("0");
        }
    }
}