fn main() {

    let orbits : Vec<(String, String)> = std::fs::read_to_string("inputs.txt").unwrap().lines().map(|x| {
        let mut split = x.split(")");
        let centre = split.next().unwrap();
        let orbitter = split.next().unwrap();

        println!("{} orbits {}", orbitter, centre);

        (centre.to_string(), orbitter.to_string())
    }).collect();

    let mut orbit_map = std::collections::HashMap::<String, String>::new();

    for (centre, orbitter) in orbits.iter() {
        orbit_map.insert(orbitter.clone(), centre.clone());
    }

    let mut distance_map = std::collections::HashMap::<String, u64>::new();

    let mut total_distance = 0;
    for orbitter in orbit_map.keys() {
        let dist = get_distance(orbitter.to_string(), &orbit_map, &mut distance_map);
        println!("Distance of {} is {}", orbitter, dist);
        total_distance += dist;
    }

    let mut my_route = Vec::<_>::new();

    let mut orbitter = "YOU".to_string();
    loop {
        println!("Checking my route for {}", orbitter);
        match orbit_map.get(&orbitter) {
            Some(c) => {
                my_route.push(c.to_string());
                orbitter = c.to_string();
            },
            None => {
                break;
            }
        }        
    }

    println!("My route is {:?}", my_route);

    let mut orbitter = "SAN".to_string();
    let mut count = 0;
    loop {
        println!("Checking Santa route for {}", orbitter);
        if my_route.contains(&orbitter) {
            println!("On my route!");
            break;
        }
        match orbit_map.get(&orbitter) {
            Some(c) => {
                count += 1;
                orbitter = c.to_string();
            },
            None => {
                break;
            }
        }
    }

    println!("Count is {}", count);
    println!("{}", count + distance_map.get("YOU").unwrap() - distance_map.get(&orbitter).unwrap() - 2);

    println!("Total distance {}", total_distance);
}

fn get_distance(orbitter: String, 
                orbit_map: &std::collections::HashMap::<String, String>, 
                distance_map: &mut std::collections::HashMap::<String, u64>) -> u64 {
    match distance_map.get(&orbitter) {
        Some(d) => {
            println!("Got dist {}: {}", orbitter, d);
            *d
        },
        None => {
            match orbit_map.get(&orbitter) {
                Some(center) => {
                    println!("Looking up dist for {}", center);
                    let dist = get_distance(center.to_string(), orbit_map, distance_map) + 1;
                    println!("Inferred dist {}: {}", orbitter, dist);
                    distance_map.insert(orbitter, dist);
                    dist
                },
                None => {
                    println!("Not found in orbit map!");
                    0
                }
            }
        }
    }
}