// Create the transmission and receiver channels in order to send packets
let (mut tx, mut rx) = match datalink::channel(&interface, &Default::default()) {
Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
Ok(_) => panic!("Unhandled channel type"),
Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
};


/*#[derive(Debug)]


let mut container = Vec::new();
let mut nodes: Vec<Node> = Vec::new();

{
    let file = File::open("/proc/net/arp").unwrap();


    for line in BufReader::new(&file).lines() {
        container.push(line.unwrap());
    }
}


for item in container {

    let mut split = item.split_whitespace().collect::<Vec<&str>>();
    split.remove(0);
    let ip = split[0].split(".").collect::<Vec<&str>>();

    let node = Node {
        IPaddr: Ipv4Addr::new(ip[0].parse::<u8>().unwrap(),ip[1].parse::<u8>().unwrap(),
                              ip[2].parse::<u8>().unwrap(),ip[3].parse::<u8>().unwrap()),
        HWaddr: split[3].to_string(),
        Device: split[5].to_string()
    };
    nodes.push(node);
}

for node in nodes {
    println!("{:?}", node);

}
*/
