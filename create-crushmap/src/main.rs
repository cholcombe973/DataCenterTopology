extern crate crushtool;
#[macro_use]
extern crate log;
extern crate juju;

use std::collections::{HashMap, HashSet};
use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::process::Command;

/*
Here is where the controller takes input from the subordinate services,
determines which nodes are in the same failure domain, and finally
creates the crushmap from those clusters.

*/

fn main (){

    let juju_relation_ids = juju::relation_ids_by_identifier("controller").unwrap();
    let relation_id = &juju_relation_ids[0];
    let controller_id = env::var("JUJU_UNIT_NAME").unwrap_or("".to_string());

    let controller = parse_unit_into_relation(controller_id);

    let juju_related_units = juju::relation_get_by_id("related-units", &relation_id, &controller).unwrap();
    let mut juju_parsed_units: Vec<juju::Relation> = vec![];

    for unit in juju_related_units.split_whitespace() {
        juju_parsed_units.push(parse_unit_into_relation(unit.to_string()));
    }

    let mut machines: HashMap<String, Vec<String>> = HashMap::new();

    for unit in juju_parsed_units {
        let hostname = juju::relation_get_by_id("hostname", &relation_id, &unit).unwrap();
        let neighbors_raw = juju::relation_get_by_id("neighbors", &relation_id, &unit).unwrap();
        let hostname_trimmed = hostname.trim_matches('\n').trim();
        let neighbors_trimmed = neighbors_raw.trim_matches('\n').trim();

        let neighbors: Vec<String> = neighbors_trimmed.split_whitespace()
                                                    .map(|item| item.to_owned())
                                                    .collect();

        println!("Hostname:{}, Neighbors:{:?}", hostname_trimmed, neighbors_trimmed);
        machines.insert(hostname_trimmed.to_owned(), neighbors);
    }

    let mut racks: HashMap<usize, HashSet<String>> = HashMap::new();
    let mut potential_racks: Vec<HashSet<String>> = vec![];
    let mut rack_id: usize = 0;

    for (machine, neighbors) in machines {
        let mut members = HashSet::new();
        members.insert(machine);
        for neighbor in neighbors {
            members.insert(neighbor.clone());
        }
        potential_racks.push(members);
    }
    println!("Potential racks: {:?}", potential_racks);

    racks.insert(rack_id, potential_racks[0].clone());
    rack_id += 1;
    let new_racks = racks.clone();
    for rack in potential_racks.iter() {
        for (_, existing_rack) in new_racks.iter() {
            if rack == existing_rack {
                break;
            }
            racks.insert(rack_id, rack.clone());
            rack_id += 1;
        }
    }
    println!("Racks: {:?}", racks);

    let crush_result = match generate_crushmap(racks) {
        Ok(_) => { juju::status_set(juju::Status{ status_type: juju::StatusType::Maintenance,
                                    message: "Crushmap generated in \"/usr\". Please examine crushmap with Ceph before use."
                                        .to_string()});
                },
        Err(e) => { let message = format!("Failed to create crushmap with error: {}", e);
                    juju::status_set(juju::Status{ status_type: juju::StatusType::Maintenance,
                     message: message});
                }
    };

}



fn parse_unit_into_relation(unit: String) -> juju::Relation {
    let v: Vec<&str> = unit.split('/').collect();
    let id: usize = v[1].parse::<usize>().unwrap();
    let parsed_unit = juju::Relation {
        name: v[0].to_string(),
        id: id,
    };
    parsed_unit
}

fn generate_crushmap(racks: HashMap<usize, HashSet<String>>) -> Result<(), String> {

    Command::new("ceph")
                .current_dir("/usr")
                .args(&["osd", "getcrushmap", "-o", "dctmap.txt"])
                .spawn()
                .expect("failed to grab current cruhsmap");


    let path = Path::new("/usr/dctmap.txt");
    let some_crushmap_file = File::open(path).unwrap();
    let mut crushmap_bytes: Vec<u8> = Vec::new();
    for byte in some_crushmap_file.bytes() {
        crushmap_bytes.push(byte.unwrap());
    }
    let current_map: crushtool::CrushMap = try!(crushtool::decode_crushmap(&crushmap_bytes[..]));

    let mut current_index: i32 = match current_map.name_map.iter().min() {
        Some(&(index, _)) => index,
        None => {
            return Err("Cannot proceed due to error: Could not find current index.
                    Either no bucket items are present or map decode failed to generate meaningful
                    buckets.". to_string());
        }
    };

    let mut name_map: HashMap<String, i32> = HashMap::new();
    let mut machines_map: HashMap<String, i32> = HashMap::new();
    let mut machines: HashSet<String> = HashSet::new();
    let mut current_buckets = current_map.buckets.clone();

    for (index, name) in current_map.name_map {
        name_map.insert(name, index);
    };

    for (_, members) in racks.clone() {
        machines.extend(members);
    }

    for (id, index) in name_map.clone() {
        if machines.contains(&id){
            machines_map.insert(id.clone(), index.clone());
        }
    }

    let mut host_bucket_list: HashMap<String, crushtool::BucketTypes> = HashMap::new();

    for (name, index) in &name_map {
        for bucket in current_map.buckets.clone() {
            let id: i32;
            match bucket {
                crushtool::BucketTypes::Uniform(ref uniform) => {
                    id = uniform.bucket.id;
                }
                crushtool::BucketTypes::List(ref list) => {
                    id = list.bucket.id;
                }
                crushtool::BucketTypes::Tree(ref tree) => {
                    id = tree.bucket.id;
                }
                crushtool::BucketTypes::Straw(ref straw) => {
                    id = straw.bucket.id;
                }
                crushtool::BucketTypes::Unknown => {
                    id = 65536;
                }
            }
            if &id == index {
                host_bucket_list.insert(name.clone(), bucket.clone());
            }

        }
    }

    let mut new_rack_buckets: Vec<(i32, Option<String>)> = Vec::new();

    for (id, members) in racks {
        let name = id.to_string();
        let mut bucket_items: Vec<(i32, Option<String>)> = Vec::new();

        for machine in members.clone() {
            let index = machines_map.get(&machine);
            bucket_items.push((*index.unwrap(), Some(machine.to_string())));
        }

        let bucket = crushtool::BucketTypes::Straw(crushtool::CrushBucketStraw {
            bucket: crushtool::Bucket {
                id: current_index,
                bucket_type: crushtool::OpCode::Take,
                alg: crushtool::BucketAlg::Straw,
                hash: crushtool::CrushHash::RJenkins1,
                weight: 0,
                size: members.len() as u32,
                items: bucket_items,
                perm_n: 0,
                perm: members.len() as u32,
            },
            item_weights: vec![(0, 0), (0, 0), (0, 0)]

        });
        new_rack_buckets.push((current_index, Some(name.to_string())));
        current_buckets.push(bucket);
        name_map.insert(name, current_index);
        current_index += -1;
    }

    let new_default_bucket = crushtool::BucketTypes::Straw(crushtool:: CrushBucketStraw {
        bucket: crushtool::Bucket {
            id: -1,
            bucket_type: crushtool::OpCode::SetChooseLocalTries,
            alg: crushtool::BucketAlg::Straw,
            hash: crushtool::CrushHash::RJenkins1,
            weight: 0,
            size: new_rack_buckets.len() as u32,
            items: new_rack_buckets.clone(),
            perm_n: 0,
            perm: new_rack_buckets.len() as u32,
        },
        item_weights: vec![(0, 0), (0, 0), (0, 0)]
    });

    current_buckets.push(new_default_bucket);
    let mut final_name_map: Vec<(i32, String)> = Vec::new();

    for (name, index) in name_map.clone() {
        final_name_map.push((index, name));

    }

    let new_crushmap: crushtool::CrushMap = crushtool::CrushMap {
        magic: 65536,
        max_buckets: 8,
        max_rules: 1,
        max_devices: 3,
        buckets: current_buckets,
        rules: vec![Some(crushtool::Rule {
            len: 3,
            mask: crushtool::CrushRuleMask {
                ruleset: 0,
                rule_type: crushtool::RuleType::Replicated,
                min_size: 1,
                max_size: 10,
            },
            steps: vec![crushtool::CrushRuleStep {
                op: crushtool::OpCode::Take,
                arg1: (-1, None),
                arg2: (0, None),
            },
            crushtool::CrushRuleStep {
                op: crushtool::OpCode::ChooseLeafFirstN,
                arg1: (0, None),
                arg2: (1, None),
            },
            crushtool::CrushRuleStep {
                op: crushtool::OpCode::Emit,
                arg1: (0, None),
                arg2: (0, None),
            }],
        })],
        type_map: vec![(0, "osd".to_string()),
            (1, "host".to_string()),
            (2, "chassis".to_string()),
            (3, "rack".to_string()),
            (4, "row".to_string()),
            (5, "pdu".to_string()),
            (6, "pod".to_string()),
            (7, "room".to_string()),
            (8, "datacenter".to_string()),
            (9, "region".to_string()),
            (10, "root".to_string())],

        name_map: final_name_map,
        rule_name_map: vec![(0, "replicated_ruleset".to_string())],
        choose_local_tries: Some(0),
        choose_local_fallback_tries: Some(0),
        choose_total_tries: Some(50),
        chooseleaf_descend_once: Some(1),
        chooseleaf_vary_r: Some(0),
        straw_calc_version: Some(1),
        choose_tries: None,
    };

    let encoded_crushmap = crushtool::encode_crushmap(new_crushmap).unwrap();
    let mut finished_map = try!(File::create("/usr/dctmap_output.txt").map_err(|e| e.to_string()));

    try!(finished_map.write_all(&encoded_crushmap[..]).map_err(|e| e.to_string()));

    Ok(())

}

